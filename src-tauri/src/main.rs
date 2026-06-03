#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod db;
mod models;
mod commands;
mod repositories;
mod services;

use db::AppState;

fn main() {
    let app_data_dir = std::env::current_dir()
        .expect("failed to get current dir");

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

    tauri::Builder::default()
        .manage(AppState { pool })
        .invoke_handler(tauri::generate_handler![
            commands::folder_commands::create_folder,
            commands::folder_commands::get_folders,
            commands::note_commands::create_note,
            commands::note_commands::get_notes,
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