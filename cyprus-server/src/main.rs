use axum::{
    http::{StatusCode, header::{self, HeaderMap, HeaderName}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, extract,
    body::StreamBody,
};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use serde::Serialize;
use std::collections::HashMap;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio_util::io::ReaderStream;

#[derive(Serialize, Default)]
struct Audiobook {
    filename: String,
    #[serde(skip)]
    file_path: PathBuf,
    #[serde(skip)]
    id: u32,
}

#[derive(Serialize, Default)]
struct AudiobookList {
    books: HashMap<u32, Audiobook>,
}

impl AudiobookList {
    fn from_directory(path: &Path) -> Self {
        Self::default()
    }
}

async fn list_audiobooks() -> Json<AudiobookList> {
    Json(AudiobookList::from_directory(Path::new(dotenv!(
        "BOOKS_DIRECTORY"
    ))))
}

async fn return_audiobook(extract::Path(audiobook_id): extract::Path<u32>) -> impl IntoResponse {
    let books =  AudiobookList::from_directory(Path::new(dotenv!(
        "BOOKS_DIRECTORY"
    )));
    let book = books.books.get(&audiobook_id).unwrap();
    let file = tokio::fs::File::open("Cargo.toml").await.unwrap();

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "audio/mp4".parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", book.filename).parse().unwrap());
    
    (headers, body)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/audiobooks", get(list_audiobooks))
        .route("/aduiobooks/:audiobook_id", get(return_audiobook));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(dotenv!("SOCKET_ADDR")).unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
