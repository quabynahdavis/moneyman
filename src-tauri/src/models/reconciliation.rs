use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReconciliationSession {
    pub id: Option<i64>,
    pub account_id: i64,
    pub statement_date: String,
    pub ending_balance: i64,
    pub starting_balance: i64,
    pub state: String,
    pub created_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReconcileSplit {
    pub id: i64,
    pub transaction_id: i64,
    pub account_id: i64,
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub post_date: String,
    pub description: String,
    pub num: Option<String>,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
    pub reconcile_state: String,
}

#[derive(Debug, Serialize)]
pub struct ReconciliationData {
    pub session: ReconciliationSession,
    pub splits: Vec<ReconcileSplit>,
    pub cleared_total: i64,
    pub difference: i64,
}

#[derive(Debug, Deserialize)]
pub struct StartReconciliationPayload {
    pub account_id: i64,
    pub statement_date: String,
    pub ending_balance: i64,
    pub starting_balance: i64,
}
