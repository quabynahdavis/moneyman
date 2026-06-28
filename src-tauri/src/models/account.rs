use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub account_type: String,
    pub code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub currency_code: String,
    pub is_placeholder: bool,
    pub is_active: bool,
    pub sort_order: i64,
    pub balance: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub account_type: String,
    pub code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub currency_code: String,
    pub is_placeholder: bool,
    pub is_active: bool,
    pub sort_order: i64,
    pub balance: String,
    pub children: Vec<AccountNode>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountPayload {
    pub parent_id: Option<i64>,
    pub account_type: String,
    pub code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub currency_code: Option<String>,
    pub is_placeholder: Option<bool>,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountPayload {
    pub id: i64,
    pub parent_id: Option<Option<i64>>,
    pub account_type: Option<String>,
    pub code: Option<Option<String>>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub currency_code: Option<String>,
    pub is_placeholder: Option<bool>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i64>,
}
