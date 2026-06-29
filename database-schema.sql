-- =============================================================================
-- Moneyman Database Schema — SQLite
-- =============================================================================
-- This DDL defines a complete GnuCash-compatible local-first accounting engine.
-- See ARCHITECTURE.md for data-flow diagrams.
-- =============================================================================

-- ---------------------------------------------------------------------------
-- 1. ENUMS (via lookup tables + CHECK constraints)
-- ---------------------------------------------------------------------------

CREATE TABLE account_types (
    id   TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO account_types (id, name) VALUES
    ('ROOT',        'Root'),
    ('ASSET',       'Asset'),
    ('BANK',        'Bank'),
    ('CASH',        'Cash'),
    ('INVESTMENT',  'Investment'),
    ('STOCK',       'Stock'),
    ('MUTUAL_FUND', 'Mutual Fund'),
    ('RECEIVABLE',  'Accounts Receivable'),
    ('LIABILITY',   'Liability'),
    ('CREDIT_CARD', 'Credit Card'),
    ('PAYABLE',     'Accounts Payable'),
    ('EQUITY',      'Equity'),
    ('INCOME',      'Income'),
    ('EXPENSE',     'Expense');

CREATE TABLE transaction_states (
    id   TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO transaction_states (id, name) VALUES
    ('UNRECONCILED', 'Unreconciled'),
    ('CLEARED',      'Cleared'),
    ('RECONCILED',   'Reconciled'),
    ('VOID',         'Void');

CREATE TABLE recurring_frequencies (
    id   TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO recurring_frequencies (id, name) VALUES
    ('DAILY',     'Daily'),
    ('WEEKLY',    'Weekly'),
    ('BIWEEKLY',  'Bi-Weekly'),
    ('MONTHLY',   'Monthly'),
    ('QUARTERLY', 'Quarterly'),
    ('SEMI_ANNUAL', 'Semi-Annual'),
    ('ANNUAL',    'Annual');

-- ---------------------------------------------------------------------------
-- 2. CURRENCY
-- ---------------------------------------------------------------------------

CREATE TABLE currencies (
    code        TEXT PRIMARY KEY,          -- ISO 4217: GHC, EUR, GBP, JPY...
    name        TEXT NOT NULL,
    symbol      TEXT NOT NULL,
    decimal_places INTEGER NOT NULL DEFAULT 2
);

CREATE TABLE exchange_rates (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    from_currency   TEXT NOT NULL REFERENCES currencies(code),
    to_currency     TEXT NOT NULL REFERENCES currencies(code),
    rate            TEXT NOT NULL,  -- Stored as DECIMAL string
    date            DATE NOT NULL DEFAULT (date('now')),
    source          TEXT DEFAULT 'manual',  -- 'manual', 'api_fetch'
    UNIQUE(from_currency, to_currency, date)
);

-- ---------------------------------------------------------------------------
-- 3. CHART OF ACCOUNTS (Hierarchical Tree via Adjacency List)
-- ---------------------------------------------------------------------------

CREATE TABLE accounts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id       INTEGER REFERENCES accounts(id) ON DELETE RESTRICT,
    account_type    TEXT NOT NULL REFERENCES account_types(id),
    code            TEXT,                   -- Optional account number (e.g. "1000")
    name            TEXT NOT NULL,
    description     TEXT,
    currency_code   TEXT NOT NULL DEFAULT 'GHC' REFERENCES currencies(code),
    placeholder     INTEGER NOT NULL DEFAULT 0,  -- 1 = parent-only (no txns allowed)
    is_active       INTEGER NOT NULL DEFAULT 1,
    sort_order      INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_accounts_parent ON accounts(parent_id);
CREATE INDEX idx_accounts_type ON accounts(account_type);

-- ---------------------------------------------------------------------------
-- 4. TRANSACTIONS & SPLITS (Double-Entry Core)
-- ---------------------------------------------------------------------------

CREATE TABLE transactions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    currency_code   TEXT NOT NULL DEFAULT 'GHC' REFERENCES currencies(code),
    description     TEXT,
    notes           TEXT,
    payee           TEXT,                   -- For checkbook-style register
    number          TEXT,                   -- Check number / reference
    date            DATE NOT NULL DEFAULT (date('now')),
    date_posted     DATE NOT NULL DEFAULT (date('now')),
    state           TEXT NOT NULL DEFAULT 'UNRECONCILED' REFERENCES transaction_states(id),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_transactions_date ON transactions(date);
CREATE INDEX idx_transactions_payee ON transactions(payee);
CREATE INDEX idx_transactions_state ON transactions(state);

-- Splits: the fundamental building blocks of double-entry accounting.
-- Each split belongs to one transaction.
-- debit_amount and credit_amount are stored separately (never both > 0 at once)
-- to preserve the double-entry paper-trail.
CREATE TABLE splits (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id  INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    debit_amount    TEXT NOT NULL DEFAULT '0',   -- DECIMAL string
    credit_amount   TEXT NOT NULL DEFAULT '0',   -- DECIMAL string
    memo            TEXT,
    quantity        TEXT,                        -- For stock accounts: number of shares
    action          TEXT,                        -- 'Buy', 'Sell', 'Dividend', 'Fee'
    reconciled_date DATE,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_splits_txn ON splits(transaction_id);
CREATE INDEX idx_splits_account ON splits(account_id);

-- ---------------------------------------------------------------------------
-- 5. RECURRING TRANSACTIONS
-- ---------------------------------------------------------------------------

CREATE TABLE recurring_transactions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    template_txn_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    frequency       TEXT NOT NULL REFERENCES recurring_frequencies(id),
    interval_count  INTEGER NOT NULL DEFAULT 1,  -- Every N intervals
    next_date       DATE NOT NULL,
    end_date        DATE,
    auto_execute    INTEGER NOT NULL DEFAULT 0,
    last_generated  DATE,
    is_active       INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_recurring_next ON recurring_transactions(next_date);

-- ---------------------------------------------------------------------------
-- 6. RECONCILIATION
-- ---------------------------------------------------------------------------

CREATE TABLE reconciliations (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    statement_date  DATE NOT NULL,
    statement_balance TEXT NOT NULL,           -- DECIMAL string
    closing_balance   TEXT NOT NULL,           -- Calculated ledger balance
    is_complete     INTEGER NOT NULL DEFAULT 0,
    started_at      TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at    TEXT
);

CREATE TABLE reconciliation_entries (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    reconciliation_id INTEGER NOT NULL REFERENCES reconciliations(id) ON DELETE CASCADE,
    split_id        INTEGER NOT NULL REFERENCES splits(id) ON DELETE CASCADE,
    is_cleared      INTEGER NOT NULL DEFAULT 0,
    UNIQUE(reconciliation_id, split_id)
);

-- ---------------------------------------------------------------------------
-- 7. CONTACTS (Clients & Vendors)
-- ---------------------------------------------------------------------------

CREATE TABLE contacts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_type    TEXT NOT NULL CHECK(contact_type IN ('CLIENT', 'VENDOR', 'BOTH')),
    name            TEXT NOT NULL,
    company         TEXT,
    email           TEXT,
    phone           TEXT,
    address_line1   TEXT,
    address_line2   TEXT,
    city            TEXT,
    state           TEXT,
    postal_code     TEXT,
    country         TEXT,
    tax_id          TEXT,
    default_terms   INTEGER DEFAULT 30,       -- Net 30, Net 60, etc.
    currency_code   TEXT DEFAULT 'GHC' REFERENCES currencies(code),
    notes           TEXT,
    is_active       INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_contacts_type ON contacts(contact_type);
CREATE INDEX idx_contacts_name ON contacts(name);

-- ---------------------------------------------------------------------------
-- 8. INVOICES & BILLS
-- ---------------------------------------------------------------------------

CREATE TABLE invoices (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_type    TEXT NOT NULL CHECK(invoice_type IN ('INVOICE', 'BILL')),
    invoice_number  TEXT NOT NULL,
    contact_id      INTEGER NOT NULL REFERENCES contacts(id),
    currency_code   TEXT NOT NULL DEFAULT 'GHC' REFERENCES currencies(code),
    issue_date      DATE NOT NULL DEFAULT (date('now')),
    due_date        DATE NOT NULL,
    payment_terms   INTEGER DEFAULT 30,
    status          TEXT NOT NULL DEFAULT 'DRAFT' CHECK(status IN ('DRAFT', 'SENT', 'PAID', 'OVERDUE', 'CANCELLED')),
    subtotal        TEXT NOT NULL DEFAULT '0',
    tax_total       TEXT NOT NULL DEFAULT '0',
    total           TEXT NOT NULL DEFAULT '0',
    paid_amount     TEXT NOT NULL DEFAULT '0',
    notes           TEXT,
    txn_id          INTEGER REFERENCES transactions(id),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_invoices_contact ON invoices(contact_id);
CREATE INDEX idx_invoices_status ON invoices(status);

CREATE TABLE invoice_lines (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id      INTEGER NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    line_type       TEXT NOT NULL DEFAULT 'ITEM' CHECK(line_type IN ('ITEM', 'SUBTOTAL', 'DISCOUNT')),
    description     TEXT NOT NULL,
    quantity        REAL DEFAULT 1,
    unit_price      TEXT NOT NULL DEFAULT '0',  -- DECIMAL string
    tax_rate        REAL DEFAULT 0,              -- Percentage, e.g. 8.25
    amount          TEXT NOT NULL DEFAULT '0',
    sort_order      INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_invoice_lines_invoice ON invoice_lines(invoice_id);

-- ---------------------------------------------------------------------------
-- 9. BUDGETS
-- ---------------------------------------------------------------------------

CREATE TABLE budgets (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL,
    fiscal_year     INTEGER NOT NULL,
    period_type     TEXT NOT NULL DEFAULT 'MONTHLY' CHECK(period_type IN ('MONTHLY', 'QUARTERLY', 'ANNUAL')),
    is_active       INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE budget_amounts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    budget_id       INTEGER NOT NULL REFERENCES budgets(id) ON DELETE CASCADE,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    period_index    INTEGER NOT NULL,  -- 0-11 for monthly, 0-3 for quarterly
    amount          TEXT NOT NULL DEFAULT '0',  -- DECIMAL string
    UNIQUE(budget_id, account_id, period_index)
);

-- ---------------------------------------------------------------------------
-- 10. ASSET DEPRECIATION
-- ---------------------------------------------------------------------------

CREATE TABLE assets (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id      INTEGER NOT NULL REFERENCES accounts(id),
    name            TEXT NOT NULL,
    purchase_date   DATE NOT NULL,
    purchase_cost   TEXT NOT NULL,         -- DECIMAL
    useful_life_years REAL NOT NULL,
    salvage_value   TEXT NOT NULL DEFAULT '0',
    depreciation_method TEXT NOT NULL DEFAULT 'STRAIGHT_LINE' CHECK(depreciation_method IN ('STRAIGHT_LINE', 'DECLINING_BALANCE')),
    accumulated_depreciation TEXT NOT NULL DEFAULT '0',
    current_value   TEXT NOT NULL,
    is_active       INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ---------------------------------------------------------------------------
-- 11. PRICE QUOTES (for stock/investment accounts)
-- ---------------------------------------------------------------------------

CREATE TABLE price_quotes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    date            DATE NOT NULL DEFAULT (date('now')),
    price           TEXT NOT NULL,         -- DECIMAL: price per share/unit
    source          TEXT DEFAULT 'manual', -- 'manual', 'yahoo', 'alpha_vantage'
    currency_code   TEXT NOT NULL DEFAULT 'GHC',
    UNIQUE(account_id, date)
);

-- =============================================================================
-- VIEWS
-- =============================================================================

-- Account balance rollup: recursive CTE that sums all leaf-node balances up
CREATE VIEW v_account_balances AS
WITH RECURSIVE account_tree AS (
    -- Base: all accounts
    SELECT id, parent_id, account_type, name, currency_code, placeholder
    FROM accounts
    WHERE is_active = 1
),
-- Calculate leaf balances from splits
leaf_balances AS (
    SELECT
        a.id AS account_id,
        CAST(COALESCE(SUM(
            CAST(COALESCE(s.debit_amount, '0') AS REAL)
            - CAST(COALESCE(s.credit_amount, '0') AS REAL)
        ), 0) AS TEXT) AS balance
    FROM accounts a
    LEFT JOIN splits s ON s.account_id = a.id
    LEFT JOIN transactions t ON t.id = s.transaction_id AND t.state != 'VOID'
    GROUP BY a.id
),
-- Roll up children to parents
rollup AS (
    SELECT
        lb.account_id,
        CAST(lb.balance AS REAL) AS total_balance
    FROM leaf_balances lb
    UNION ALL
    SELECT
        a.id,
        CAST(r.total_balance AS REAL)
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
    a.placeholder,
    COALESCE(SUM(r.total_balance), 0) AS balance
FROM accounts a
LEFT JOIN rollup r ON r.account_id = a.id
GROUP BY a.id
ORDER BY a.sort_order, a.name;

-- Transaction detail with splits (for ledger display)
CREATE VIEW v_transaction_splits AS
SELECT
    t.id AS txn_id,
    t.date,
    t.description,
    t.payee,
    t.number,
    t.state,
    t.currency_code,
    s.id AS split_id,
    s.account_id,
    a.name AS account_name,
    a.account_type,
    s.debit_amount,
    s.credit_amount,
    s.memo,
    s.quantity,
    s.action
FROM transactions t
JOIN splits s ON s.transaction_id = t.id
JOIN accounts a ON a.id = s.account_id;

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================

CREATE INDEX idx_splits_debit_credit ON splits(account_id, debit_amount, credit_amount);
CREATE INDEX idx_txn_date_state ON transactions(date, state);
CREATE INDEX idx_invoice_number ON invoices(invoice_number);
