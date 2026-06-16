use tauri::State;

use crate::{db::sqlite::AppState, errors::AppResult, models::tag::{CreateTagRequest, Tag}, services::tag_service};

#[tauri::command]
pub async fn create_tag(state: State<'_, AppState>, req: CreateTagRequest) -> AppResult<Tag> {
    tag_service::create_tag(&state.pool, req).await
}

#[tauri::command]
pub async fn get_tags(state: State<'_, AppState>) -> AppResult<Vec<Tag>> {
    tag_service::get_tags(&state.pool).await
}
