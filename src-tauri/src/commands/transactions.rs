use crate::db::Database;
use crate::models::transaction::{
    CreateTransactionPayload, ListTransactionsQuery, PaginatedTransactions, Split, Transaction,
};
use rusqlite::params;
use tauri::State;

fn row_to_split(row: &rusqlite::Row) -> rusqlite::Result<Split> {
    Ok(Split {
        id: Some(row.get("id")?),
        transaction_id: Some(row.get("transaction_id")?),
        account_id: row.get("account_id")?,
        account_name: row.get("account_name")?,
        account_type: row.get("account_type")?,
        debit: row.get("debit")?,
        credit: row.get("credit")?,
        memo: row.get("memo")?,
        reconcile_state: row.get("reconcile_state")?,
        quantity: row.get("quantity")?,
        action: row.get("action")?,
    })
}

fn load_splits_for_txn(conn: &rusqlite::Connection, txn_id: i64) -> Result<Vec<Split>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT s.*, a.name AS account_name, a.account_type
             FROM splits s
             LEFT JOIN accounts a ON a.id = s.account_id
             WHERE s.transaction_id = ?1
             ORDER BY s.id",
        )
        .map_err(|e| e.to_string())?;

    let splits = stmt
        .query_map(params![txn_id], row_to_split)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(splits)
}

