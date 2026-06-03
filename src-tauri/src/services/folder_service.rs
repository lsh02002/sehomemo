use sqlx::SqlitePool;

use crate::{errors::AppResult, models::folder::{CreateFolderRequest, Folder}, repositories::folder_repository};

pub async fn create_folder(pool: &SqlitePool, req: CreateFolderRequest) -> AppResult<Folder> {
    folder_repository::create(pool, req).await
}

pub async fn get_folders(pool: &SqlitePool) -> AppResult<Vec<Folder>> {
    folder_repository::find_all(pool).await
}
