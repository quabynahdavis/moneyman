use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Split {
    pub id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub account_id: i64,
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
    pub reconcile_state: String,
    pub quantity: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<i64>,
    pub currency_code: String,
    pub description: String,
    pub notes: Option<String>,
    pub num: Option<String>,
    pub post_date: String,
    pub state: String,
    pub splits: Vec<Split>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionPayload {
    pub currency_code: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub num: Option<String>,
    pub post_date: Option<String>,
    pub state: Option<String>,
    pub splits: Vec<CreateSplitPayload>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSplitPayload {
    pub account_id: i64,
    pub debit: i64,
    pub credit: i64,
    pub memo: Option<String>,
    pub quantity: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
    pub filter_text: Option<String>,
    pub filter_account_id: Option<i64>,
    pub filter_state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedTransactions {
    pub transactions: Vec<Transaction>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
