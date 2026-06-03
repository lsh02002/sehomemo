use tauri::State;

use crate::{db::AppState, errors::AppResult, models::folder::{CreateFolderRequest, Folder}, services::folder_service};

#[tauri::command]
pub async fn create_folder(state: State<'_, AppState>, req: CreateFolderRequest) -> AppResult<Folder> {
    folder_service::create_folder(&state.pool, req).await
}

#[tauri::command]
pub async fn get_folders(state: State<'_, AppState>) -> AppResult<Vec<Folder>> {
    folder_service::get_folders(&state.pool).await
}
