use crate::database::conn;
/// Module for the `book` struct. Any `book` is associated with 1 row in the `books` table of the database. SQL queries related to `book`s are in `books_queries.rs`
use serde::{Deserialize, Serialize};
use std::{path, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    name: String,
    length: time::Duration,
    file_location: path::PathBuf,
}

impl Book {
    fn new(name: String, length: time::Duration, file_location: path::PathBuf) -> Self {
        Self {
            name,
            length,
            file_location,
        }
    }

    fn new_from_path(file_location: path::PathBuf) -> Self {
        // TODO parse book name, length from file
        Self {
            name: String::from("name_parsing_not_yet_implemented"),
            length: time::Duration::from_secs(42),
            file_location,
        }
    }

    async fn add_to_db(&self) -> Result<(), sqlx::Error> {
        let mut conn = conn().await?;

        sqlx::query("INSERT INTO books (name, length, file_location) VALUES ($1, $2, $3)")
            .bind(&self.name)
            .bind(&self.length)
            .bind(self.file_location.to_str().unwrap())
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::reset_tables;
    use rand::Rng;

    fn random_book() -> Book {
        let random_vec_u8 = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(10)
            .collect::<Vec<u8>>();
        let random_string = String::from_utf8(random_vec_u8).unwrap();
        let random_u16: u16 = rand::thread_rng().gen();
        let random_path = std::env::temp_dir().join(&random_string);

        Book {
            name: random_string,
            length: time::Duration::from_secs(random_u16 as u64),
            file_location: random_path,
        }
    }

    #[test]
    fn test_add_book() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                reset_tables().await.unwrap();
                for _ in 0..500 {
                    let test_book = random_book();
                    test_book.add_to_db().await.unwrap();
                }
            });
    }
}
