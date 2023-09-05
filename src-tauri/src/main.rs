// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{self, Manager};
use thiserror;
use tokio;
use tokio::sync::{Mutex, mpsc::{self, Sender, Receiver}};

mod book;
use book::Book;

type Result<T> = std::result::Result<T, CyprusError>;

#[derive(thiserror::Error, Debug)]
pub enum CyprusError {
    #[error("Placeholder error")]
    NotImplemented,
    #[error("Error while returning () to frontend: {0}")]
    SendError(#[from] mpsc::error::SendError<()>),
    #[error("SystemTime failed: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("mp4 crate error: {0}")]
    Mp4(#[from] mp4::Error),
    #[error("Error while finding MP4 chapters")]
    Mp4Chapters,
    #[error("Lofty error: {0}")]
    Lofty(#[from] lofty::LoftyError),
    #[error("Item had no primary tag")]
    LoftyPrimaryTag,
    #[error("Passed a path which wasn't a directory")]
    NotDirectory,
    #[error("Passed a path which wasn't a file")]
    NotFile,
    #[error("Passed a path which is a folder with no mp3s")]
    EmptyFolder,
    #[error("Tried to find the parent of a PathBuf with no parent")]
    NoParent,
    #[error("Passed a folder with files containing mismatched metadata")]
    MixedFilesInFolder,
}

impl Serialize for CyprusError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

async fn get_books_loop(
    mut input_rx: Receiver<()>,
    output_tx: Sender<Vec<Book>>,
    books: Arc<Mutex<Vec<Book>>>,
) -> Result<()> {
    loop {
        if input_rx.recv().await.is_some() {
            println!("Gathering books...");
            let mut books = books.lock().await;
            *books = vec![];
            books.push(Book::from_file_path("../static/books/Tress_of_the_Emerald_Sea_by_Brandon_Sanderson.m4b")?);
            books.push(Book::from_folder_path("../static/books/amateur_1206_librivox")?);
            output_tx.send((*books).clone()).await;
        }
    }
}

async fn emit_coffee_loop(
    output_tx: Sender<()>,
    flow_of_coffee: Arc<AtomicBool>,
    last_coffee_granted_at: Arc<Mutex<SystemTime>>,
) -> Result<()> {
    loop {
        let mut last_coffee_granted_at = last_coffee_granted_at.lock().await;
        if flow_of_coffee.load(Ordering::Relaxed)
            && last_coffee_granted_at.elapsed()? > Duration::new(2, 0)
        {
            println!("Time to send a coffee!");
            *last_coffee_granted_at = SystemTime::now();
            output_tx.send(()).await?;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tauri::command]
async fn get_books(
    message: (),
    state: tauri::State<'_, Mutex<Sender<()>>>,
) -> Result <()> {
    println!("Clicking Fetch Books got to Rust");
    let proc_input_tx = state.inner().lock().await;
    Ok(proc_input_tx.send(()).await?)
}

fn send_books<R: tauri::Runtime>(message: Vec<Book>, manager: &impl Manager<R>) {
    // manager.emit_all("new-books", serde_json::to_string(&message).unwrap()).unwrap();
    println!("Emitting message: {:?}", message);
    manager.emit_all("new-books", message).unwrap();
}

type Dummy = ();

#[tokio::main]
async fn main() {
    // We'll own the runtime
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let (input_tx_book, input_rx_book) = mpsc::channel::<()>(1);
    let (output_tx_book, mut output_rx_book) = mpsc::channel::<Vec<Book>>(1);

    let books = Arc::new(Mutex::new(vec![]));
    let books_loop = books.clone();

    tokio::spawn(async move {
        get_books_loop(input_rx_book, output_tx_book, books_loop).await;
    });

    // let input_tx = sync::Mutex::new(input_tx);

    tauri::Builder::default()
        .manage(Mutex::<Sender<Dummy>>::new(input_tx_book))
        .invoke_handler(tauri::generate_handler![get_books])
        .setup(|app| {
            let app_handle = app.handle();

            tokio::spawn(async move {
                loop {
                    if let Some(output) = output_rx_book.recv().await {
                        send_books(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
