use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_manager_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("manager") {
        window.show().map_err(|e| e.to_string())?;
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
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_empty_sticky_window(
    app: AppHandle,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("empty-sticky") {
        window.show().map_err(|e| e.to_string())?;
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
    .always_on_top(true)
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
        window.show().map_err(|e| e.to_string())?;
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
    .build()
    .map_err(|e| e.to_string())?;

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
    note_id: i64,
) -> Result<(), String> {
    let label = format!("sticky-{}", note_id);

    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }

    Ok(())
}