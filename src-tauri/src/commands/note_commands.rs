use tauri::State;

use crate::{db::AppState, errors::AppResult, models::note::{CreateNoteRequest, Note, UpdateNoteRequest}, services::note_service};

#[tauri::command]
pub async fn create_note(state: State<'_, AppState>, req: CreateNoteRequest) -> AppResult<Note> {
    note_service::create_note(&state.pool, req).await
}

#[tauri::command]
pub async fn get_notes(state: State<'_, AppState>) -> AppResult<Vec<Note>> {
    note_service::get_notes(&state.pool).await
}

#[tauri::command]
pub async fn get_one_note(state: State<'_, AppState>, id: i64) -> AppResult<Note> {
    note_service::get_one_note(&state.pool, id).await
}

#[tauri::command]
pub async fn update_note(state: State<'_, AppState>, req: UpdateNoteRequest) -> AppResult<Note> {
    note_service::update_note(&state.pool, req).await
}

#[tauri::command]
pub async fn delete_note(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    note_service::delete_note(&state.pool, id).await
}
