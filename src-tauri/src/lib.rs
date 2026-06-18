pub mod commands;
pub mod db;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use crate::db::sqlite::{AppState, init_db};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, menu::{Menu, MenuItem}, tray::TrayIconBuilder,};

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
                init_db(&database_url)
                    .await
                    .expect("DB init failed")
            });

            app.manage(AppState { pool });

            let manager = MenuItem::with_id(app, "manager", "메모 관리자 열기", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&manager, &quit])?;
            let icon = app.default_window_icon().unwrap().clone();

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "manager" => {
                            if let Some(win) = app.get_webview_window("manager") {
                                let _ = win.show();
                                let _ = win.unminimize();
                                let _ = win.set_focus();
                            } else {
                                let _ = WebviewWindowBuilder::new(
                                    app,
                                    "manager",
                                    WebviewUrl::App("/#/manager".into())
                                )
                                .title("메모 관리")
                                .inner_size(1000.0, 700.0)
                                .visible(true)
                                .build();
                            }
                        }
                        "quit" => app.exit(0),
                        _ => {}
                    }
                })
                .build(app)?;

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
            commands::window_commands::preload_sticky_window,
            commands::window_commands::show_sticky_window,
            commands::window_commands::hide_sticky_window,
            commands::window_commands::show_current_window,
            commands::window_commands::close_current_window,
            commands::window_commands::close_sticky_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
