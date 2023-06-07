use sqlx::{
    postgres::{self, types::PgInterval, PgDatabaseError},
    ConnectOptions,
};
use std::time;

pub async fn conn() -> Result<postgres::PgConnection, sqlx::Error> {
    postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("my_user")
        .database("postgres")
        .password("password")
        .ssl_mode(postgres::PgSslMode::Disable)
        .connect()
        .await
}

/// Make the tables, and do nothing if they already exists; always runs at the start
pub async fn make_tables() -> Result<(), sqlx::Error> {
    let mut conn = conn().await?;

    let query_make_books_table = r#"
        CREATE TABLE IF NOT EXISTS books (
            id SERIAL PRIMARY KEY,
            name TEXT,
            length INTERVAL,
            file_location TEXT
        );
    "#;

    let query_make_users_table = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT
        );
    "#;

    let query_make_playback_locations_table = r#"
        CREATE TABLE IF NOT EXISTS playback_locations (
            id SERIAL PRIMARY KEY,
            book_id INT REFERENCES books (id),
            user_id INT REFERENCES users (id),
            time INTERVAL,
            CONSTRAINT duplicate_pair UNIQUE (book_id, user_id)
        );
    "#;

    let query_make_length_check_function = r#"
        CREATE OR REPLACE FUNCTION check_playback_time()
        RETURNS TRIGGER AS $$
            BEGIN
                IF NEW.time > (SELECT length FROM books WHERE id = NEW.book_id) THEN
                    RAISE EXCEPTION 'Playback time exceeds the length of the corresponding book.';
                END IF;
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
    "#;

    let query_make_length_check_trigger = r#"
        CREATE OR REPLACE TRIGGER playback_time_check
            BEFORE INSERT OR UPDATE ON playback_locations
            FOR EACH ROW
            EXECUTE FUNCTION check_playback_time();    
    "#;

    sqlx::query(query_make_books_table)
        .execute(&mut conn)
        .await?;
    sqlx::query(query_make_users_table)
        .execute(&mut conn)
        .await?;
    sqlx::query(query_make_playback_locations_table)
        .execute(&mut conn)
        .await?;
    sqlx::query(query_make_length_check_function)
        .execute(&mut conn)
        .await?;
    sqlx::query(query_make_length_check_trigger)
        .execute(&mut conn)
        .await?;

    Ok(())
}

/// deletes the tables
pub async fn drop_tables() -> Result<(), sqlx::Error> {
    let mut conn = conn().await?;

    let query_drop_tables = "DROP TABLE books, users, playback_locations;";

    let query_drop_function = "DROP FUNCTION check_playback_time";

    if let Err(e) = sqlx::query(query_drop_tables).execute(&mut conn).await {
        let pg_error = e.into_database_error().unwrap();
        let pg_error2: &PgDatabaseError = pg_error.downcast_ref();
        let pg_error_code = pg_error2.code();
        if pg_error_code != "42P01" {
            //error code for tables not existing
            println!("{:?}", pg_error2);
            panic!(); // TODO return an Err() here, need to implement anyhow
        }
    };

    if let Err(e) = sqlx::query(query_drop_function).execute(&mut conn).await {
        let pg_error = e.into_database_error().unwrap();
        let pg_error2: &PgDatabaseError = pg_error.downcast_ref();
        let pg_error_code = pg_error2.code();
        if pg_error_code != "42883" {
            // error code for function not existing
            println!("{:?}", pg_error2);
            panic!(); // TODO return an Err() here, need to implement anyhow
        }
    };

    Ok(())
}

/// drops and then makes again the tables
pub async fn reset_tables() -> Result<(), sqlx::Error> {
    drop_tables().await?;
    make_tables().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_creation() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                make_tables().await.unwrap();
                drop_tables().await.unwrap();
                drop_tables().await.unwrap(); // drop_tables() shouldn't panic if the tables don't exist
                reset_tables().await.unwrap(); // reset_tables() shouldn't panic if tables don't exist
                make_tables().await.unwrap();
                reset_tables().await.unwrap(); // reset_tables() shouldn't panic if tables do exist
            });
    }
}

pub fn pg_interval_to_std_time_duration(interval: PgInterval) -> time::Duration {
    if interval.months != 0 {
        panic!("PgInterval with months != 0 not supported"); // TODO improve error handling here
    }
    time::Duration::from_micros(
        ((interval.days * 24 * 3600 * 1000000) as i64 + interval.microseconds) as u64,
    ) // PgInterval sucks!
}
