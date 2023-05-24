use crate::user::User;
use axum::{
    extract, http, response,
    routing::{get, post},
    Router,
};

use std::{net::SocketAddr, str::FromStr};

mod api_response;
use api_response::ApiResponse;

pub async fn start_server() {
    const SOCKET_ADDR: &str = "127.0.0.1:53135";

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/users/:username", post(make_user))
        .route("/books", get(get_list_of_books))
        .route("/books/:bookname", get(download_book))
        .route("/playback/:username", get(get_users_playback_locations))
        .route(
            "/playback/:username/:bookname",
            get(get_users_playback_location),
        )
        .route(
            "/playback/:username/:bookname",
            post(update_playback_location),
        );

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

async fn get_list_of_books() -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}

async fn download_book(extract::Path(bookname): extract::Path<String>) -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}

async fn get_users_playback_locations(
    extract::Path(username): extract::Path<String>,
) -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}

async fn get_users_playback_location(
    extract::Path(username): extract::Path<String>,
    extract::Path(bookname): extract::Path<String>,
) -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}

async fn update_playback_location(
    extract::Path(username): extract::Path<String>,
    extract::Path(bookname): extract::Path<String>,
) -> ApiResponse<String> {
    ApiResponse::Error(
        http::StatusCode::NOT_IMPLEMENTED,
        String::from("this function is not yet implemented"),
    )
}
