# Database Schema

The full DDL is in [`database-schema.sql`](../../database-schema.sql).

## Entity-Relationship Summary

```
accounts (adjacency list)
  ↳ parent_id → accounts.id (ON DELETE RESTRICT)
  ↳ splits (1:N)

transactions
  ↳ splits (1:N)
  ↳ recurring_transactions (1:N)

splits
  ↳ transaction_id → transactions.id
  ↳ account_id → accounts.id
  ↳ reconciliation_entries (1:N)

contacts
  ↳ invoices (1:N)

invoices
  ↳ contact_id → contacts.id
  ↳ invoice_lines (1:N)
  ↳ txn_id → transactions.id (nullable)

budgets
  ↳ budget_amounts (1:N)
    ↳ account_id → accounts.id

price_quotes
  ↳ account_id → accounts.id
```

## Key Design Points (v3)

- **`debit` / `credit`** are stored as INTEGER (cents), not TEXT or REAL — all monetary math uses strict 64-bit integer arithmetic.
- **`reconcile_state`** on splits is constrained to `'n'` (unreconciled), `'c'` (cleared), or `'r'` (reconciled).
- **`post_date`** is the transaction date (ISO 8601 text); **`num`** holds optional check/reference numbers.
- **`is_placeholder`** on accounts prevents posting transactions to parent nodes.
- **account_type** has a CHECK constraint enforcing the fixed set of GnuCash type codes.
- **Recursive CTE balance rollup** — The `list_accounts` and `get_account_tree` commands use a recursive CTE to sum leaf balances (converted from cents to dollars via `/ 100.0`) up the hierarchy.
- **Schema versioning** — The `schema_version` table tracks the current version; migrations in `schema.rs` run sequentially on startup. Post-migration indexes run after all migrations to avoid column-not-found errors on upgrade.
