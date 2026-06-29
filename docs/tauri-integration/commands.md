# Tauri Commands

## Accounts

| Command | Payload | Returns | Description |
|---|---|---|---|
| `list_accounts` | — | `Vec<Account>` | Flat list with recursive CTE balance rollup |
| `get_account_tree` | — | `Vec<AccountNode>` | Nested tree built server-side from adjacency list |
| `create_account` | `CreateAccountPayload` | `Account` | Insert new account |
| `update_account` | `UpdateAccountPayload` | `Account` | Partial update |
| `delete_account` | `id: i64` | — | Checks children/splits before deleting |

## Transactions (v3 — integer cents)

| Command | Payload | Returns | Description |
|---|---|---|---|
| `post_transaction` | `CreateTransactionPayload` | `Transaction` | Insert transaction + splits; validates strict integer balance (`SUM(debits) === SUM(credits)`) |
| `list_transactions` | `ListTransactionsQuery` | `PaginatedTransactions` | Paginated, filtered, sorted query (fields: post_date, description, state, num) |
| `void_transaction` | `id: i64` | — | Soft-delete (sets state = "VOID") |
| `get_dashboard_summary` | — | `serde_json::Value` | Net worth, income, expenses, cash — all values in integer cents |

## Conventions

- All commands are registered in `lib.rs` via `generate_handler![]`.
- Snake_case JSON keys (Serde convention); the frontend `api.ts` translates camelCase ↔ snake_case.
- Database connection is injected via `tauri::State<Database>`.
- Monetary values in splits (`debit` / `credit`) are **64-bit integers representing cents** — never `f64` or decimal strings.
- Account `balance` is returned as a decimal string (converted from cents via `/ 100.0` in SQL).
