# Migrations

Moneyman uses schema versioning stored in a `_schema_version` table:

```sql
CREATE TABLE _schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

Migrations are Rust functions that check `_schema_version` and apply DDL patches sequentially. This keeps the schema in sync across app updates without external tools.
