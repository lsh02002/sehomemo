use sqlx::SqlitePool;

use crate::{errors::AppResult, models::note::{CreateNoteRequest, Note, UpdateNoteRequest}, repositories::note_repository};

pub async fn create_note(pool: &SqlitePool, req: CreateNoteRequest) -> AppResult<Note> {
    note_repository::create(pool, req).await
}

pub async fn get_notes(pool: &SqlitePool) -> AppResult<Vec<Note>> {
    note_repository::find_all(pool).await
}

pub async fn get_one_note(pool: &SqlitePool, id: i64) -> AppResult<Note> {
    note_repository::find_one(pool, id).await
}

pub async fn update_note(pool: &SqlitePool, req: UpdateNoteRequest) -> AppResult<Note> {
    note_repository::update(pool, req).await
}

pub async fn delete_note(pool: &SqlitePool, id: i64) -> AppResult<()> {
    note_repository::soft_delete(pool, id).await
}
