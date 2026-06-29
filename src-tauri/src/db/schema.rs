pub const CREATE_TABLES: &[&str] = &[
    // Account types lookup
    "CREATE TABLE IF NOT EXISTS account_types (
        id   TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    )",
    // Currency table
    "CREATE TABLE IF NOT EXISTS currencies (
        code           TEXT PRIMARY KEY,
        name           TEXT NOT NULL,
        symbol         TEXT NOT NULL,
        decimal_places INTEGER NOT NULL DEFAULT 2
    )",
    // Transaction states
    "CREATE TABLE IF NOT EXISTS transaction_states (
        id   TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    )",
    // Chart of Accounts
    "CREATE TABLE IF NOT EXISTS accounts (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        parent_id       INTEGER REFERENCES accounts(id) ON DELETE RESTRICT,
        account_type    TEXT NOT NULL CHECK (account_type IN ('ROOT','ASSET','BANK','CASH','INVESTMENT','STOCK','MUTUAL_FUND','RECEIVABLE','LIABILITY','CREDIT_CARD','PAYABLE','EQUITY','INCOME','EXPENSE')),
        code            TEXT,
        name            TEXT NOT NULL,
        description     TEXT,
        currency_code   TEXT NOT NULL DEFAULT 'USD' REFERENCES currencies(code),
        is_placeholder  INTEGER NOT NULL DEFAULT 0,
        is_active       INTEGER NOT NULL DEFAULT 1,
        sort_order      INTEGER NOT NULL DEFAULT 0,
        created_at      TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
    )",
    "CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id)",
    "CREATE INDEX IF NOT EXISTS idx_accounts_type ON accounts(account_type)",
    // Transactions
    "CREATE TABLE IF NOT EXISTS transactions (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        currency_code   TEXT NOT NULL DEFAULT 'USD' REFERENCES currencies(code),
        description     TEXT,
        notes           TEXT,
        payee           TEXT,
        number          TEXT,
        date            TEXT NOT NULL DEFAULT (date('now')),
        date_posted     TEXT NOT NULL DEFAULT (date('now')),
        state           TEXT NOT NULL DEFAULT 'UNRECONCILED' REFERENCES transaction_states(id),
        created_at      TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
    )",
    "CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date)",
    "CREATE INDEX IF NOT EXISTS idx_transactions_payee ON transactions(payee)",
    "CREATE INDEX IF NOT EXISTS idx_transactions_state ON transactions(state)",
    // Splits
    "CREATE TABLE IF NOT EXISTS splits (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        transaction_id  INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
        account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
        debit_amount    TEXT NOT NULL DEFAULT '0',
        credit_amount   TEXT NOT NULL DEFAULT '0',
        memo            TEXT,
        quantity        TEXT,
        action          TEXT,
        reconciled_date TEXT,
        created_at      TEXT NOT NULL DEFAULT (datetime('now'))
    )",
    "CREATE INDEX IF NOT EXISTS idx_splits_txn ON splits(transaction_id)",
    "CREATE INDEX IF NOT EXISTS idx_splits_account ON splits(account_id)",
    // Schema version tracking
    "CREATE TABLE IF NOT EXISTS schema_version (
        version    INTEGER PRIMARY KEY,
        applied_at TEXT NOT NULL DEFAULT (datetime('now'))
    )",
];

pub const INSERT_SEED_DATA: &[&str] = &[
    // Account types
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('ROOT', 'Root')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('ASSET', 'Asset')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('BANK', 'Bank')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('CASH', 'Cash')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('INVESTMENT', 'Investment')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('STOCK', 'Stock')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('MUTUAL_FUND', 'Mutual Fund')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('RECEIVABLE', 'Accounts Receivable')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('LIABILITY', 'Liability')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('CREDIT_CARD', 'Credit Card')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('PAYABLE', 'Accounts Payable')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('EQUITY', 'Equity')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('INCOME', 'Income')",
    "INSERT OR IGNORE INTO account_types (id, name) VALUES ('EXPENSE', 'Expense')",
    // Transaction states
    "INSERT OR IGNORE INTO transaction_states (id, name) VALUES ('UNRECONCILED', 'Unreconciled')",
    "INSERT OR IGNORE INTO transaction_states (id, name) VALUES ('CLEARED', 'Cleared')",
    "INSERT OR IGNORE INTO transaction_states (id, name) VALUES ('RECONCILED', 'Reconciled')",
    "INSERT OR IGNORE INTO transaction_states (id, name) VALUES ('VOID', 'Void')",
    // Currencies
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('GHC', 'Ghana Cedi', '₵', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('USD', 'US Dollar', '$', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('EUR', 'Euro', '€', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('GBP', 'British Pound', '£', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('JPY', 'Japanese Yen', '¥', 0)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('CHF', 'Swiss Franc', 'Fr', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('CAD', 'Canadian Dollar', 'C$', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('AUD', 'Australian Dollar', 'A$', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('INR', 'Indian Rupee', '₹', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('BRL', 'Brazilian Real', 'R$', 2)",
    "INSERT OR IGNORE INTO currencies (code, name, symbol, decimal_places) VALUES ('CNY', 'Chinese Yuan', '¥', 2)",
    // Seed default accounts
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (1, NULL, 'ROOT', NULL, 'Root Account', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (2, 1, 'ASSET', '1000', 'Assets', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (3, 2, 'BANK', '1100', 'Checking Account', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (4, 2, 'BANK', '1200', 'Savings Account', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (5, 2, 'CASH', '1300', 'Cash on Hand', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (6, 1, 'LIABILITY', '2000', 'Liabilities', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (7, 6, 'CREDIT_CARD', '2100', 'Credit Card', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (8, 1, 'EQUITY', '3000', 'Equity', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (9, 8, 'EQUITY', '3100', 'Opening Balances', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (10, 1, 'INCOME', '4000', 'Income', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (11, 10, 'INCOME', '4100', 'Employment Income', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (12, 1, 'EXPENSE', '5000', 'Expenses', 'USD', 1)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (13, 12, 'EXPENSE', '5100', 'Housing', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (14, 12, 'EXPENSE', '5200', 'Food & Dining', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (15, 12, 'EXPENSE', '5300', 'Transportation', 'USD', 0)",
    "INSERT OR IGNORE INTO accounts (id, parent_id, account_type, code, name, currency_code, is_placeholder) VALUES (16, 12, 'EXPENSE', '5400', 'Utilities', 'USD', 0)",
];

pub const SCHEMA_VERSION: i64 = 2;

// Each entry runs only when the stored version is less than the key.
// Keys must be sequential, starting from the first version to migrate.
pub const MIGRATIONS: &[(i64, &[&str])] = &[
    (
        2,
        &[
            "ALTER TABLE accounts RENAME COLUMN placeholder TO is_placeholder",
        ],
    ),
];
