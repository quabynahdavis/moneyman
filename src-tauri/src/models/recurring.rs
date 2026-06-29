use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecurringSplit {
    pub id: Option<i64>,
    pub recurring_id: Option<i64>,
    pub account_id: i64,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecurringTransaction {
    pub id: Option<i64>,
    pub frequency: String,
    pub interval_count: i64,
    pub next_date: String,
    pub end_date: Option<String>,
    pub auto_execute: bool,
    pub last_generated: Option<String>,
    pub is_active: bool,
    pub description: String,
    pub currency_code: String,
    pub notes: Option<String>,
    pub num: Option<String>,
    pub splits: Vec<RecurringSplit>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecurringPayload {
    pub frequency: String,
    pub interval_count: Option<i64>,
    pub next_date: String,
    pub end_date: Option<String>,
    pub auto_execute: Option<bool>,
    pub is_active: Option<bool>,
    pub description: String,
    pub currency_code: Option<String>,
    pub notes: Option<String>,
    pub num: Option<String>,
    pub splits: Vec<CreateRecurringSplitPayload>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecurringSplitPayload {
    pub account_id: i64,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecurringPayload {
    pub id: i64,
    pub frequency: Option<String>,
    pub interval_count: Option<i64>,
    pub next_date: Option<String>,
    pub end_date: Option<String>,
    pub auto_execute: Option<bool>,
    pub is_active: Option<bool>,
    pub description: Option<String>,
    pub currency_code: Option<String>,
    pub notes: Option<String>,
    pub num: Option<String>,
    pub splits: Option<Vec<CreateRecurringSplitPayload>>,
}
