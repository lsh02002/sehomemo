use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri::webview::Color;
use crate::db::AppState;
use crate::services::note_service;

#[tauri::command]
pub async fn open_manager_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("manager") {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(
        &app,
        "manager",
        WebviewUrl::App("/#/manager".into()),
    )
    .title("메모 관리")
    .inner_size(1000.0, 700.0)
    .resizable(true)
    .decorations(true)
    .visible(false)
    .background_color(Color(33, 37, 41, 255))
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_empty_sticky_window(
    app: AppHandle,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("empty-sticky") {        
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(
        &app,
        "empty-sticky",
        WebviewUrl::App("/#/sticky/new".into()),
    )
    .title("새 메모")
    .inner_size(280.0, 280.0)
    .resizable(true)
    .decorations(false)
    .always_on_top(true)
    .visible(false)
    .background_color(Color(255, 193, 7, 255))
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_sticky_window(
    app: AppHandle,
    note_id: i64,
) -> Result<(), String> {
    let label = format!("sticky-{}", note_id);
    let url = format!("/#/sticky/{}", note_id);

    if let Some(window) = app.get_webview_window(&label) {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(
        &app,
        label,
        WebviewUrl::App(url.into()),
    )
    .title("포스트잇")
    .inner_size(280.0, 280.0)
    .resizable(true)
    .decorations(false)
    .always_on_top(true)
    .visible(false)
    .background_color(Color(255, 193, 7, 255))
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn show_current_window(
    window: tauri::WebviewWindow,
) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn close_current_window(
    window: tauri::WebviewWindow,
) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn close_sticky_window(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<(), String> {
    note_service::update_pinned_note(&state.pool, note_id)
    .await
    .map_err(|e| e.to_string())?;

    app.emit("sticky-closed", serde_json::json!({
        "id": note_id
    }))
    .map_err(|e| e.to_string())?;

    let label = format!("sticky-{}", note_id);

    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn preload_sticky_window(
    app: AppHandle,
    note_id: i64,
) -> Result<(), String> {
    let label = format!("sticky-{}", note_id);

    if app.get_webview_window(&label).is_some() {
        return Ok(());
    }

    let url = format!("/#/sticky/{}", note_id);

    WebviewWindowBuilder::new(
        &app,
        label,
        WebviewUrl::App(url.into()),
    )
    .title("포스트잇")
    .inner_size(280.0, 280.0)
    .resizable(true)
    .decorations(false)
    .always_on_top(true)
    .visible(false)
    .background_color(Color(255, 193, 7, 255))
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn show_sticky_window(
    app: AppHandle,
    note_id: i64,
) -> Result<(), String> {
    let label = format!("sticky-{}", note_id);

    if let Some(window) = app.get_webview_window(&label) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn hide_sticky_window(
    app: AppHandle,
    note_id: i64,
) -> Result<(), String> {    
    let label = format!("sticky-{}", note_id);

    if let Some(window) = app.get_webview_window(&label) {
        window.hide().map_err(|e| e.to_string())?;        
    }

    exit_if_no_visible_windows(&app);

    Ok(())
}

fn exit_if_no_visible_windows(app: &AppHandle) {
    let has_visible_window = app
        .webview_windows()
        .values()
        .any(|w| w.is_visible().unwrap_or(false));    

    if !has_visible_window {
        println!("{}", "종료됨");
        app.exit(0);
    }
}