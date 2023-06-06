use crate::{
    book::Book,
    playback_location::{self, PlaybackLocation},
    user::User,
};
use axum::{
    extract, http,
    routing::{get, post},
    Router,
};

use std::{net::SocketAddr, str::FromStr};

mod api_response;
use api_response::ApiResponse;

fn app() -> Router {
    Router::new()
        .route("/users/:username", post(make_user))
        .route("/books", get(get_list_of_books))
        .route("/books/:bookname", get(download_book))
        .route("/playback/:username", get(get_users_playback_locations))
        .route(
            "/playback/:username/:bookname",
            get(get_users_playback_location),
        )
        .route("/playback", post(update_playback_location))
}

pub async fn start_server() {
    const SOCKET_ADDR: &str = "127.0.0.1:53135";

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = app();

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(SOCKET_ADDR).unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn make_user(extract::Path(username): extract::Path<String>) -> ApiResponse<String> {
    // Instantiate the new user
    let new_user = User::new(username);

    // Check list of users if it already exists
    if let Ok(existing_users) = User::get_list_of_users(None).await {
        if existing_users
            .iter()
            .any(|user| user.username == new_user.username)
        {
            return ApiResponse::Error(
                http::StatusCode::CONFLICT,
                String::from("username already exists, please choose another"),
            );
        }
    } else {
        return ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to get a list of users from the database"),
        );
    }

    // Add the new user
    if new_user.add_to_db().await.is_ok() {
        ApiResponse::Success(String::from("successfully added the new user"))
    } else {
        ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to add the new user to the database"),
        )
    }
}

async fn get_list_of_books() -> ApiResponse<Vec<Book>> {
    if let Ok(list_of_books) = Book::get_list_of_books(None).await {
        ApiResponse::Success(list_of_books)
    } else {
        ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to get a list of books from the database"),
        )
    }
}

async fn download_book(extract::Path(bookname): extract::Path<String>) -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}

async fn get_users_playback_locations(
    extract::Path(username): extract::Path<String>,
) -> ApiResponse<Vec<PlaybackLocation>> {
    if let Ok(playback_locations) = PlaybackLocation::get_users_playback_times(username) {
        ApiResponse::Success(playback_locations)
    } else {
        ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to get a list of playback locations for the given user"),
        )
    }
}

async fn get_users_playback_location(
    extract::Path(username): extract::Path<String>,
    extract::Path(bookname): extract::Path<String>,
) -> ApiResponse<PlaybackLocation> {
    if let Ok(playback_location) = PlaybackLocation::get_users_playback_time(username, bookname) {
        if let Some(playback_location) = playback_location {
            ApiResponse::Success(playback_location)
        } else {
            ApiResponse::Error(
                http::StatusCode::NOT_FOUND,
                String::from("No playback location found for this book/user"),
            )
        }
    } else {
        ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to get a list of playback locations for the given user"),
        )
    }
}

