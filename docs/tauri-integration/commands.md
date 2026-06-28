# Tauri Commands

## Accounts

| Command | Payload | Returns | Description |
|---|---|---|---|
| `list_accounts` | — | `Vec<Account>` | Flat list with recursive CTE balance rollup |
| `get_account_tree` | — | `Vec<AccountNode>` | Nested tree built server-side from adjacency list |
| `create_account` | `CreateAccountPayload` | `Account` | Insert new account |
| `update_account` | `UpdateAccountPayload` | `Account` | Partial update |

## Transactions

| Command | Payload | Returns | Description |
|---|---|---|---|
| `post_transaction` | `CreateTransactionPayload` | `Transaction` | Insert transaction + splits; validates balance |
| `list_transactions` | `ListTransactionsQuery` | `PaginatedTransactions` | Paginated, filtered, sorted query |
| `void_transaction` | `id: i64` | — | Soft-delete (sets state = "VOID") |
| `get_dashboard_summary` | — | `serde_json::Value` | Net worth, income, expenses, cash |

## Conventions

- All commands are registered in `lib.rs` via `generate_handler![]`.
- Snake_case JSON keys (Serde convention); the frontend `api.ts` translates camelCase → snake_case.
- Database connection is injected via `tauri::State<Database>`.
- Monetary values are returned as decimal strings — never `f64`.