#[tauri::command]
pub fn post_transaction(
    db: State<Database>,
    payload: CreateTransactionPayload,
) -> Result<Transaction, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate: at least 2 splits
    if payload.splits.len() < 2 {
        return Err("Transaction must have at least 2 splits".to_string());
    }

    // Validate: debits == credits using strict integer math
    let total_debits: i64 = payload.splits.iter().map(|s| s.debit).sum();
    let total_credits: i64 = payload.splits.iter().map(|s| s.credit).sum();

    if total_debits != total_credits {
        return Err(format!(
            "Transaction out of balance: debits={} cents, credits={} cents",
            total_debits, total_credits
        ));
    }

    // Validate: no split has both debit and credit non-zero
    for s in &payload.splits {
        if s.debit > 0 && s.credit > 0 {
            return Err("Split cannot have both debit and credit amounts".to_string());
        }
    }

    // Validate: no splits reference a placeholder account
    for s in &payload.splits {
        let is_placeholder: i64 = conn
            .query_row(
                "SELECT is_placeholder FROM accounts WHERE id = ?1",
                params![s.account_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        if is_placeholder != 0 {
            return Err(format!(
                "Cannot post transaction to placeholder account (ID: {})",
                s.account_id
            ));
        }
    }

    let description = payload
        .description
        .clone()
        .unwrap_or_else(|| "".to_string());
    let post_date = payload
        .post_date
        .clone()
        .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());

    // Insert transaction
    conn.execute(
        "INSERT INTO transactions (currency_code, description, notes, num, post_date, state)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            payload.currency_code.as_deref().unwrap_or("USD"),
            description,
            payload.notes,
            payload.num,
            post_date,
            payload.state.as_deref().unwrap_or("UNRECONCILED"),
        ],
    )
    .map_err(|e| e.to_string())?;

    let txn_id = conn.last_insert_rowid();

    // Insert splits
    for s in &payload.splits {
        conn.execute(
            "INSERT INTO splits (transaction_id, account_id, debit, credit, memo, reconcile_state, quantity, action)
             VALUES (?1, ?2, ?3, ?4, ?5, 'n', ?6, ?7)",
            params![
                txn_id,
                s.account_id,
                s.debit,
                s.credit,
                s.memo,
                s.quantity,
                s.action,
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    let splits = load_splits_for_txn(&conn, txn_id)?;

    let mut stmt = conn
        .prepare("SELECT * FROM transactions WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let txn = stmt
        .query_row(params![txn_id], |row| {
            Ok(Transaction {
                id: Some(row.get("id")?),
                currency_code: row.get("currency_code")?,
                description: row.get("description")?,
                notes: row.get("notes")?,
                num: row.get("num")?,
                post_date: row.get("post_date")?,
                state: row.get("state")?,
                splits,
                created_at: Some(row.get("created_at")?),
                updated_at: Some(row.get("updated_at")?),
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(txn)
}

#[tauri::command]
pub fn list_transactions(
    db: State<Database>,
    query: ListTransactionsQuery,
) -> Result<PaginatedTransactions, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(50).max(1).min(500);
    let offset = (page - 1) * page_size;
    let sort_field = query.sort_field.unwrap_or_else(|| "post_date".to_string());
    let sort_dir = query.sort_direction.unwrap_or_else(|| "desc".to_string());

    let allowed_sort_fields = ["post_date", "description", "state", "num"];
    let sort_col = if allowed_sort_fields.contains(&sort_field.as_str()) {
        &sort_field
    } else {
        "post_date"
    };
    let dir = if sort_dir == "asc" { "ASC" } else { "DESC" };

    let mut where_clauses = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    where_clauses.push("t.state != 'VOID'".to_string());

    if let Some(ref filter_text) = query.filter_text {
        if !filter_text.is_empty() {
            where_clauses.push(
                "(t.description LIKE ? OR t.num LIKE ?)".to_string(),
            );
            let pattern = format!("%{}%", filter_text);
            param_values.push(Box::new(pattern.clone()));
            param_values.push(Box::new(pattern));
        }
    }

    if let Some(account_id) = query.filter_account_id {
        where_clauses.push("EXISTS (SELECT 1 FROM splits s2 WHERE s2.transaction_id = t.id AND s2.account_id = ?)".to_string());
        param_values.push(Box::new(account_id));
    }

    if let Some(ref state) = query.filter_state {
        where_clauses.push("t.state = ?".to_string());
        param_values.push(Box::new(state.clone()));
    }

    let where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // Count total
    let count_sql = format!("SELECT COUNT(*) FROM transactions t {}", where_sql);
    let total: i64 = {
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        conn.query_row(&count_sql, param_refs.as_slice(), |row| row.get(0))
            .map_err(|e| e.to_string())?
    };

    // Fetch transactions
    let query_sql = format!(
        "SELECT t.* FROM transactions t {} ORDER BY t.{} {} LIMIT ? OFFSET ?",
        where_sql, sort_col, dir
    );

    let mut all_params: Vec<Box<dyn rusqlite::types::ToSql>> = param_values;
    all_params.push(Box::new(page_size));
    all_params.push(Box::new(offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        all_params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&query_sql).map_err(|e| e.to_string())?;

    let txn_ids: Vec<i64> = stmt
        .query_map(param_refs.as_slice(), |row| row.get("id"))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    drop(stmt);

    let mut transactions = Vec::new();

    for txn_id in txn_ids {
        let splits = load_splits_for_txn(&conn, txn_id)?;

        let mut stmt = conn
            .prepare("SELECT * FROM transactions WHERE id = ?1")
            .map_err(|e| e.to_string())?;

        let txn = stmt
            .query_row(params![txn_id], |row| {
                Ok(Transaction {
                    id: Some(row.get("id")?),
                    currency_code: row.get("currency_code")?,
                    description: row.get("description")?,
                    notes: row.get("notes")?,
                    num: row.get("num")?,
                    post_date: row.get("post_date")?,
                    state: row.get("state")?,
                    splits,
                    created_at: Some(row.get("created_at")?),
                    updated_at: Some(row.get("updated_at")?),
                })
            })
            .map_err(|e| e.to_string())?;

        transactions.push(txn);
    }

    Ok(PaginatedTransactions {
        transactions,
        total,
        page,
        page_size,
    })
}

#[tauri::command]
pub fn void_transaction(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE transactions SET state = 'VOID', updated_at = datetime('now') WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_dashboard_summary(db: State<Database>) -> Result<serde_json::Value, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Helper: query balance for a set of account types (returns cents as i64)
    fn query_balance(
        conn: &rusqlite::Connection,
        types: &[&str],
        check_sign: Option<&str>,
    ) -> Result<i64, String> {
        let sign_condition = match check_sign {
            Some("positive") => " AND bal.balance > 0",
            Some("negative") => " AND bal.balance < 0",
            _ => "",
        };
        let type_list = types
            .iter()
            .map(|t| format!("'{}'", t))
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "SELECT COALESCE(SUM(bal.balance), 0) FROM (
                SELECT COALESCE(SUM(s.debit - s.credit), 0) AS balance
                FROM splits s
                JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                WHERE s.account_id IN (
                    WITH RECURSIVE desc_ids AS (
                        SELECT id FROM accounts WHERE parent_id = a.id
                        UNION ALL
                        SELECT acc.id FROM accounts acc JOIN desc_ids ON acc.parent_id = desc_ids.id
                    )
                    SELECT id FROM desc_ids
                    UNION SELECT a.id
                )
                FROM accounts a
                WHERE a.account_type IN ({})
                AND a.is_active = 1
            ) bal{}",
            type_list, sign_condition
        );
        conn.query_row(&sql, [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }

    let total_assets = query_balance(
        &conn,
        &["ASSET", "BANK", "CASH", "INVESTMENT", "STOCK", "MUTUAL_FUND", "RECEIVABLE"],
        Some("positive"),
    )?;

    let total_liabilities = query_balance(
        &conn,
        &["LIABILITY", "CREDIT_CARD", "PAYABLE"],
        Some("negative"),
    )?;

    // Income this month: sum credits on income accounts (normal credit balance)
    let total_income: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(s.credit), 0) FROM splits s
             JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
             JOIN accounts a ON a.id = s.account_id
             WHERE a.account_type = 'INCOME'
             AND t.post_date >= date('now', 'start of month')
             AND t.post_date < date('now', 'start of month', '+1 month')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Expenses this month: sum debits on expense accounts
    let total_expenses: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(s.debit), 0) FROM splits s
             JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
             JOIN accounts a ON a.id = s.account_id
             WHERE a.account_type = 'EXPENSE'
             AND t.post_date >= date('now', 'start of month')
             AND t.post_date < date('now', 'start of month', '+1 month')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Cash accounts (IDs 3,4,5) balance
    let total_cash: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(bal.balance), 0) FROM (
                SELECT COALESCE(SUM(s.debit - s.credit), 0) AS balance
                FROM splits s
                JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                WHERE s.account_id = a.id
                FROM accounts a
                WHERE a.id IN (3, 4, 5)
                AND a.is_active = 1
            ) bal",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "total_assets": total_assets,
        "total_liabilities": total_liabilities,
        "total_income": total_income,
        "total_expenses": total_expenses,
        "total_cash": total_cash,
    }))
}
