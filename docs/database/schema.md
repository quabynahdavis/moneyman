# Database Schema

The full DDL is in [`database-schema.sql`](../../database-schema.sql).

## Entity-Relationship Summary

```
accounts (adjacency list)
  ↳ parent_id → accounts.id
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

## Key Design Points

- **`debit_amount` / `credit_amount`** are stored as TEXT (decimal strings), not REAL.
- **`placeholder`** on accounts prevents posting transactions to parent nodes.
- **`v_account_balances`** uses a recursive CTE to roll up leaf-node balances to every ancestor.
- **`v_transaction_splits`** joins transactions, splits, and accounts for the ledger display.
