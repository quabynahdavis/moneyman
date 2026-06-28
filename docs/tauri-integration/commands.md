# Tauri Commands

## Current

The Rust backend currently exposes a single greeting command:

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

## Planned Commands

| Command | Purpose |
|---|---|
| `list_accounts` | Fetch full account tree with balances |
| `create_account` | Insert new account |
| `update_account` | Edit account metadata |
| `post_transaction` | Insert transaction + splits |
| `void_transaction` | Soft-delete transaction |
| `list_transactions` | Paginated, filterable query |
| `start_reconciliation` | Begin reconciliation session |
| `complete_reconciliation` | Finalize reconciliation |
| `import_ofx` | Parse and import OFX/QIF/CSV |
| `fetch_quote` | Get latest price from API |
| `get_budget_report` | Aggregated budget vs actuals |
