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
use std::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Default)]
struct Audiobook {
    file_name: String,
    #[serde(skip)]
    file_path: PathBuf,
}

impl Audiobook {
    fn new(file_path: PathBuf) -> Self {
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        Self {
            file_name,
            file_path,
        }
    }
}

#[derive(Serialize, Default)]
struct AudiobookList {
    #[serde(flatten)]
    books: HashMap<u32, Audiobook>,
}

impl AudiobookList {
    fn from_directory(path: &Path) -> Self {
        let mut books = Self::default();
        let mut id = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(file_name) = entry.file_name().to_str() {
                                if file_name.to_lowercase().ends_with(".m4b") {
                                    let book = Audiobook::new(entry.path());
                                    books.books.insert(id, book);
                                    id += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        books
    }
}

async fn list_audiobooks() -> Json<AudiobookList> {
    tracing::info!("retrieving the audiobook list...");
    Json(AudiobookList::from_directory(Path::new(dotenv!(
        "BOOKS_DIRECTORY"
    ))))
}

// reference: https://github.com/tokio-rs/axum/discussions/608
async fn return_audiobook(extract::Path(audiobook_id): extract::Path<u32>) -> impl IntoResponse {
    tracing::debug!("Request book {}", audiobook_id);

    let books =  AudiobookList::from_directory(Path::new(dotenv!(
        "BOOKS_DIRECTORY"
    )));
    let book = books.books.get(&audiobook_id).unwrap();
    let file = tokio::fs::File::open(&book.file_path).await.unwrap();

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "audio/mp4".parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", book.file_name).parse().unwrap());
    
    tracing::info!("Headers:\n{:?}", headers);

    (headers, body)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/audiobooks", get(list_audiobooks))
        .route("/audiobooks/:audiobook_id", get(return_audiobook));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(dotenv!("SOCKET_ADDR")).unwrap();
    println!("1");
    tracing::debug!("listening on {}", addr);
    println!("2");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
