use sqlx::SqlitePool;

use crate::{
    errors::AppResult,
    models::attachment::{Attachment, CreateAttachmentRequest},
    repositories::attachment_repository,
};

pub async fn get_attachments_by_note_id(
    pool: &SqlitePool,
    note_id: i64,
) -> AppResult<Vec<Attachment>> {
    attachment_repository::find_by_note_id(pool, note_id).await
}

pub async fn create_attachment(
    pool: &SqlitePool,
    req: CreateAttachmentRequest,
) -> AppResult<Attachment> {
    attachment_repository::create(pool, req).await
}

pub async fn delete_attachment(
    pool: &SqlitePool,
    id: i64,
) -> AppResult<()> {
    attachment_repository::delete(pool, id).await
}