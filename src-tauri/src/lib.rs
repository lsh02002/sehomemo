pub mod commands;
pub mod db;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use db::sqlite::init_db;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = init_db(&handle).await?;
                app.manage(db::AppState { pool });
                Ok::<(), errors::AppError>(())
            })?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::note_commands::create_note,
            commands::note_commands::get_notes,
            commands::note_commands::update_note,
            commands::note_commands::delete_note,
            commands::folder_commands::create_folder,
            commands::folder_commands::get_folders,
            commands::tag_commands::create_tag,
            commands::tag_commands::get_tags,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
