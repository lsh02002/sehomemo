use sqlx::SqlitePool;

use crate::{errors::AppResult, models::note::{CreateNoteRequest, Note, UpdateNoteRequest}, repositories::note_repository};

pub async fn create_note(pool: &SqlitePool, req: CreateNoteRequest) -> AppResult<Note> {
    note_repository::create(pool, req).await
}

pub async fn get_notes(pool: &SqlitePool) -> AppResult<Vec<Note>> {
    note_repository::find_all(pool).await
}

pub async fn get_notes_by_folder_id(pool: &SqlitePool, id: i64) -> AppResult<Vec<Note>> {
    note_repository::find_by_folder_id(pool, id).await
}

pub async fn get_pinned_notes(pool: &SqlitePool) -> AppResult<Vec<Note>> {
    note_repository::find_pinned_all(pool).await
}

pub async fn get_notes_by_keyword(pool: &SqlitePool, keyword: String) -> AppResult<Vec<Note>> {
    note_repository::find_by_keyword(pool, keyword).await
}

pub async fn get_deleted_notes(pool: &SqlitePool) -> AppResult<Vec<Note>> {
    note_repository::find_deleted_all(pool).await
}

pub async fn get_one_note(pool: &SqlitePool, id: i64) -> AppResult<Note> {
    note_repository::find_one(pool, id).await
}

pub async fn restore_note(pool: &SqlitePool, id: i64) -> AppResult<Note> {
    note_repository::restore_one(pool, id).await
}

pub async fn update_note(pool: &SqlitePool, req: UpdateNoteRequest) -> AppResult<Note> {
    note_repository::update(pool, req).await
}

pub async fn update_note_silent(pool: &SqlitePool, req: UpdateNoteRequest) -> AppResult<Note> {
    note_repository::update_silent(pool, req).await
}

pub async fn delete_note_softly(pool: &SqlitePool, id: i64) -> AppResult<()> {
    note_repository::soft_delete(pool, id).await
}

pub async fn delete_note_permanently(pool: &SqlitePool, id: i64) -> AppResult<()> {
    note_repository::delete(pool, id).await
}