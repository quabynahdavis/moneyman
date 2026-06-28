pub mod schema;

use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
        let db_path = app_data_dir.join("moneyman.db");
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Create tables (IF NOT EXISTS so existing tables are preserved)
        for stmt in schema::CREATE_TABLES {
            conn.execute_batch(stmt)?;
        }

        // Check current schema version
        let current_version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if current_version == 0 {
            // Fresh database: insert seed data and stamp latest version
            for stmt in schema::INSERT_SEED_DATA {
                conn.execute_batch(stmt)?;
            }
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?1)",
                [schema::SCHEMA_VERSION],
            )?;
        } else {
            // Existing database: run pending migrations sequentially
            for (target_version, migration_stmts) in schema::MIGRATIONS {
                if current_version < *target_version {
                    for stmt in *migration_stmts {
                        conn.execute_batch(stmt)?;
                    }
                    conn.execute(
                        "INSERT INTO schema_version (version) VALUES (?1)",
                        [*target_version],
                    )?;
                }
            }
        }

        Ok(())
    }
}
