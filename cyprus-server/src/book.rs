use crate::database::conn;
/// Module for the `book` struct. Any `book` is associated with 1 row in the `books` table of the database. SQL queries related to `book`s are in `books_queries.rs`
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{types::PgInterval, PgRow},
    FromRow, Row,
};
use std::{path, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    name: String,
    length: time::Duration,
    file_location: path::PathBuf,
}

fn pg_interval_to_std_time_duration(interval: PgInterval) -> time::Duration {
    if interval.months != 0 {
        panic!("PgInterval with months != 0 not supported"); // TODO improve error handling here
    }
    time::Duration::from_micros(
        ((interval.days * 24 * 3600 * 1000000) as i64 + interval.microseconds) as u64,
    ) // PgInterval sucks!
}

impl FromRow<'_, PgRow> for Book {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let name = row.get::<String, usize>(0);

        let length_from_row = row.get(1);
        let length = pg_interval_to_std_time_duration(length_from_row);

        let file_location_from_row = row.get::<&str, usize>(2);
        let file_location = path::PathBuf::from(file_location_from_row);

        Ok(Book {
            name,
            length,
            file_location,
        })
    }
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

    async fn get_list_of_books(limit: Option<i64>) -> Result<Vec<Book>, sqlx::Error> {
        let mut lim = limit.unwrap_or_else(|| i64::MAX);
        if lim < 0 {
            lim = i64::MAX;
        }

        let mut conn = conn().await?;

        Ok(
            sqlx::query_as::<_, Book>("SELECT name, length, file_location FROM books LIMIT ($1)")
                .bind(&lim)
                .fetch_all(&mut conn)
                .await?,
        )
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
            length: time::Duration::from_micros(random_u16 as u64),
            file_location: random_path,
        }
    }

    async fn add_random_book_to_db() {
        let test_book = random_book();
        let serialized = serde_json::to_string(&test_book).expect("Failed to serialize");
        println!("add_random_book_to_db(): {}", serialized);
        test_book.add_to_db().await.unwrap();
    }

    async fn add_a_bunch_of_books_to_db() {
        reset_tables().await.unwrap();

        let mut handles = Vec::new();

        for i in 0..10 {
            let handle = tokio::task::spawn(async move { add_random_book_to_db().await });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn add_a_bunch_of_random_books() {
        add_a_bunch_of_books_to_db().await;
    }

    #[tokio::test]
    async fn get_list_of_books() {
        add_a_bunch_of_books_to_db().await;
        let book_list = Book::get_list_of_books(None).await.unwrap();
        println!("{:?}", book_list);
        for book in book_list.iter() {
            let serialized = serde_json::to_string(book).expect("Failed to serialize");
            println!("get_list_of_books(): {}", serialized);
        }
    }
}
