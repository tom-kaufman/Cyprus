use axum::{
    body::StreamBody,
    extract,
    http::{
        header::{self, HeaderMap, HeaderName},
        StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio_util::io::ReaderStream;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod database;
use database::make_tables;
mod book;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    //let app = Router::new();

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // let addr = SocketAddr::from_str("127.0.0.1:32123").unwrap();
    // tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
