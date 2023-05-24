use crate::database::{conn, pg_interval_to_std_time_duration};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{types::PgInterval, PgRow},
    FromRow, Row,
};
use std::{path, time};

use crate::book::add_a_bunch_of_books_to_db;
use crate::user::add_a_bunch_of_users_to_db;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaybackLocation {
    book_id: i64,
    user_id: i64,
    time: time::Duration,
}

impl FromRow<'_, PgRow> for PlaybackLocation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let book_id = row.get::<i64, usize>(0);
        let user_id = row.get::<i64, usize>(1);

        let time_from_row = row.get(1);
        let time = pg_interval_to_std_time_duration(time_from_row);

        Ok(PlaybackLocation {
            book_id,
            user_id,
            time,
        })
    }
}

impl PlaybackLocation {
    pub fn new(book_id: i64, user_id: i64, time: time::Duration) -> Self {
        Self {
            book_id,
            user_id,
            time,
        }
    }

    async fn upsert_to_db(&self) -> Result<(), sqlx::Error> {
        let mut conn = conn().await?;

        let query_upsert = r#"
            INSERT INTO playback_locations (book_id, user_id, time)
            VALUES ($1, $2, $3)
            ON CONFLICT (book_id, user_id) DO UPDATE
            SET time = EXCLUDED.time;
        "#;

        sqlx::query(query_upsert)
            .bind(&self.book_id)
            .bind(&self.user_id)
            .bind(&self.time)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

use rand::Rng;
/// Add back to tests mod?
fn random_playback_time(n: u8) -> PlaybackLocation {
    let mut rng = rand::thread_rng();
    let random_user = rng.gen_range(1..=n);
    let random_book = rng.gen_range(1..=n);

    let random_u16: u16 = rand::thread_rng().gen();
    let time = time::Duration::from_millis(random_u16 as u64);

    PlaybackLocation {
        user_id: random_user as i64,
        book_id: random_book as i64,
        time,
    }
}

async fn add_random_playback_time_to_db(n: u8) {
    let test_playback_time = random_playback_time(n);
    let serialized = serde_json::to_string(&test_playback_time).expect("Failed to serialize");
    println!("add_random_playback_time_to_db(): {}", serialized);
    test_playback_time.upsert_to_db().await.unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_duplicate_playback_time() {
        add_a_bunch_of_books_to_db(true, 3).await;
        add_a_bunch_of_users_to_db(false, 3).await;

        let test_playback_time = PlaybackLocation::new(1, 1, time::Duration::from_secs(0));
        test_playback_time.upsert_to_db().await.unwrap();
        test_playback_time.upsert_to_db().await.unwrap(); // 2nd call shouldn't panic
    }

    #[tokio::test]
    async fn test_add_a_bunch_of_playback_times_to_db() {
        add_a_bunch_of_playback_times_to_db(true, 20, 50).await;
    }
}
