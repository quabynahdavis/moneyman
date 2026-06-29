# Migrations

Moneyman uses schema versioning stored in the `schema_version` table:

```sql
CREATE TABLE IF NOT EXISTS schema_version (
    version    INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

Migrations are defined as an array of `(version, [sql_stmts])` tuples in `schema.rs`. On startup, `Database::initialize()` checks the current version and runs each pending migration sequentially, recording each version after completion.

## Migration History

| Version | Description |
|---|---|
| 1 | Initial schema (account_types lookup table, `placeholder` column) |
| 2 | Rename `placeholder` → `is_placeholder`, add CHECK constraint on `account_type` |
| 3 | Drop old `splits`/`transactions` tables, recreate with `debit`/`credit` as INTEGER (cents), `post_date`, `num`, `reconcile_state` |

Fresh databases skip all migrations — they are created at the latest schema version directly.
