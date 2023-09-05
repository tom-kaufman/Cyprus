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
use tokio::sync::{self, mpsc};

mod book;

type Result<T> = std::result::Result<T, CyprusError>;

#[derive(thiserror::Error, Debug)]
enum CyprusError {
    #[error("Placeholder errror")]
    NotImplemented,
    #[error("Error while returning () to frontend")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<()>),
    #[error("SystemTime failed")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("mp4 crate error")]
    Mp4(#[from] mp4::Error),
    #[error("Error while finding MP4 chapters")]
    Mp4Chapters,
    #[error("Lofty error")]
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

async fn toggle_coffee_loop(
    mut input_rx: mpsc::Receiver<()>,
    flow_of_coffee: Arc<AtomicBool>,
) -> Result<()> {
    loop {
        if input_rx.recv().await.is_some() {
            println!("Toggling coffee..");
            let initial_value = flow_of_coffee.load(Ordering::Relaxed);
            flow_of_coffee.store(!initial_value, Ordering::Relaxed);
        }
    }
}

async fn emit_coffee_loop(
    output_tx: mpsc::Sender<()>,
    flow_of_coffee: Arc<AtomicBool>,
    last_coffee_granted_at: Arc<sync::Mutex<SystemTime>>,
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
async fn toggle_coffees(
    message: (),
    state: tauri::State<'_, sync::Mutex<mpsc::Sender<()>>>,
) -> Result<()> {
    println!("Clicking toggle_coffees button got to rust!");
    let proc_input_tx = state.inner().lock().await;
    Ok(proc_input_tx.send(()).await?)
}

fn send_coffee<R: tauri::Runtime>(message: (), manager: &impl Manager<R>) {
    // TODO emit_to?
    manager.emit_all("coffee-gained", ()).unwrap();
}

#[tokio::main]
async fn main() {
    // We'll own the runtime
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let (input_tx, input_rx) = mpsc::channel::<()>(1);
    let (output_tx, mut output_rx) = mpsc::channel::<()>(1);

    // No way the state should live here? I guess this would go in some struct in async land
    let flow_of_coffee = Arc::new(AtomicBool::new(false));
    let last_coffee_granted_at = Arc::new(sync::Mutex::new(SystemTime::now()));

    let flow_of_coffee_loop = flow_of_coffee.clone();
    let flow_of_coffee_emit = flow_of_coffee.clone();

    tokio::spawn(async move { toggle_coffee_loop(input_rx, flow_of_coffee_loop).await });
    tokio::spawn(async move {
        emit_coffee_loop(
            output_tx,
            flow_of_coffee_emit,
            last_coffee_granted_at.clone(),
        )
        .await
    });

    // let input_tx = sync::Mutex::new(input_tx);

    tauri::Builder::default()
        .manage(sync::Mutex::<mpsc::Sender<()>>::new(input_tx))
        .invoke_handler(tauri::generate_handler![toggle_coffees])
        .setup(|app| {
            let app_handle = app.handle();
            tokio::spawn(async move {
                loop {
                    if let Some(output) = output_rx.recv().await {
                        send_coffee(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
