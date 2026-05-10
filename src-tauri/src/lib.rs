mod api;
mod commands;
mod crypto;
mod db;
mod download;
mod errors;
mod hasher;
mod sync;
mod upload;
mod watcher;

use std::sync::Arc;
use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("studio-backup")
        .join("studio-backup.db");

    std::fs::create_dir_all(db_path.parent().unwrap()).ok();

    let db = Arc::new(
        db::LocalDb::new(&db_path).expect("Failed to initialize database"),
    );

    let config = Arc::new(api::config::AppConfig::from_env());
    let supabase = Arc::new(api::supabase::SupabaseClient::new(
        config.supabase_url.clone(),
        config.supabase_anon_key.clone(),
    ));
    let b2 = Arc::new(api::b2::B2Client::new(
        &config.b2_key_id,
        &config.b2_app_key,
        &config.b2_bucket,
        &config.b2_endpoint,
    ));

    let sync_engine = Arc::new(sync::SyncEngine::new(db.clone()));

    tauri::Builder::default()
        .manage(AppState {
            db,
            sync_engine,
            supabase,
            b2,
            config,
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.handle().plugin(tauri_plugin_store::Builder::default().build())?;
            app.handle().plugin(tauri_plugin_dialog::init())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sign_up,
            commands::sign_in,
            commands::sign_out,
            commands::get_user,
            commands::health_check,
            commands::list_projects,
            commands::add_project,
            commands::trigger_snapshot,
            commands::get_setting,
            commands::set_setting,
            commands::get_pending_upload_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
