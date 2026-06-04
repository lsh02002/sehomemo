#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod db;
mod models;
mod commands;
mod repositories;
mod services;

use db::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir)
                .expect("failed to create app data dir");

            let db_path = app_data_dir.join("notes.db");

            let database_url = format!(
                "sqlite://{}?mode=rwc",
                db_path.to_string_lossy().replace('\\', "/")
            );

            let pool = tauri::async_runtime::block_on(async {
                db::init_db(&database_url)
                    .await
                    .expect("DB init failed")
            });

            app.manage(AppState { pool });

            Ok(())
        })        
        .invoke_handler(tauri::generate_handler![
            commands::folder_commands::create_folder,
            commands::folder_commands::get_folders,
            commands::note_commands::create_note,
            commands::note_commands::get_notes,
            commands::note_commands::get_one_note,
            commands::note_commands::update_note,
            commands::note_commands::delete_note,
            commands::tag_commands::create_tag,
            commands::tag_commands::get_tags,
            commands::attachment_commands::get_attachments_by_note_id,
            commands::attachment_commands::create_attachment,
            commands::attachment_commands::delete_attachment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}