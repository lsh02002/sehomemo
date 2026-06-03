use sqlx::SqlitePool;

use crate::{errors::AppResult, models::folder::{CreateFolderRequest, Folder}};

pub async fn create(pool: &SqlitePool, req: CreateFolderRequest) -> AppResult<Folder> {
    let folder = sqlx::query_as::<_, Folder>(
        r#"
        INSERT INTO folders (name, parent_id)
        VALUES (?1, ?2)
        RETURNING *
        "#,
    )
    .bind(req.name)
    .bind(req.parent_id)
    .fetch_one(pool)
    .await?;

    Ok(folder)
}

pub async fn find_all(pool: &SqlitePool) -> AppResult<Vec<Folder>> {
    let folders = sqlx::query_as::<_, Folder>(
        "SELECT * FROM folders ORDER BY sort_order ASC, name ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(folders)
}
