use sqlx::SqlitePool;

use crate::{errors::AppResult, models::note::{CreateNoteRequest, Note, UpdateNoteRequest}};

pub async fn create(pool: &SqlitePool, req: CreateNoteRequest) -> AppResult<Note> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        INSERT INTO notes (title, content, folder_id)
        VALUES (?1, ?2, ?3)
        RETURNING *
        "#,
    )
    .bind(req.title)
    .bind(req.content)
    .bind(req.folder_id)
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn find_all(pool: &SqlitePool) -> AppResult<Vec<Note>> {
    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT
            n.*,
            f.name AS folder_name
        FROM notes n
        LEFT JOIN folders f
            ON n.folder_id = f.id
        WHERE n.is_deleted = 0
        ORDER BY
            n.is_pinned DESC,
            n.updated_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

pub async fn find_by_folder_id(
    pool: &SqlitePool,
    folder_id: i64,
) -> AppResult<Vec<Note>> {
    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT
            n.*,            
            f.name AS folder_name            
        FROM notes n
        LEFT JOIN folders f
            ON n.folder_id = f.id
        WHERE n.folder_id = ?
          AND n.is_deleted = 0
        ORDER BY
            n.is_pinned DESC,
            n.updated_at DESC
        "#
    )
    .bind(folder_id)
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

pub async fn find_one(pool: &SqlitePool, id: i64) -> AppResult<Note> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        SELECT
            n.*,            
            f.name AS folder_name            
        FROM notes n
        LEFT JOIN folders f
            ON n.folder_id = f.id
        WHERE n.id = ?
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn update(pool: &SqlitePool, req: UpdateNoteRequest) -> AppResult<Note> {
    let current = sqlx::query_as::<_, Note>(
        r#"
        SELECT
            n.*,            
            f.name AS folder_name            
        FROM notes n
        LEFT JOIN folders f
            ON n.folder_id = f.id
        WHERE n.id = ?
        "#,
    )
    .bind(req.id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        UPDATE notes
        SET title = ?1,
            content = ?2,
            folder_id = ?3,
            is_pinned = ?4,
            is_archived = ?5,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?6
        "#,
    )
    .bind(req.title.unwrap_or(current.title))
    .bind(req.content.unwrap_or(current.content))
    .bind(req.folder_id.unwrap_or(current.folder_id))
    .bind(req.is_pinned.unwrap_or(current.is_pinned))
    .bind(req.is_archived.unwrap_or(current.is_archived))
    .bind(req.id)
    .execute(pool)
    .await?;

    let note = sqlx::query_as::<_, Note>(
        r#"
        SELECT
            n.*,            
            f.name AS folder_name            
        FROM notes n
        LEFT JOIN folders f
            ON n.folder_id = f.id
        WHERE n.id = ?
        "#,
    )
    .bind(req.id)
    .fetch_one(pool)
    .await?;

    Ok(note)
}
pub async fn soft_delete(pool: &SqlitePool, id: i64) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE notes
        SET is_deleted = 1,
            deleted_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
