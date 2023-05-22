use sqlx::{postgres, ConnectOptions};

async fn conn() -> Result<postgres::PgConnection, sqlx::Error> {
    Ok(postgres::PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("my_user")
        .database("postgres")
        .password("password")
        .ssl_mode(postgres::PgSslMode::Disable)
        .connect()
        .await?)
}


pub async fn make_tables(passed_connection: Option<postgres::PgConnection>) -> Result<(), sqlx::Error> {
    let mut connection: postgres::PgConnection;

    if let Some(passed_connection) = passed_connection {
        connection = passed_connection;
    } else {
        connection = conn().await?;
    };

    let query_make_books_table = r#"
        -- Create "books" table
        CREATE TABLE IF NOT EXISTS books (
            id SERIAL PRIMARY KEY,
            name TEXT,
            length INTERVAL,
            file_location TEXT
        );
    "#;
        
    let query_make_users_table = r#"
        -- Create "users" table
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT
        );
    "#;
        
    let query_make_playback_locations_table = r#"
        -- Create "playback_locations" table
        CREATE TABLE IF NOT EXISTS playback_locations (
            id SERIAL PRIMARY KEY,
            book_id INT REFERENCES books (id),
            user_id INT REFERENCES users (id),
            time INTERVAL,
            CONSTRAINT duplicate_pair UNIQUE (book_id, user_id)
        );
    "#;
        
    let query_make_length_check_function = r#"
        -- Create a trigger function to enforce the time constraint
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
        -- Create a trigger on "playback_locations" to call the function
        CREATE TRIGGER playback_time_check
            BEFORE INSERT OR UPDATE ON playback_locations
            FOR EACH ROW
            EXECUTE FUNCTION check_playback_time();    
    "#;

    sqlx::query(&query_make_books_table).execute(&mut connection).await?;
    sqlx::query(&query_make_users_table).execute(&mut connection).await?;
    sqlx::query(&query_make_playback_locations_table).execute(&mut connection).await?;
    sqlx::query(&query_make_length_check_function).execute(&mut connection).await?;
    sqlx::query(&query_make_length_check_trigger).execute(&mut connection).await?;

    Ok(())
}