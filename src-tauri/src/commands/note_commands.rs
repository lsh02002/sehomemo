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
pub async fn get_notes_by_folder_id(state: State<'_, AppState>, id: i64) -> AppResult<Vec<Note>> {
    note_service::get_notes_by_folder_id(&state.pool, id).await
}

#[tauri::command]
pub async fn get_notes_by_keyword(state: State<'_, AppState>, keyword: String) -> AppResult<Vec<Note>> {
    note_service::get_notes_by_keyword(&state.pool, keyword).await
}

#[tauri::command]
pub async fn get_deleted_notes(state: State<'_, AppState>) -> AppResult<Vec<Note>> {
    note_service::get_deleted_notes(&state.pool).await
}

#[tauri::command]
pub async fn get_one_note(state: State<'_, AppState>, id: i64) -> AppResult<Note> {
    note_service::get_one_note(&state.pool, id).await
}

#[tauri::command]
pub async fn restore_note(state: State<'_, AppState>, id: i64) -> AppResult<Note> {
    note_service::restore_note(&state.pool, id).await
}

#[tauri::command]
pub async fn update_note(state: State<'_, AppState>, req: UpdateNoteRequest) -> AppResult<Note> {
    note_service::update_note(&state.pool, req).await
}

#[tauri::command]
pub async fn delete_note_softly(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    note_service::delete_note_softly(&state.pool, id).await
}

#[tauri::command]
pub async fn delete_note_permanently(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    note_service::delete_note_permanently(&state.pool, id).await
}