async fn update_playback_location(
    extract::Json(playback_location): extract::Json<PlaybackLocation>,
) -> ApiResponse<()> {
    if playback_location.upsert_to_db().await.is_ok() {
        ApiResponse::Success(())
    } else {
        ApiResponse::Error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            String::from("server failed to update the playback location for this book/user"),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::book;
    use crate::book::add_a_bunch_of_books_to_db;
    use crate::book::random_book;
    use crate::database::reset_tables;
    use crate::playback_location::add_a_bunch_of_playback_times_to_db;
    use crate::playback_location::PlaybackLocation;
    use crate::user;

    use super::*;
    use axum::body;
    use axum::response;
    use std::time;
    use tower::ServiceExt; // for `oneshot` and `ready`

    async fn add_user_named_tom(app: Router) -> response::Response {
        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri("/users/tom")
            .body(body::Body::from(""))
            .unwrap();
        app.oneshot(request).await.unwrap()
    }

    #[tokio::test]
    async fn duplicate_user() {
        reset_tables().await.unwrap();
        let app = app();
        let response_1 = add_user_named_tom(app.clone()).await;
        assert_eq!(response_1.status(), http::StatusCode::OK);
        let response_2 = add_user_named_tom(app.clone()).await;
        assert_eq!(response_2.status(), http::StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn get_book_list() {
        add_a_bunch_of_books_to_db(true, 10).await;
        let app = app();

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri("/books")
            .body(body::Body::from(""))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body_as_str = std::str::from_utf8(&response_body).unwrap();
        let response_body_deserialized =
            serde_json::from_str::<Vec<Book>>(response_body_as_str).unwrap();

        assert_eq!(response_body_deserialized.len(), 10);
    }

    #[tokio::test]
    async fn download_book() {
        reset_tables().await.unwrap();

        let mut book_path = std::env::current_dir().unwrap();
        // TODO add some public domain book to repo for testing this
        book_path.push("books");
        book_path.push("tress.m4b");
        println!("{:?}", book_path);
        let mut test_book = book::Book::new_from_path(book_path);
        test_book.name = String::from("tress");
        test_book.add_to_db().await.unwrap();

        let app = app();

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("/books/{}", test_book.name))
            .body(body::Body::from(""))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);

        // TODO save the bytes of the repsonse as a file on disk
    }

    #[tokio::test]
    async fn get_all_of_a_users_playback_locations() {
        add_a_bunch_of_playback_times_to_db(true, 5, 50).await;
        let users = user::User::get_list_of_users(None).await.unwrap();
        assert_eq!(users.len(), 5);
        let test_user = users.get(0).unwrap();

        let app = app();

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("/playback/{}", test_user.username))
            .body(body::Body::from(""))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body_as_str = std::str::from_utf8(&response_body).unwrap();
        let response_body_deserialized =
            serde_json::from_str::<Vec<PlaybackLocation>>(response_body_as_str).unwrap();

        println!(
            "get_all_of_a_users_playback_locations() response:\n{:?}\n",
            response_body_deserialized
        )

        // TODO further validation of the response; check length?
    }

    #[tokio::test]
    async fn get_a_users_playback_location_on_some_book() {
        // repeat get_all_of_a_users_playback_locations() so that we can get a valid
        // book/user pair
        add_a_bunch_of_playback_times_to_db(true, 5, 50).await;
        let users = user::User::get_list_of_users(None).await.unwrap();
        assert_eq!(users.len(), 5);
        let test_user = users.get(0).unwrap();

        let app1 = app();

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("/playback/{}", test_user.username))
            .body(body::Body::from(""))
            .unwrap();

        let response = app1.oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body_as_str = std::str::from_utf8(&response_body).unwrap();
        let response_body_deserialized =
            serde_json::from_str::<Vec<PlaybackLocation>>(response_body_as_str).unwrap();

        let test_playback_location = response_body_deserialized.get(0).unwrap();

        let request2 = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!(
                "/playback/{}/{}",
                test_playback_location.user_name, test_playback_location.book_name
            ))
            .body(body::Body::from(""))
            .unwrap();

        let app2 = app();
        let response2 = app2.oneshot(request2).await.unwrap();

        assert_eq!(response2.status(), http::StatusCode::OK);

        let response_body2 = hyper::body::to_bytes(response2.into_body()).await.unwrap();
        let response_body_as_str2 = std::str::from_utf8(&response_body2).unwrap();
        let response_body_deserialized2 =
            serde_json::from_str::<PlaybackLocation>(response_body_as_str2).unwrap();

        println!(
            "get_a_users_playback_location_on_some_book() response:\n{:?}\n",
            response_body_deserialized2
        )
    }

    #[tokio::test]
    async fn get_a_users_playback_location_on_unread_book() {
        add_a_bunch_of_playback_times_to_db(true, 5, 50).await;
        let app = app();
        add_user_named_tom(app.clone()).await;
        let test_book = random_book();
        test_book.add_to_db().await.unwrap();

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("/playback/{}/{}", "tom", test_book.name))
            .body(body::Body::from(""))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body_as_str = std::str::from_utf8(&response_body).unwrap();
        let response_body_deserialized =
            serde_json::from_str::<String>(response_body_as_str).unwrap();

        assert_eq!(
            response_body_deserialized,
            "No playback location found for this book/user"
        )
    }

    #[tokio::test]
    async fn update_playback_location_for_a_user() {
        reset_tables().await.unwrap();
        let app = app();
        add_user_named_tom(app.clone()).await;
        let test_book = random_book();
        test_book.add_to_db().await.unwrap();

        let test_playback_location1 = PlaybackLocation::new(
            (&test_book.name).to_owned(),
            String::from("tom"),
            time::Duration::from_millis(1),
        );

        let request1 = http::Request::builder()
            .method(http::Method::POST)
            .uri("/playback")
            .body(body::Body::from(
                serde_json::to_string(&test_playback_location1).unwrap(),
            ))
            .unwrap();

        let response1 = app.clone().oneshot(request1).await.unwrap();

        assert_eq!(http::StatusCode::OK, response1.status());

        let test_playback_location2 = PlaybackLocation::new(
            (&test_book.name).to_owned(),
            String::from("tom"),
            time::Duration::from_millis(2),
        );

        let request2 = http::Request::builder()
            .method(http::Method::POST)
            .uri("/playback")
            .body(body::Body::from(
                serde_json::to_string(&test_playback_location2).unwrap(),
            ))
            .unwrap();

        let response2 = app.clone().oneshot(request2).await.unwrap();

        assert_eq!(http::StatusCode::OK, response2.status());
    }
}
