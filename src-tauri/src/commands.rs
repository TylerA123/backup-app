use std::sync::Arc;
use tauri::State;
use crate::db::LocalDb;
use crate::sync::SyncEngine;
use crate::api::supabase::SupabaseClient;
use crate::api::b2::B2Client;
use crate::api::config::AppConfig;

pub struct AppState {
    pub db: Arc<LocalDb>,
    pub sync_engine: Arc<SyncEngine>,
    pub supabase: Arc<SupabaseClient>,
    pub b2: Arc<B2Client>,
    pub config: Arc<AppConfig>,
}

#[tauri::command]
pub async fn sign_up(
    state: State<'_, AppState>,
    email: String,
    password: String,
) -> Result<crate::api::supabase::AuthSession, String> {
    state.supabase.sign_up(&email, &password).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sign_in(
    state: State<'_, AppState>,
    email: String,
    password: String,
) -> Result<crate::api::supabase::AuthSession, String> {
    state.supabase.sign_in(&email, &password).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sign_out(
    state: State<'_, AppState>,
    access_token: String,
) -> Result<(), String> {
    state.supabase.sign_out(&access_token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user(
    state: State<'_, AppState>,
    access_token: String,
) -> Result<crate::api::supabase::AuthUser, String> {
    state.supabase.get_user(&access_token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn health_check(state: State<'_, AppState>) -> Result<bool, String> {
    state.supabase.health_check().await.map_err(|e| e.to_string())
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
