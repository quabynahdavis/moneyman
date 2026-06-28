# IPC Bridge

## Command Pattern

Every database mutation follows this pattern:

1. **Frontend** builds a typed payload
2. **Composable** validates client-side
3. **Pinia store** calls `invoke("command_name", { payload })`
4. **Rust** deserializes via Serde, re-validates
5. **Rust** executes SQL within a transaction
6. **Response** returns updated entities to frontend
7. **Store** replaces local state with response data

## Planned Commands

| Command | Description |
|---|---|
| `create_account` | Insert a new account node |
| `update_account` | Modify account metadata |
| `post_transaction` | Insert a transaction + all splits |
| `void_transaction` | Mark transaction as VOID |
| `reconcile_account` | Start/complete reconciliation |
| `fetch_price_quote` | Get latest price for a security |
| `import_ofx` | Parse and import OFX/QIF/CSV |
