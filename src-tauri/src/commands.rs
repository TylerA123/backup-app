use std::sync::Arc;
use tauri::State;
use crate::db::LocalDb;
use crate::sync::SyncEngine;
use crate::errors::AppError;

pub struct AppState {
    pub db: Arc<LocalDb>,
    pub sync_engine: Arc<SyncEngine>,
}

#[tauri::command]
pub fn list_projects(state: State<'_, AppState>) -> Result<Vec<crate::db::local::LocalProject>, String> {
    state.db.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_project(
    state: State<'_, AppState>,
    id: String,
    name: String,
    local_path: String,
) -> Result<(), String> {
    state.db.add_project(&id, &name, &local_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn trigger_snapshot(
    state: State<'_, AppState>,
    project_id: String,
    project_path: String,
) -> Result<String, String> {
    state.sync_engine.trigger_snapshot(&project_id, &project_path)
}

#[tauri::command]
pub fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>, String> {
    state.db.get_setting(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    state.db.set_setting(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_upload_count(state: State<'_, AppState>) -> Result<i64, String> {
    state.db
        .get_pending_uploads()
        .map(|items| items.len() as i64)
        .map_err(|e| e.to_string())
}
