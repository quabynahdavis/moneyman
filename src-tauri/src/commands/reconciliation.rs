use crate::db::Database;
use crate::models::reconciliation::{
    ReconcileSplit, ReconciliationData, ReconciliationSession, StartReconciliationPayload,
};
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn start_reconciliation(
    db: State<Database>,
    payload: StartReconciliationPayload,
) -> Result<ReconciliationData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check no open session exists for this account
    let open_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM reconciliation_sessions WHERE account_id = ?1 AND state = 'open'",
            params![payload.account_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    if open_count > 0 {
        return Err("An open reconciliation session already exists for this account".to_string());
    }

    conn.execute(
        "INSERT INTO reconciliation_sessions (account_id, statement_date, ending_balance, starting_balance, state)
         VALUES (?1, ?2, ?3, ?4, 'open')",
        params![
            payload.account_id,
            payload.statement_date,
            payload.ending_balance,
            payload.starting_balance,
        ],
    )
    .map_err(|e| e.to_string())?;

    let session_id = conn.last_insert_rowid();

    let session = ReconciliationSession {
        id: Some(session_id),
        account_id: payload.account_id,
        statement_date: payload.statement_date,
        ending_balance: payload.ending_balance,
        starting_balance: payload.starting_balance,
        state: "open".to_string(),
        created_at: None,
        completed_at: None,
    };

    let splits = fetch_reconciliation_splits(&conn, payload.account_id)?;
    let cleared_total = compute_cleared_total(&splits);
    let difference = cleared_total - payload.ending_balance;

    Ok(ReconciliationData {
        session,
        splits,
        cleared_total,
        difference,
    })
}

#[tauri::command]
pub fn get_reconciliation_data(
    db: State<Database>,
    session_id: i64,
) -> Result<ReconciliationData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let session = conn
        .query_row(
            "SELECT * FROM reconciliation_sessions WHERE id = ?1",
            params![session_id],
            |row| {
                Ok(ReconciliationSession {
                    id: Some(row.get("id")?),
                    account_id: row.get("account_id")?,
                    statement_date: row.get("statement_date")?,
                    ending_balance: row.get("ending_balance")?,
                    starting_balance: row.get("starting_balance")?,
                    state: row.get("state")?,
                    created_at: row.get("created_at")?,
                    completed_at: row.get("completed_at")?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    let splits = fetch_reconciliation_splits(&conn, session.account_id)?;
    let cleared_total = compute_cleared_total(&splits);
    let difference = cleared_total - session.ending_balance;

    Ok(ReconciliationData {
        session,
        splits,
        cleared_total,
        difference,
    })
}

#[tauri::command]
pub fn toggle_split_reconcile_state(
    db: State<Database>,
    split_id: i64,
) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let current_state: String = conn
        .query_row(
            "SELECT reconcile_state FROM splits WHERE id = ?1",
            params![split_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Split not found: {}", e))?;

    let new_state = match current_state.as_str() {
        "n" => "c",
        "c" => "n",
        "r" => return Err("Cannot toggle a reconciled split".to_string()),
        _ => return Err(format!("Unknown reconcile state: {}", current_state)),
    };

    conn.execute(
        "UPDATE splits SET reconcile_state = ?1 WHERE id = ?2",
        params![new_state, split_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(new_state.to_string())
}

#[tauri::command]
pub fn finalize_reconciliation(
    db: State<Database>,
    session_id: i64,
) -> Result<ReconciliationData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let session: ReconciliationSession = conn
        .query_row(
            "SELECT * FROM reconciliation_sessions WHERE id = ?1 AND state = 'open'",
            params![session_id],
            |row| {
                Ok(ReconciliationSession {
                    id: Some(row.get("id")?),
                    account_id: row.get("account_id")?,
                    statement_date: row.get("statement_date")?,
                    ending_balance: row.get("ending_balance")?,
                    starting_balance: row.get("starting_balance")?,
                    state: row.get("state")?,
                    created_at: row.get("created_at")?,
                    completed_at: row.get("completed_at")?,
                })
            },
        )
        .map_err(|_| "Reconciliation session not found or already completed".to_string())?;

    let splits = fetch_reconciliation_splits(&conn, session.account_id)?;
    let cleared_total = compute_cleared_total(&splits);
    let difference = cleared_total - session.ending_balance;

    if difference != 0 {
        return Err(format!(
            "Cannot finalize: difference is {} cents. Must be zero.",
            difference
        ));
    }

    // Mark all 'c' (cleared) splits as 'r' (reconciled)
    conn.execute(
        "UPDATE splits SET reconcile_state = 'r' WHERE account_id = ?1 AND reconcile_state = 'c'",
        params![session.account_id],
    )
    .map_err(|e| e.to_string())?;

    // Mark session completed
    conn.execute(
        "UPDATE reconciliation_sessions SET state = 'completed', completed_at = datetime('now') WHERE id = ?1",
        params![session_id],
    )
    .map_err(|e| e.to_string())?;

    let updated_splits = fetch_reconciliation_splits(&conn, session.account_id)?;
    let updated_cleared = compute_cleared_total(&updated_splits);

    Ok(ReconciliationData {
        session: ReconciliationSession {
            state: "completed".to_string(),
            completed_at: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            ..session
        },
        splits: updated_splits,
        cleared_total: updated_cleared,
        difference: updated_cleared - session.ending_balance,
    })
}

fn fetch_reconciliation_splits(
    conn: &rusqlite::Connection,
    account_id: i64,
) -> Result<Vec<ReconcileSplit>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.transaction_id, s.account_id, a.name AS account_name, a.account_type,
                    t.post_date, t.description, t.num, s.debit, s.credit, s.memo, s.reconcile_state
             FROM splits s
             JOIN transactions t ON t.id = s.transaction_id
             LEFT JOIN accounts a ON a.id = s.account_id
             WHERE s.account_id = ?1 AND t.state != 'VOID'
             ORDER BY t.post_date DESC, t.id DESC",
        )
        .map_err(|e| e.to_string())?;

    let splits = stmt
        .query_map(params![account_id], |row| {
            Ok(ReconcileSplit {
                id: row.get("id")?,
                transaction_id: row.get("transaction_id")?,
                account_id: row.get("account_id")?,
                account_name: row.get("account_name")?,
                account_type: row.get("account_type")?,
                post_date: row.get("post_date")?,
                description: row.get("description")?,
                num: row.get("num")?,
                debit: row.get("debit")?,
                credit: row.get("credit")?,
                memo: row.get("memo")?,
                reconcile_state: row.get("reconcile_state")?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(splits)
}

fn compute_cleared_total(splits: &[ReconcileSplit]) -> i64 {
    splits
        .iter()
        .filter(|s| s.reconcile_state == "c")
        .map(|s| s.debit - s.credit)
        .sum()
}

/// Guard: prevent modifying reconciled splits via existing commands
#[tauri::command]
pub fn check_reconciled_split(db: State<Database>, split_id: i64) -> Result<bool, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let state: String = conn
        .query_row(
            "SELECT reconcile_state FROM splits WHERE id = ?1",
            params![split_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Split not found: {}", e))?;
    Ok(state == "r")
}
