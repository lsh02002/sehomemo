use sqlx::SqlitePool;

use crate::{errors::AppResult, models::tag::{CreateTagRequest, Tag}, repositories::tag_repository};

pub async fn create_tag(pool: &SqlitePool, req: CreateTagRequest) -> AppResult<Tag> {
    tag_repository::create(pool, req).await
}

pub async fn get_tags(pool: &SqlitePool) -> AppResult<Vec<Tag>> {
    tag_repository::find_all(pool).await
}
