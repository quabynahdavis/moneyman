use crate::db::Database;
use crate::models::account::{Account, AccountNode, CreateAccountPayload, UpdateAccountPayload};
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
        is_placeholder: row.get::<_, i64>("is_placeholder")? != 0,
        is_active: row.get::<_, i64>("is_active")? != 0,
        sort_order: row.get("sort_order")?,
        balance: "0".to_string(),
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

fn build_tree(accounts: Vec<Account>) -> Vec<AccountNode> {
    let mut map: std::collections::HashMap<i64, AccountNode> = std::collections::HashMap::new();
    let mut roots = Vec::new();

    for a in accounts {
        map.insert(
            a.id,
            AccountNode {
                id: a.id,
                parent_id: a.parent_id,
                account_type: a.account_type,
                code: a.code,
                name: a.name,
                description: a.description,
                currency_code: a.currency_code,
                is_placeholder: a.is_placeholder,
                is_active: a.is_active,
                sort_order: a.sort_order,
                balance: a.balance,
                children: Vec::new(),
            },
        );
    }

    let mut sorted_ids: Vec<i64> = map.keys().copied().collect();
    sorted_ids.sort();

    for id in sorted_ids {
        if let Some(node) = map.remove(&id) {
            let parent_id = node.parent_id;
            if let Some(parent_id) = parent_id {
                if let Some(parent) = map.get_mut(&parent_id) {
                    parent.children.push(node);
                } else {
                    // Parent not in map (shouldn't happen with FK constraint, but handle gracefully)
                    roots.push(node);
                }
            } else {
                roots.push(node);
            }
        }
    }

    roots
}

fn compute_balance_for_account(conn: &rusqlite::Connection, account_id: i64) -> Result<String, String> {
    conn.query_row(
        "SELECT CAST(COALESCE(SUM(
            CAST(COALESCE(s.debit_amount, '0') AS REAL)
            - CAST(COALESCE(s.credit_amount, '0') AS REAL)
        ), 0) AS TEXT) FROM splits s
        JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
        WHERE s.account_id = ?1",
        params![account_id],
        |row| row.get(0),
    )
    .map_err(|e| e.to_string())
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
                WHERE s.account_id = a.id
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
pub fn get_account_tree(db: State<Database>) -> Result<Vec<AccountNode>, String> {
    let accounts = list_accounts_internal(&db)?;
    Ok(build_tree(accounts))
}

fn list_accounts_internal(db: &Database) -> Result<Vec<Account>, String> {
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
                WHERE s.account_id = a.id
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
        "INSERT INTO accounts (parent_id, account_type, code, name, description, currency_code, is_placeholder, sort_order)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            payload.parent_id,
            payload.account_type,
            payload.code,
            payload.name,
            payload.description,
            payload.currency_code.unwrap_or_else(|| "USD".to_string()),
            payload.is_placeholder.unwrap_or(false).then(|| 1).unwrap_or(0),
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
        acc.balance = compute_balance_for_account(&conn, id).unwrap_or_else(|_| "0".to_string());
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
    if let Some(is_placeholder) = payload.is_placeholder {
        sets.push("is_placeholder = ?");
        param_values.push(Box::new(if is_placeholder { 1 } else { 0 }));
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
        acc.balance = compute_balance_for_account(&conn, payload.id).unwrap_or_else(|_| "0".to_string());
        Ok(acc)
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_account(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let child_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM accounts WHERE parent_id = ?1",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if child_count > 0 {
        return Err("Account has sub-accounts. Delete them first.".to_string());
    }

    let split_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM splits WHERE account_id = ?1",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if split_count > 0 {
        return Err("Account has transactions. Void or reassign them first.".to_string());
    }

    conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
