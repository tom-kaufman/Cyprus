use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use anyhow::Result;
use thiserror::Error;

#[derive(Default)]
struct AudioBook {
    file_path: PathBuf,
    media_information: MediaInformation,
    metadata: HashMap<String, String>,
    streams: Vec<Stream>,
}

#[derive(Default)]
struct MediaInformation {
    title: String,
    artist: String,
    album: String,
    genre: String,
    now_playing: String,
    publisher: String,
    copyright: String,
    encoded_by: String,
    comments: String,
    date: String,
    track_number: String,
    track_number_out_of: String,
    language: String,
    album_art: String,
}

struct Stream {
    codec: Codec,
    ty: String,
    channels: Option<String>,
    sample_rate: Option<i32>,
    bits_per_sample: Option<i32>,
}

enum Codec {
    Mp4a,
    Tx3g,
}

#[derive(Error, Debug)]
enum DecodeError  {
    #[error("Error during file io")]
    Io(#[from] io::Error),
    #[error("Expected first 8 bytes to be 'ftyp'")]
    NoFtyp,

}


fn decode_mp4_from_path(path: &Path) -> Result<AudioBook, DecodeError> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut ftyp_buffer = [0; 8];
    let expected_ftyp: [u8; 8] = [0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70];
    reader.read_exact(&mut ftyp_buffer)?;

    if ftyp_buffer != expected_ftyp {
        return Err(DecodeError::NoFtyp);
    }

    Ok(AudioBook::default())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = "/home/tom/rust/Cyprus/cyprus-server/tress.m4b".to_owned();
    let path = args.get(1).unwrap_or_else(|| {
        println!("No path provided in args, defaulting to {default_path}");
        &default_path
    });

    let my_mp4 = decode_mp4_from_path(Path::new(path));

    if my_mp4.is_ok() {
        println!("SUCCESS")
    } else {
        println!("FAIL")
    }
}
