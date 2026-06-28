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
        debit_amount: row.get("debit_amount")?,
        credit_amount: row.get("credit_amount")?,
        memo: row.get("memo")?,
        quantity: row.get("quantity")?,
        action: row.get("action")?,
        reconciled_date: row.get("reconciled_date")?,
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

    // Validate: debits == credits
    let total_debits: f64 = payload
        .splits
        .iter()
        .map(|s| s.debit_amount.parse::<f64>().unwrap_or(0.0))
        .sum();
    let total_credits: f64 = payload
        .splits
        .iter()
        .map(|s| s.credit_amount.parse::<f64>().unwrap_or(0.0))
        .sum();

    if (total_debits - total_credits).abs() > 0.0001 {
        return Err(format!(
            "Transaction out of balance: debits={}, credits={}",
            total_debits, total_credits
        ));
    }

    // Insert transaction
    conn.execute(
        "INSERT INTO transactions (currency_code, description, notes, payee, number, date, date_posted, state)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            payload.currency_code.as_deref().unwrap_or("USD"),
            payload.description,
            payload.notes,
            payload.payee,
            payload.number,
            payload.date.as_deref().unwrap_or(""),
            payload.date_posted.as_deref().unwrap_or(""),
            payload.state.as_deref().unwrap_or("UNRECONCILED"),
        ],
    )
    .map_err(|e| e.to_string())?;

    let txn_id = conn.last_insert_rowid();

    // Insert splits
    for s in &payload.splits {
        conn.execute(
            "INSERT INTO splits (transaction_id, account_id, debit_amount, credit_amount, memo, quantity, action)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                txn_id,
                s.account_id,
                s.debit_amount,
                s.credit_amount,
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
                payee: row.get("payee")?,
                number: row.get("number")?,
                date: row.get("date")?,
                date_posted: row.get("date_posted")?,
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
    let sort_field = query.sort_field.unwrap_or_else(|| "date".to_string());
    let sort_dir = query.sort_direction.unwrap_or_else(|| "desc".to_string());

    let allowed_sort_fields = ["date", "payee", "state", "description", "number"];
    let sort_col = if allowed_sort_fields.contains(&sort_field.as_str()) {
        &sort_field
    } else {
        "date"
    };
    let dir = if sort_dir == "asc" { "ASC" } else { "DESC" };

    let mut where_clauses = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    where_clauses.push("t.state != 'VOID'".to_string());

    if let Some(ref filter_text) = query.filter_text {
        if !filter_text.is_empty() {
            where_clauses.push(
                "(t.description LIKE ? OR t.payee LIKE ? OR t.number LIKE ?)".to_string(),
            );
            let pattern = format!("%{}%", filter_text);
            param_values.push(Box::new(pattern.clone()));
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
    drop(param_refs);
    drop(all_params);

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
                    payee: row.get("payee")?,
                    number: row.get("number")?,
                    date: row.get("date")?,
                    date_posted: row.get("date_posted")?,
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

    // Total assets (account types with normal debit balance)
    let total_assets: String = conn
        .query_row(
            "SELECT CAST(COALESCE(SUM(bal.balance), 0) AS TEXT) FROM (
                SELECT COALESCE((
                    SELECT CAST(SUM(
                        CAST(COALESCE(s.debit_amount, '0') AS REAL)
                        - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                    ) AS REAL)
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
                ), 0) AS balance
                FROM accounts a
                WHERE a.account_type IN ('ASSET', 'BANK', 'CASH', 'INVESTMENT', 'STOCK', 'MUTUAL_FUND', 'RECEIVABLE')
                AND a.is_active = 1
            ) bal WHERE bal.balance > 0",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    // Total liabilities
    let total_liabilities: String = conn
        .query_row(
            "SELECT CAST(COALESCE(SUM(bal.balance), 0) AS TEXT) FROM (
                SELECT COALESCE((
                    SELECT CAST(SUM(
                        CAST(COALESCE(s.debit_amount, '0') AS REAL)
                        - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                    ) AS REAL)
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
                ), 0) AS balance
                FROM accounts a
                WHERE a.account_type IN ('LIABILITY', 'CREDIT_CARD', 'PAYABLE')
                AND a.is_active = 1
            ) bal WHERE bal.balance < 0",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    // Total income this month
    let total_income: String = conn
        .query_row(
            "SELECT CAST(COALESCE(SUM(
                CAST(COALESCE(s.credit_amount, '0') AS REAL)
            ), 0) AS TEXT) FROM splits s
            JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
            JOIN accounts a ON a.id = s.account_id
            WHERE a.account_type = 'INCOME'
            AND t.date >= date('now', 'start of month')
            AND t.date < date('now', 'start of month', '+1 month')",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    // Total expenses this month
    let total_expenses: String = conn
        .query_row(
            "SELECT CAST(COALESCE(SUM(
                CAST(COALESCE(s.debit_amount, '0') AS REAL)
            ), 0) AS TEXT) FROM splits s
            JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
            JOIN accounts a ON a.id = s.account_id
            WHERE a.account_type = 'EXPENSE'
            AND t.date >= date('now', 'start of month')
            AND t.date < date('now', 'start of month', '+1 month')",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    // Cash accounts sum
    let total_cash: String = conn
        .query_row(
            "SELECT CAST(COALESCE(SUM(bal.balance), 0) AS TEXT) FROM (
                SELECT COALESCE((
                    SELECT CAST(SUM(
                        CAST(COALESCE(s.debit_amount, '0') AS REAL)
                        - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                    ) AS REAL)
                    FROM splits s
                    JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                    WHERE s.account_id = a.id
                ), 0) AS balance
                FROM accounts a
                WHERE a.id IN (3, 4, 5)
                AND a.is_active = 1
            ) bal",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    let net_worth_val: f64 =
        total_assets.parse::<f64>().unwrap_or(0.0) - total_liabilities.parse::<f64>().unwrap_or(0.0);

    Ok(serde_json::json!({
        "net_worth": net_worth_val.to_string(),
        "total_assets": total_assets,
        "total_liabilities": total_liabilities,
        "total_income": total_income,
        "total_expenses": total_expenses,
        "total_cash": total_cash,
    }))
}
