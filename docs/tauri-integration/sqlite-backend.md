# SQLite Backend

The Rust backend uses `rusqlite` (or `tauri-plugin-sql`) to manage a local SQLite database.

## Database Location

`~/.moneyman/data.db` (platform-specific app data directory via Tauri's `app_data_dir`).

## Key Rust Dependencies (Planned)

```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
rust_decimal = "1.33"
serde = { version = "1", features = ["derive"] }
tauri = { version = "2", features = ["sql"] }
```

## Thread Safety

SQLite connections are wrapped in `Mutex<Connection>` and managed through a Tauri state struct. All query functions acquire the lock, execute, and release — ensuring single-writer semantics for the database file.
