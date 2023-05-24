use axum::{
    extract, response,
    routing::{get, post},
    Router,
};

use std::{net::SocketAddr, str::FromStr};

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

async fn make_user(extract::Path(username): extract::Path<String>) -> impl response::IntoResponse {}

async fn get_list_of_books() -> impl response::IntoResponse {}

async fn download_book(
    extract::Path(bookname): extract::Path<String>,
) -> impl response::IntoResponse {
}

async fn get_users_playback_locations(
    extract::Path(username): extract::Path<String>,
) -> impl response::IntoResponse {
}

async fn get_users_playback_location(
    extract::Path(username): extract::Path<String>,
    extract::Path(bookname): extract::Path<String>,
) -> impl response::IntoResponse {
}

async fn update_playback_location(
    extract::Path(username): extract::Path<String>,
    extract::Path(bookname): extract::Path<String>,
) -> impl response::IntoResponse {
}
