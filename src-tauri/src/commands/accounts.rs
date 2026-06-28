use crate::db::Database;
use crate::models::account::{Account, CreateAccountPayload, UpdateAccountPayload};
use rusqlite::params;
use tauri::State;

fn row_to_account(row: &rusqlite::Row) -> rusqlite::Result<Account> {
    Ok(Account {
        id: row.get("id")?,
        parent_id: row.get("parent_id")?,
        account_type: row.get("account_type")?,
        code: row.get("code")?,
        name: row.get("name")?,
        description: row.get("description")?,
        currency_code: row.get("currency_code")?,
        placeholder: row.get::<_, i64>("placeholder")? != 0,
        is_active: row.get::<_, i64>("is_active")? != 0,
        sort_order: row.get("sort_order")?,
        balance: "0".to_string(),
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

#[tauri::command]
pub fn list_accounts(db: State<Database>) -> Result<Vec<Account>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT a.*, COALESCE((
                SELECT CAST(SUM(
                    CAST(COALESCE(s.debit_amount, '0') AS REAL)
                    - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                ) AS TEXT)
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
            ), '0') AS balance
            FROM accounts a
            ORDER BY a.sort_order, a.name",
        )
        .map_err(|e| e.to_string())?;

    let accounts = stmt
        .query_map([], |row| {
            let mut acc = row_to_account(row)?;
            acc.balance = row.get("balance")?;
            Ok(acc)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(accounts)
}

#[tauri::command]
pub fn create_account(
    db: State<Database>,
    payload: CreateAccountPayload,
) -> Result<Account, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO accounts (parent_id, account_type, code, name, description, currency_code, placeholder, sort_order)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            payload.parent_id,
            payload.account_type,
            payload.code,
            payload.name,
            payload.description,
            payload.currency_code.unwrap_or_else(|| "USD".to_string()),
            payload.placeholder.unwrap_or(false).then(|| 1).unwrap_or(0),
            payload.sort_order.unwrap_or(0),
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    let mut stmt = conn
        .prepare("SELECT * FROM accounts WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        let mut acc = row_to_account(row)?;
        // Compute balance for this single account
        let balance: String = conn
            .query_row(
                "SELECT CAST(COALESCE(SUM(
                    CAST(COALESCE(s.debit_amount, '0') AS REAL)
                    - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                ), 0) AS TEXT) FROM splits s
                JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                WHERE s.account_id = ?1",
                params![id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "0".to_string());
        acc.balance = balance;
        Ok(acc)
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_account(
    db: State<Database>,
    payload: UpdateAccountPayload,
) -> Result<Account, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut sets = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref parent_id) = payload.parent_id {
        sets.push("parent_id = ?");
        param_values.push(Box::new(*parent_id));
    }
    if let Some(ref account_type) = payload.account_type {
        sets.push("account_type = ?");
        param_values.push(Box::new(account_type.clone()));
    }
    if let Some(ref code) = payload.code {
        sets.push("code = ?");
        param_values.push(Box::new(code.clone()));
    }
    if let Some(ref name) = payload.name {
        sets.push("name = ?");
        param_values.push(Box::new(name.clone()));
    }
    if let Some(ref description) = payload.description {
        sets.push("description = ?");
        param_values.push(Box::new(description.clone()));
    }
    if let Some(ref currency_code) = payload.currency_code {
        sets.push("currency_code = ?");
        param_values.push(Box::new(currency_code.clone()));
    }
    if let Some(placeholder) = payload.placeholder {
        sets.push("placeholder = ?");
        param_values.push(Box::new(if placeholder { 1 } else { 0 }));
    }
    if let Some(is_active) = payload.is_active {
        sets.push("is_active = ?");
        param_values.push(Box::new(if is_active { 1 } else { 0 }));
    }
    if let Some(sort_order) = payload.sort_order {
        sets.push("sort_order = ?");
        param_values.push(Box::new(sort_order));
    }

    if sets.is_empty() {
        return Err("No fields to update".to_string());
    }

    sets.push("updated_at = datetime('now')");

    let sql = format!(
        "UPDATE accounts SET {} WHERE id = ?",
        sets.join(", ")
    );

    let mut final_params: Vec<Box<dyn rusqlite::types::ToSql>> = param_values;
    final_params.push(Box::new(payload.id));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = final_params.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, param_refs.as_slice())
        .map_err(|e| e.to_string())?;

    drop(param_refs);
    drop(final_params);

    let mut stmt = conn
        .prepare("SELECT * FROM accounts WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![payload.id], |row| {
        let mut acc = row_to_account(row)?;
        let balance: String = conn
            .query_row(
                "SELECT CAST(COALESCE(SUM(
                    CAST(COALESCE(s.debit_amount, '0') AS REAL)
                    - CAST(COALESCE(s.credit_amount, '0') AS REAL)
                ), 0) AS TEXT) FROM splits s
                JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                WHERE s.account_id = ?1",
                params![payload.id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "0".to_string());
        acc.balance = balance;
        Ok(acc)
    })
    .map_err(|e| e.to_string())
}
