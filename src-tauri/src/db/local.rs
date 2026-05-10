use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Mutex;
use crate::errors::AppError;

pub struct LocalDb {
    conn: Mutex<Connection>,
}

impl LocalDb {
    pub fn new(db_path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = LocalDb { conn: Mutex::new(conn) };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS local_projects (
                id                  TEXT PRIMARY KEY,
                name                TEXT NOT NULL,
                local_path          TEXT NOT NULL,
                last_synced_at      TEXT,
                server_id           TEXT,
                is_deleted          INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS local_snapshots (
                id                  TEXT PRIMARY KEY,
                project_id          TEXT NOT NULL REFERENCES local_projects(id),
                fingerprint         TEXT NOT NULL,
                file_count          INTEGER NOT NULL,
                total_size          INTEGER NOT NULL,
                trigger             TEXT DEFAULT 'auto',
                synced              INTEGER DEFAULT 0,
                created_at          TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS upload_queue (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id          TEXT NOT NULL,
                relative_path       TEXT NOT NULL,
                content_hash        TEXT NOT NULL,
                file_size           INTEGER NOT NULL,
                status              TEXT DEFAULT 'pending',
                retry_count         INTEGER DEFAULT 0,
                last_error          TEXT,
                chunked             INTEGER DEFAULT 0,
                upload_id           TEXT,
                chunks_completed    TEXT DEFAULT '[]',
                created_at          TEXT DEFAULT (datetime('now')),
                updated_at          TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS file_cache (
                content_hash        TEXT PRIMARY KEY,
                storage_key         TEXT NOT NULL,
                local_path          TEXT NOT NULL,
                file_size           INTEGER NOT NULL,
                last_accessed       TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS app_settings (
                key                 TEXT PRIMARY KEY,
                value               TEXT NOT NULL
            );

            INSERT OR IGNORE INTO app_settings (key, value) VALUES
                ('debounce_ms', '30000'),
                ('max_concurrent_uploads', '3'),
                ('chunk_size_mb', '5'),
                ('snapshot_interval_minutes', '60'),
                ('version', '1');"
        )?;
        Ok(())
    }

    pub fn add_project(&self, id: &str, name: &str, local_path: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO local_projects (id, name, local_path) VALUES (?1, ?2, ?3)",
            params![id, name, local_path],
        )?;
        Ok(())
    }

    pub fn get_projects(&self) -> Result<Vec<LocalProject>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, local_path, last_synced_at, server_id, is_deleted FROM local_projects WHERE is_deleted = 0"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(LocalProject {
                id: row.get(0)?,
                name: row.get(1)?,
                local_path: row.get(2)?,
                last_synced_at: row.get(3)?,
                server_id: row.get(4)?,
                is_deleted: row.get(5)?,
            })
        })?;
        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }
        Ok(projects)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM app_settings WHERE key = ?1")?;
        let mut rows = stmt.query_map(params![key], |row| row.get::<_, String>(0))?;
        Ok(rows.next().transpose()?)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn add_to_upload_queue(
        &self,
        project_id: &str,
        relative_path: &str,
        content_hash: &str,
        file_size: i64,
    ) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO upload_queue (project_id, relative_path, content_hash, file_size)
             VALUES (?1, ?2, ?3, ?4)",
            params![project_id, relative_path, content_hash, file_size],
        )?;
        Ok(())
    }

    pub fn get_pending_uploads(&self) -> Result<Vec<UploadQueueItem>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, relative_path, content_hash, file_size, status, retry_count, last_error
             FROM upload_queue WHERE status = 'pending' ORDER BY file_size ASC LIMIT 50"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(UploadQueueItem {
                id: row.get(0)?,
                project_id: row.get(1)?,
                relative_path: row.get(2)?,
                content_hash: row.get(3)?,
                file_size: row.get(4)?,
                status: row.get(5)?,
                retry_count: row.get(6)?,
                last_error: row.get(7)?,
            })
        })?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalProject {
    pub id: String,
    pub name: String,
    pub local_path: String,
    pub last_synced_at: Option<String>,
    pub server_id: Option<String>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadQueueItem {
    pub id: i64,
    pub project_id: String,
    pub relative_path: String,
    pub content_hash: String,
    pub file_size: i64,
    pub status: String,
    pub retry_count: i32,
    pub last_error: Option<String>,
}
