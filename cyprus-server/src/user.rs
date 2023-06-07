use crate::database::conn;
use crate::playback_location::PlaybackLocation;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub username: String,
}

impl User {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    pub async fn add_to_db(&self) -> Result<(), sqlx::Error> {
        let mut conn = conn().await?;

        sqlx::query("INSERT INTO users (username) VALUES ($1)")
            .bind(&self.username)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_list_of_users(limit: Option<i64>) -> Result<Vec<User>, sqlx::Error> {
        let mut lim = limit.unwrap_or(i64::MAX);
        if lim < 0 {
            lim = i64::MAX;
        }

        let mut conn = conn().await?;

        sqlx::query_as::<_, User>("SELECT username FROM users LIMIT ($1)")
            .bind(lim)
            .fetch_all(&mut conn)
            .await
    }

    pub async fn get_list_of_playback_times(&self) -> Result<Vec<PlaybackLocation>, sqlx::Error> {
        let query_get_users_playback_times = r#"
            WITH user_row AS (
                SELECT id AS user_id
                FROM users
                WHERE username = $1
            )
            SELECT l.user_id, b.id as book_id, l.time
            FROM playback_locations l
            INNER JOIN books b
            ON b.id = l.book_id
            WHERE l.user_id = (SELECT user_id from user_row)
        "#;

        let mut conn = conn().await?;

        sqlx::query_as::<_, PlaybackLocation>(query_get_users_playback_times)
            .bind(&self.username)
            .fetch_all(&mut conn)
            .await
    }
}

use rand::Rng;
/// Add back to tests mod?
pub fn random_user() -> User {
    let random_vec_u8 = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(10)
        .collect::<Vec<u8>>();
    let random_string = String::from_utf8(random_vec_u8).unwrap();

    User {
        username: random_string,
    }
}

/// Add back to tests mod?
async fn add_random_user_to_db() {
    let test_book = random_user();
    test_book.add_to_db().await.unwrap();
}

use crate::database::reset_tables;
/// Add back to tests mod?
pub async fn add_a_bunch_of_users_to_db(reset_db_tables: bool, n: u8) {
    if reset_db_tables {
        reset_tables().await.unwrap();
    }

    let mut handles = Vec::new();

    for _ in 0..n {
        let handle = tokio::task::spawn(async move { add_random_user_to_db().await });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn add_a_bunch_of_users() {
        add_a_bunch_of_users_to_db(true, 20).await;
    }

    #[tokio::test]
    async fn get_list_of_books() {
        add_a_bunch_of_users_to_db(true, 20).await;
        let _book_list = User::get_list_of_users(None).await.unwrap();
    }
}
