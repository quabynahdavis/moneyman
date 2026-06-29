use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportProfile {
    pub id: Option<i64>,
    pub name: String,
    pub file_format: String,
    pub column_mapping: String,
    pub default_account_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveImportProfilePayload {
    pub name: String,
    pub file_format: String,
    pub column_mapping: String,
    pub default_account_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportPreviewItem {
    pub row_index: u32,
    pub date: String,
    pub description: String,
    pub amount_cents: i64,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
    pub num: Option<String>,
    pub is_duplicate: bool,
    pub matched_transaction_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitImportPayload {
    pub account_id: i64,
    pub items: Vec<ImportPreviewItem>,
}

#[derive(Debug, Serialize)]
pub struct CommitImportResult {
    pub imported: i64,
    pub skipped: i64,
}
