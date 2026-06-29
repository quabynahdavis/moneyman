use crate::db::Database;
use crate::models::import_profile::{
    CommitImportPayload, CommitImportResult, ImportPreviewItem, ImportProfile,
    SaveImportProfilePayload,
};
use rusqlite::params;
use tauri::State;
use std::collections::HashMap;

#[tauri::command]
pub fn list_import_profiles(db: State<Database>) -> Result<Vec<ImportProfile>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT * FROM import_profiles ORDER BY name")
        .map_err(|e| e.to_string())?;

    let profiles = stmt
        .query_map([], |row| {
            Ok(ImportProfile {
                id: Some(row.get("id")?),
                name: row.get("name")?,
                file_format: row.get("file_format")?,
                column_mapping: row.get("column_mapping")?,
                default_account_id: row.get("default_account_id")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(profiles)
}

#[tauri::command]
pub fn save_import_profile(
    db: State<Database>,
    payload: SaveImportProfilePayload,
) -> Result<ImportProfile, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO import_profiles (name, file_format, column_mapping, default_account_id)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            payload.name,
            payload.file_format,
            payload.column_mapping,
            payload.default_account_id,
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    Ok(ImportProfile {
        id: Some(id),
        name: payload.name,
        file_format: payload.file_format,
        column_mapping: payload.column_mapping,
        default_account_id: payload.default_account_id,
        created_at: None,
        updated_at: None,
    })
}

#[tauri::command]
pub fn delete_import_profile(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM import_profiles WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Parse raw CSV content into preview items.
/// `header_map` is a JSON string mapping internal field names to CSV column indices, e.g. {"date":0,"description":1,"amount":2}
/// `default_debit_account_id` / `default_credit_account_id` allow assigning a fallback account.
#[tauri::command]
pub fn preview_csv_import(
    _db: State<Database>,
    raw_content: String,
    header_map: String,
) -> Result<Vec<ImportPreviewItem>, String> {
    let mapping: HashMap<String, usize> =
        serde_json::from_str(&header_map).map_err(|e| format!("Invalid header map: {}", e))?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(raw_content.as_bytes());

    let mut items = Vec::new();
    let mut row_index: u32 = 0;

    for result in reader.records() {
        let record = result.map_err(|e| format!("CSV parse error at row {}: {}", row_index, e))?;
        row_index += 1;

        let get_field = |key: &str| -> Option<String> {
            mapping.get(key).and_then(|&idx| record.get(idx)).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
        };

        let date = get_field("date").unwrap_or_default();
        let description = get_field("description").unwrap_or_default();
        let amount_str = get_field("amount").unwrap_or_default();
        let memo = get_field("memo");
        let num = get_field("num");

        if date.is_empty() || description.is_empty() || amount_str.is_empty() {
            continue;
        }

        let amount_cents = parse_amount_to_cents(&amount_str);
        let (debit, credit) = if amount_cents >= 0 {
            (amount_cents, 0)
        } else {
            (0, -amount_cents)
        };

        items.push(ImportPreviewItem {
            row_index,
            date,
            description,
            amount_cents,
            debit,
            credit,
            memo,
            num,
            is_duplicate: false, // checked at commit time
            matched_transaction_id: None,
        });
    }

    Ok(items)
}

/// Check for duplicates and commit imported items as transactions.
#[tauri::command]
pub fn commit_import(
    db: State<Database>,
    payload: CommitImportPayload,
) -> Result<CommitImportResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut imported: i64 = 0;
    let mut skipped: i64 = 0;

    for item in &payload.items {
        // Check for duplicate: same date + amount + description
        let description_pattern = format!("%{}%", item.description);
        let existing: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM splits s
                 JOIN transactions t ON t.id = s.transaction_id
                 WHERE s.account_id = ?1
                   AND t.post_date = ?2
                   AND t.description LIKE ?3
                   AND (s.debit = ?4 OR s.credit = ?5)",
                params![
                    payload.account_id,
                    item.date,
                    description_pattern,
                    item.debit,
                    item.credit,
                ],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        if existing > 0 {
            skipped += 1;
            continue;
        }

        // Create transaction
        let desc = if item.description.is_empty() {
            "Imported transaction"
        } else {
            &item.description
        };

        conn.execute(
            "INSERT INTO transactions (currency_code, description, notes, num, post_date, state)
             VALUES ('USD', ?1, ?2, ?3, ?4, 'UNRECONCILED')",
            params![desc, item.memo, item.num, item.date],
        )
        .map_err(|e| e.to_string())?;

        let txn_id = conn.last_insert_rowid();

        // Create the split for this account
        conn.execute(
            "INSERT INTO splits (transaction_id, account_id, debit, credit, memo, reconcile_state)
             VALUES (?1, ?2, ?3, ?4, ?5, 'n')",
            params![txn_id, payload.account_id, item.debit, item.credit, item.memo],
        )
        .map_err(|e| e.to_string())?;

        imported += 1;
    }

    Ok(CommitImportResult { imported, skipped })
}

fn parse_amount_to_cents(amount: &str) -> i64 {
    let cleaned: String = amount
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+')
        .collect();

    // Handle parenthetical negatives: (123.45) -> -123.45
    let is_negative = cleaned.starts_with('-') || (cleaned.starts_with('(') && cleaned.ends_with(')'));
    let numeric: String = cleaned
        .trim_start_matches(|c: char| c == '-' || c == '+' || c == '(')
        .trim_end_matches(')')
        .chars()
        .collect();

    if let Ok(val) = numeric.parse::<f64>() {
        let cents = (val * 100.0).round() as i64;
        if is_negative {
            -cents
        } else {
            cents
        }
    } else {
        0
    }
}
