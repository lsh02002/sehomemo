pub mod commands;
pub mod db;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use crate::db::AppState;
use tauri::Manager;

pub fn run() {
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
            commands::note_commands::create_note,
            commands::note_commands::get_notes,
            commands::note_commands::get_notes_by_folder_id,
            commands::note_commands::get_pinned_notes,
            commands::note_commands::get_notes_by_keyword,
            commands::note_commands::get_deleted_notes,
            commands::note_commands::get_one_note,
            commands::note_commands::restore_note,
            commands::note_commands::update_note,
            commands::note_commands::update_note_silent,
            commands::note_commands::delete_note_softly,
            commands::note_commands::delete_note_permanently,
            commands::folder_commands::create_folder,
            commands::folder_commands::get_folders,
            commands::tag_commands::create_tag,
            commands::tag_commands::get_tags,
            commands::attachment_commands::get_attachments_by_note_id,
            commands::attachment_commands::create_attachment,
            commands::attachment_commands::delete_attachment,
            commands::window_commands::open_manager_window,
            commands::window_commands::open_empty_sticky_window,
            commands::window_commands::open_sticky_window,
            commands::window_commands::close_current_window,
            commands::window_commands::close_sticky_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
