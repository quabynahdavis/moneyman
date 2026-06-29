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
    // Recurring frequencies
    "CREATE TABLE IF NOT EXISTS recurring_frequencies (
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
    // Transactions (v3: integer cents for splits)
    "CREATE TABLE IF NOT EXISTS transactions (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        currency_code   TEXT NOT NULL DEFAULT 'USD' REFERENCES currencies(code),
        description     TEXT NOT NULL,
        notes           TEXT,
        num             TEXT,
        post_date       TEXT NOT NULL DEFAULT (date('now')),
        state           TEXT NOT NULL DEFAULT 'UNRECONCILED' REFERENCES transaction_states(id),
        created_at      TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
    )",
    // Splits (v3: debit/credit as INTEGER cents)
    "CREATE TABLE IF NOT EXISTS splits (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        transaction_id  INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
        account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
        memo            TEXT,
        debit           INTEGER NOT NULL DEFAULT 0,
        credit          INTEGER NOT NULL DEFAULT 0,
        reconcile_state TEXT NOT NULL DEFAULT 'n' CHECK(reconcile_state IN ('n', 'c', 'r')),
        quantity        TEXT,
        action          TEXT,
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
    // Recurring frequencies
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('DAILY', 'Daily')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('WEEKLY', 'Weekly')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('BIWEEKLY', 'Bi-Weekly')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('MONTHLY', 'Monthly')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('QUARTERLY', 'Quarterly')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('SEMI_ANNUAL', 'Semi-Annual')",
    "INSERT OR IGNORE INTO recurring_frequencies (id, name) VALUES ('ANNUAL', 'Annual')",
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

// Indexes that reference columns added in v3 (post_date).
// Must run AFTER both CREATE_TABLES and MIGRATIONS to avoid
// failing on old table schemas during upgrade.
pub const POST_MIGRATION_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_transactions_post_date ON transactions(post_date)",
    "CREATE INDEX IF NOT EXISTS idx_txn_date_state ON transactions(post_date, state)",
];

pub const SCHEMA_VERSION: i64 = 3;

// Each entry runs only when the stored version is less than the key.
// Keys must be sequential, starting from the first version to migrate.
pub const MIGRATIONS: &[(i64, &[&str])] = &[
    (
        2,
        &[
            "ALTER TABLE accounts RENAME COLUMN placeholder TO is_placeholder",
        ],
    ),
    (
        3,
        &[
            // Drop views that depend on old splits/transactions
            "DROP VIEW IF EXISTS v_account_balances",
            "DROP VIEW IF EXISTS v_transaction_splits",
            // Drop tables that reference old transactions
            "DROP TABLE IF EXISTS reconciliation_entries",
            "DROP TABLE IF EXISTS reconciliations",
            "DROP TABLE IF EXISTS recurring_transactions",
            "DROP TABLE IF EXISTS invoice_lines",
            "DROP TABLE IF EXISTS invoices",
            "DROP TABLE IF EXISTS budget_amounts",
            "DROP TABLE IF EXISTS budgets",
            "DROP TABLE IF EXISTS assets",
            "DROP TABLE IF EXISTS price_quotes",
            // Drop old splits/transactions
            "DROP TABLE IF EXISTS splits",
            "DROP TABLE IF EXISTS transactions",
            // Recreate transactions (v3)
            "CREATE TABLE transactions (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                currency_code   TEXT NOT NULL DEFAULT 'USD' REFERENCES currencies(code),
                description     TEXT NOT NULL,
                notes           TEXT,
                num             TEXT,
                post_date       TEXT NOT NULL DEFAULT (date('now')),
                state           TEXT NOT NULL DEFAULT 'UNRECONCILED' REFERENCES transaction_states(id),
                created_at      TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            "CREATE INDEX IF NOT EXISTS idx_transactions_post_date ON transactions(post_date)",
            "CREATE INDEX IF NOT EXISTS idx_transactions_state ON transactions(state)",
            "CREATE INDEX IF NOT EXISTS idx_txn_date_state ON transactions(post_date, state)",
            // Recreate splits (v3: INTEGER cents)
            "CREATE TABLE splits (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                transaction_id  INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
                account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
                memo            TEXT,
                debit           INTEGER NOT NULL DEFAULT 0,
                credit          INTEGER NOT NULL DEFAULT 0,
                reconcile_state TEXT NOT NULL DEFAULT 'n' CHECK(reconcile_state IN ('n', 'c', 'r')),
                quantity        TEXT,
                action          TEXT,
                created_at      TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            "CREATE INDEX IF NOT EXISTS idx_splits_txn ON splits(transaction_id)",
            "CREATE INDEX IF NOT EXISTS idx_splits_account ON splits(account_id)",
            "CREATE INDEX IF NOT EXISTS idx_splits_debit_credit ON splits(account_id, debit, credit)",
            // Views
            "CREATE VIEW v_account_balances AS
            WITH RECURSIVE account_tree AS (
                SELECT id, parent_id, account_type, name, currency_code, is_placeholder
                FROM accounts
                WHERE is_active = 1
            ),
            leaf_balances AS (
                SELECT
                    a.id AS account_id,
                    CAST(CAST(COALESCE(SUM(s.debit - s.credit), 0) AS REAL) / 100.0 AS TEXT) AS balance
                FROM accounts a
                LEFT JOIN splits s ON s.account_id = a.id
                LEFT JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
                GROUP BY a.id
            ),
            rollup AS (
                SELECT lb.account_id, CAST(lb.balance AS REAL) AS total_balance
                FROM leaf_balances lb
                UNION ALL
                SELECT a.id, CAST(r.total_balance AS REAL)
                FROM accounts a
                JOIN account_tree at ON at.id = a.id
                JOIN rollup r ON r.account_id = a.id
                WHERE a.parent_id IS NOT NULL
            )
            SELECT
                a.id,
                a.parent_id,
                a.account_type,
                a.name,
                a.currency_code,
                a.is_placeholder,
                COALESCE(SUM(r.total_balance), 0) AS balance
            FROM accounts a
            LEFT JOIN rollup r ON r.account_id = a.id
            GROUP BY a.id
            ORDER BY a.sort_order, a.name",
            "CREATE VIEW v_transaction_splits AS
            SELECT
                t.id AS txn_id,
                t.post_date,
                t.description,
                t.num,
                t.state,
                t.currency_code,
                s.id AS split_id,
                s.account_id,
                a.name AS account_name,
                a.account_type,
                s.debit,
                s.credit,
                s.memo,
                s.quantity,
                s.action,
                s.reconcile_state
            FROM transactions t
            JOIN splits s ON s.transaction_id = t.id
            JOIN accounts a ON a.id = s.account_id",
        ],
    ),
];
