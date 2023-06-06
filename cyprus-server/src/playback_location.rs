use crate::database::{conn, pg_interval_to_std_time_duration};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::time;

use crate::book::add_a_bunch_of_books_to_db;
use crate::user::add_a_bunch_of_users_to_db;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaybackLocation {
    pub book_name: String,
    pub user_name: String,
    time: time::Duration,
}

impl FromRow<'_, PgRow> for PlaybackLocation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let book_name = row.get::<String, usize>(0);
        let user_name = row.get::<String, usize>(1);

        let time_from_row = row.get(2);
        let time = pg_interval_to_std_time_duration(time_from_row);

        Ok(PlaybackLocation {
            book_name,
            user_name,
            time,
        })
    }
}

impl PlaybackLocation {
    pub fn new(book_name: String, user_name: String, time: time::Duration) -> Self {
        Self {
            book_name,
            user_name,
            time,
        }
    }

    async fn upsert_to_db(&self) -> Result<(), sqlx::Error> {
        let query_upsert_by_names = r#"
            WITH user_row AS (
                SELECT id AS user_id
                FROM users
                WHERE username = $1
            ), book_row AS (
                SELECT id AS book_id
                FROM books
                WHERE name = $2
            )
            INSERT INTO playback_locations (book_id, user_id, time)
            VALUES (
                (SELECT book_id FROM book_row),
                (SELECT user_id FROM user_row),
                $3
            )
            ON CONFLICT (book_id, user_id)
            DO UPDATE SET time = EXCLUDED.time;
        "#;
    
        let mut conn = conn().await?;
    
        sqlx::query(query_upsert_by_names)
            .bind(&self.user_name)
            .bind(&self.book_name)
            .bind(self.time)
            .execute(&mut conn)
            .await?;
        Ok(())
    }
}

use rand::Rng;
/// Add back to tests mod?
fn random_playback_time_by_ids(n: u8) -> (i64, i64, time::Duration) {
    let mut rng = rand::thread_rng();
    let random_user = rng.gen_range(1..=n);
    let random_book = rng.gen_range(1..=n);

    let random_u16: u16 = rand::thread_rng().gen();
    let time = time::Duration::from_millis(random_u16 as u64);

    (random_book as i64, random_user as i64, time)
}

async fn add_random_playback_time_to_db(n: u8) {
    let (book_id, user_id, time) = random_playback_time_by_ids(n);
    upsert_to_db_by_ids(book_id, user_id, time).await.unwrap();
}

use crate::database::reset_tables;
/// Add back to tests mod?
pub async fn add_a_bunch_of_playback_times_to_db(reset_db_tables: bool, n: u8, new_times: u32) {
    if reset_db_tables {
        reset_tables().await.unwrap();
    }

    add_a_bunch_of_books_to_db(false, n).await;
    add_a_bunch_of_users_to_db(false, n).await;

    let mut handles = Vec::new();

    for _ in 0..new_times {
        let handle = tokio::task::spawn(async move { add_random_playback_time_to_db(n).await });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

// TODO make this function instead find the user_id and book_id to make a PlaybackLocation, then call upsert
pub async fn upsert_playback_location_to_db_from_username_and_book(
    user: &String,
    book: &String,
    time: &time::Duration,
) -> Result<(), sqlx::Error> {
    let query_upsert_by_names = r#"
        WITH user_row AS (
            SELECT id AS user_id
            FROM users
            WHERE username = $1
        ), book_row AS (
            SELECT id AS book_id
            FROM books
            WHERE name = $2
        )
        INSERT INTO playback_locations (book_id, user_id, time)
        VALUES (
            (SELECT book_id FROM book_row),
            (SELECT user_id FROM user_row),
            $3
        )
        ON CONFLICT (book_id, user_id)
        DO UPDATE SET time = EXCLUDED.time;
    "#;

    let mut conn = conn().await?;

    sqlx::query(query_upsert_by_names)
        .bind(user)
        .bind(book)
        .bind(time)
        .execute(&mut conn)
        .await?;

    Ok(())
}

async fn upsert_to_db_by_ids(book_id: i64, user_id: i64, time: std::time::Duration) -> Result<(), sqlx::Error> {
    let mut conn = conn().await?;

    let query_upsert = r#"
        INSERT INTO playback_locations (book_id, user_id, time)
        VALUES ($1, $2, $3)
        ON CONFLICT (book_id, user_id) DO UPDATE
        SET time = EXCLUDED.time;
    "#;

    sqlx::query(query_upsert)
        .bind(book_id)
        .bind(user_id)
        .bind(time)
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::random_book;
    use crate::user::random_user;

    #[tokio::test]
    async fn test_duplicate_playback_time() {
        add_a_bunch_of_books_to_db(true, 3).await;
        add_a_bunch_of_users_to_db(false, 3).await;

        upsert_to_db_by_ids(1, 1, time::Duration::from_secs(0)).await.unwrap();
        upsert_to_db_by_ids(1, 1, time::Duration::from_secs(1)).await.unwrap(); // 2nd call shouldn't panic
    }

    #[tokio::test]
    async fn test_add_a_bunch_of_playback_times_to_db() {
        add_a_bunch_of_playback_times_to_db(true, 20, 50).await;
    }

    #[tokio::test]
    async fn test_add_playback_time_by_name() {
        reset_tables().await.unwrap();
        let test_book = random_book();
        let test_user = random_user();
        test_book.add_to_db().await.unwrap();
        test_user.add_to_db().await.unwrap();
        let test_time = time::Duration::from_millis(42);
        upsert_playback_location_to_db_from_username_and_book(
            &test_user.username,
            &test_book.name,
            &test_time,
        )
        .await
        .unwrap();
        let test_time = time::Duration::from_millis(43);
        let test_playback_time = PlaybackLocation::new(
            test_book.name,
            test_user.username,
            test_time
        );
        test_playback_time.upsert_to_db().await.unwrap();
    }
}
