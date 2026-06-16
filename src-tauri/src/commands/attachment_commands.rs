use tauri::State;

use crate::{
    db::sqlite::AppState,
    errors::AppResult,
    models::attachment::{Attachment, CreateAttachmentRequest},
    services::attachment_service,
};

#[tauri::command]
pub async fn get_attachments_by_note_id(
    state: State<'_, AppState>,
    note_id: i64,
) -> AppResult<Vec<Attachment>> {
    attachment_service::get_attachments_by_note_id(&state.pool, note_id).await
}

#[tauri::command]
pub async fn create_attachment(
    state: State<'_, AppState>,
    req: CreateAttachmentRequest,
) -> AppResult<Attachment> {
    attachment_service::create_attachment(&state.pool, req).await
}

#[tauri::command]
pub async fn delete_attachment(
    state: State<'_, AppState>,
    id: i64,
) -> AppResult<()> {
    attachment_service::delete_attachment(&state.pool, id).await
}
