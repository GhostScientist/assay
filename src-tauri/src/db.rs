use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

const MIGRATIONS: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS eval_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    eval_id TEXT NOT NULL,
    model_id TEXT NOT NULL,
    started_at DATETIME NOT NULL,
    completed_at DATETIME,
    status TEXT NOT NULL,
    config_json TEXT NOT NULL,
    metrics_json TEXT
);

CREATE TABLE IF NOT EXISTS samples (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL REFERENCES eval_runs(id),
    index_num INTEGER NOT NULL,
    input_json TEXT NOT NULL,
    output_json TEXT,
    scores_json TEXT,
    trajectory_json TEXT,
    status TEXT NOT NULL,
    latency_ms INTEGER,
    tokens_input INTEGER,
    tokens_output INTEGER
);

CREATE VIRTUAL TABLE IF NOT EXISTS samples_fts USING fts5(
    input_text,
    output_text,
    content='samples',
    content_rowid='rowid'
);

CREATE TABLE IF NOT EXISTS annotations (
    id TEXT PRIMARY KEY,
    sample_id TEXT NOT NULL REFERENCES samples(id),
    author TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    annotation_type TEXT NOT NULL,
    content TEXT NOT NULL
);
"#;

pub fn init_db(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create db directory: {err}"))?;
    }

    let connection = Connection::open(path)
        .map_err(|err| format!("Failed to open database: {err}"))?;
    apply_migrations(&connection).map_err(|err| format!("Migration error: {err}"))?;
    Ok(())
}

fn apply_migrations(connection: &Connection) -> SqlResult<()> {
    connection.execute_batch(MIGRATIONS)
}
