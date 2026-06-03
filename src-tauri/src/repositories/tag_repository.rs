use sqlx::SqlitePool;

use crate::{errors::AppResult, models::tag::{CreateTagRequest, Tag}};

pub async fn create(pool: &SqlitePool, req: CreateTagRequest) -> AppResult<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        INSERT INTO tags (name, color)
        VALUES (?1, ?2)
        RETURNING *
        "#,
    )
    .bind(req.name)
    .bind(req.color)
    .fetch_one(pool)
    .await?;

    Ok(tag)
}

pub async fn find_all(pool: &SqlitePool) -> AppResult<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name ASC")
        .fetch_all(pool)
        .await?;

    Ok(tags)
}
