use crate::db::Database;
use crate::models::recurring::{
    CreateRecurringPayload, RecurringSplit, RecurringTransaction, UpdateRecurringPayload,
};
use rusqlite::params;
use tauri::State;

fn load_splits_for_recurring(
    conn: &rusqlite::Connection,
    recurring_id: i64,
) -> Result<Vec<RecurringSplit>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM recurring_splits WHERE recurring_id = ?1 ORDER BY id")
        .map_err(|e| e.to_string())?;

    let splits = stmt
        .query_map(params![recurring_id], |row| {
            Ok(RecurringSplit {
                id: Some(row.get("id")?),
                recurring_id: Some(row.get("recurring_id")?),
                account_id: row.get("account_id")?,
                debit: row.get("debit")?,
                credit: row.get("credit")?,
                memo: row.get("memo")?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(splits)
}

fn row_to_recurring(
    row: &rusqlite::Row,
    splits: Vec<RecurringSplit>,
) -> rusqlite::Result<RecurringTransaction> {
    Ok(RecurringTransaction {
        id: Some(row.get("id")?),
        frequency: row.get("frequency")?,
        interval_count: row.get("interval_count")?,
        next_date: row.get("next_date")?,
        end_date: row.get("end_date")?,
        auto_execute: row.get("auto_execute")?,
        last_generated: row.get("last_generated")?,
        is_active: row.get("is_active")?,
        description: row.get("description")?,
        currency_code: row.get("currency_code")?,
        notes: row.get("notes")?,
        num: row.get("num")?,
        splits,
        created_at: Some(row.get("created_at")?),
        updated_at: Some(row.get("updated_at")?),
    })
}

#[tauri::command]
pub fn list_recurring_transactions(
    db: State<Database>,
) -> Result<Vec<RecurringTransaction>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT * FROM recurring_transactions ORDER BY next_date")
        .map_err(|e| e.to_string())?;

    let ids: Vec<i64> = stmt
        .query_map([], |row| row.get("id"))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for id in ids {
        let splits = load_splits_for_recurring(&conn, id)?;
        let mut stmt = conn
            .prepare("SELECT * FROM recurring_transactions WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        let txn = stmt
            .query_row(params![id], |row| row_to_recurring(row, splits))
            .map_err(|e| e.to_string())?;
        results.push(txn);
    }

    Ok(results)
}

#[tauri::command]
pub fn create_recurring_transaction(
    db: State<Database>,
    payload: CreateRecurringPayload,
) -> Result<RecurringTransaction, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate at least 2 splits
    if payload.splits.len() < 2 {
        return Err("Recurring transaction must have at least 2 splits".to_string());
    }

    // Validate splits balance
    let total_debits: i64 = payload.splits.iter().map(|s| s.debit).sum();
    let total_credits: i64 = payload.splits.iter().map(|s| s.credit).sum();
    if total_debits != total_credits {
        return Err(format!(
            "Splits out of balance: debits={} cents, credits={} cents",
            total_debits, total_credits
        ));
    }

    conn.execute(
        "INSERT INTO recurring_transactions (frequency, interval_count, next_date, end_date, auto_execute, is_active, description, currency_code, notes, num)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            payload.frequency,
            payload.interval_count.unwrap_or(1),
            payload.next_date,
            payload.end_date,
            payload.auto_execute.unwrap_or(false),
            payload.is_active.unwrap_or(true),
            payload.description,
            payload.currency_code.unwrap_or_else(|| "USD".to_string()),
            payload.notes,
            payload.num,
        ],
    )
    .map_err(|e| e.to_string())?;

    let recurring_id = conn.last_insert_rowid();

    for s in &payload.splits {
        conn.execute(
            "INSERT INTO recurring_splits (recurring_id, account_id, debit, credit, memo)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![recurring_id, s.account_id, s.debit, s.credit, s.memo],
        )
        .map_err(|e| e.to_string())?;
    }

    let splits = load_splits_for_recurring(&conn, recurring_id)?;
    let mut stmt = conn
        .prepare("SELECT * FROM recurring_transactions WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let txn = stmt
        .query_row(params![recurring_id], |row| row_to_recurring(row, splits))
        .map_err(|e| e.to_string())?;

    Ok(txn)
}

#[tauri::command]
pub fn update_recurring_transaction(
    db: State<Database>,
    payload: UpdateRecurringPayload,
) -> Result<RecurringTransaction, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate splits if provided
    if let Some(ref splits) = payload.splits {
        if splits.len() < 2 {
            return Err("Recurring transaction must have at least 2 splits".to_string());
        }
        let total_debits: i64 = splits.iter().map(|s| s.debit).sum();
        let total_credits: i64 = splits.iter().map(|s| s.credit).sum();
        if total_debits != total_credits {
            return Err(format!(
                "Splits out of balance: debits={} cents, credits={} cents",
                total_debits, total_credits
            ));
        }
    }

    conn.execute(
        "UPDATE recurring_transactions SET
            frequency         = COALESCE(?1, frequency),
            interval_count    = COALESCE(?2, interval_count),
            next_date         = COALESCE(?3, next_date),
            end_date          = COALESCE(?4, end_date),
            auto_execute      = COALESCE(?5, auto_execute),
            is_active         = COALESCE(?6, is_active),
            description       = COALESCE(?7, description),
            currency_code     = COALESCE(?8, currency_code),
            notes             = COALESCE(?9, notes),
            num               = COALESCE(?10, num),
            updated_at        = datetime('now')
         WHERE id = ?11",
        params![
            payload.frequency,
            payload.interval_count,
            payload.next_date,
            payload.end_date,
            payload.auto_execute.map(|v| v as i64),
            payload.is_active.map(|v| v as i64),
            payload.description,
            payload.currency_code,
            payload.notes,
            payload.num,
            payload.id,
        ],
    )
    .map_err(|e| e.to_string())?;

    // Replace splits if provided
    if payload.splits.is_some() {
        conn.execute(
            "DELETE FROM recurring_splits WHERE recurring_id = ?1",
            params![payload.id],
        )
        .map_err(|e| e.to_string())?;

        for s in payload.splits.unwrap_or_default() {
            conn.execute(
                "INSERT INTO recurring_splits (recurring_id, account_id, debit, credit, memo)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![payload.id, s.account_id, s.debit, s.credit, s.memo],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    let splits = load_splits_for_recurring(&conn, payload.id)?;
    let mut stmt = conn
        .prepare("SELECT * FROM recurring_transactions WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let txn = stmt
        .query_row(params![payload.id], |row| row_to_recurring(row, splits))
        .map_err(|e| e.to_string())?;

    Ok(txn)
}

#[tauri::command]
pub fn delete_recurring_transaction(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM recurring_transactions WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn compute_next_date(frequency: &str, interval_count: i64, from: &str) -> Result<String, String> {
    let mut date = chrono::NaiveDate::parse_from_str(from, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date: {}", e))?;

    match frequency {
        "DAILY" => date += chrono::Duration::days(interval_count),
        "WEEKLY" => date += chrono::Duration::weeks(interval_count),
        "BIWEEKLY" => date += chrono::Duration::weeks(interval_count * 2),
        "MONTHLY" => {
            let months = interval_count as u32;
            if let Some(d) = date.checked_add_months(chrono::Months::new(months)) {
                date = d;
            } else {
                return Err("Date overflow computing next date".to_string());
            }
        }
        "QUARTERLY" => {
            let months = (interval_count * 3) as u32;
            if let Some(d) = date.checked_add_months(chrono::Months::new(months)) {
                date = d;
            } else {
                return Err("Date overflow computing next date".to_string());
            }
        }
        "SEMI_ANNUAL" => {
            let months = (interval_count * 6) as u32;
            if let Some(d) = date.checked_add_months(chrono::Months::new(months)) {
                date = d;
            } else {
                return Err("Date overflow computing next date".to_string());
            }
        }
        "ANNUAL" => {
            let months = (interval_count * 12) as u32;
            if let Some(d) = date.checked_add_months(chrono::Months::new(months)) {
                date = d;
            } else {
                return Err("Date overflow computing next date".to_string());
            }
        }
        _ => return Err(format!("Unknown frequency: {}", frequency)),
    }

    Ok(date.format("%Y-%m-%d").to_string())
}

#[tauri::command]
pub fn execute_recurring_transaction(
    db: State<Database>,
    id: i64,
) -> Result<RecurringTransaction, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Load the recurring transaction
    let splits = load_splits_for_recurring(&conn, id)?;
    let mut stmt = conn
        .prepare("SELECT * FROM recurring_transactions WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let recurring = stmt
        .query_row(params![id], |row| row_to_recurring(row, splits))
        .map_err(|e| e.to_string())?;

    if !recurring.is_active {
        return Err("Recurring transaction is not active".to_string());
    }

    // Create a real transaction from the template
    let txn_description = if recurring.description.is_empty() {
        "Generated from recurring schedule"
    } else {
        &recurring.description
    };

    conn.execute(
        "INSERT INTO transactions (currency_code, description, notes, num, post_date, state)
         VALUES (?1, ?2, ?3, ?4, ?5, 'UNRECONCILED')",
        params![
            recurring.currency_code,
            txn_description,
            recurring.notes,
            recurring.num,
            recurring.next_date,
        ],
    )
    .map_err(|e| e.to_string())?;

    let txn_id = conn.last_insert_rowid();

    for split in &recurring.splits {
        conn.execute(
            "INSERT INTO splits (transaction_id, account_id, debit, credit, memo, reconcile_state)
             VALUES (?1, ?2, ?3, ?4, ?5, 'n')",
            params![
                txn_id,
                split.account_id,
                split.debit,
                split.credit,
                split.memo,
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    // Compute next date
    let next_date = compute_next_date(
        &recurring.frequency,
        recurring.interval_count,
        &recurring.next_date,
    )?;

    // Check if past end_date
    let should_deactivate = if let Some(ref end_date) = recurring.end_date {
        next_date > *end_date
    } else {
        false
    };

    if should_deactivate {
        conn.execute(
            "UPDATE recurring_transactions SET last_generated = ?1, is_active = 0, updated_at = datetime('now') WHERE id = ?2",
            params![recurring.next_date, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE recurring_transactions SET last_generated = ?1, next_date = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![recurring.next_date, next_date, id],
        )
        .map_err(|e| e.to_string())?;
    }

    let updated_splits = load_splits_for_recurring(&conn, id)?;
    let mut stmt = conn
        .prepare("SELECT * FROM recurring_transactions WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let updated = stmt
        .query_row(params![id], |row| row_to_recurring(row, updated_splits))
        .map_err(|e| e.to_string())?;

    Ok(updated)
}
