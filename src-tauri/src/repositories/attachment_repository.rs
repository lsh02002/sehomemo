use sqlx::SqlitePool;

use crate::{
    errors::AppResult,
    models::attachment::{Attachment, CreateAttachmentRequest},
};

pub async fn find_by_note_id(
    pool: &SqlitePool,
    note_id: i64,
) -> AppResult<Vec<Attachment>> {
    let attachments = sqlx::query_as::<_, Attachment>(
        r#"
        SELECT id, note_id, file_name, file_path, mime_type, size, created_at
        FROM attachments
        WHERE note_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(attachments)
}

pub async fn create(
    pool: &SqlitePool,
    req: CreateAttachmentRequest,
) -> AppResult<Attachment> {
    let attachment = sqlx::query_as::<_, Attachment>(
        r#"
        INSERT INTO attachments (note_id, file_name, file_path, mime_type, size)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id, note_id, file_name, file_path, mime_type, size, created_at
        "#,
    )
    .bind(req.note_id)
    .bind(req.file_name)
    .bind(req.file_path)
    .bind(req.mime_type)
    .bind(req.size)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(attachment)
}

pub async fn delete(
    pool: &SqlitePool,
    id: i64,
) -> AppResult<()> {
    sqlx::query(
        r#"
        DELETE FROM attachments
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}