use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use lofty::{AudioFile, TaggedFileExt, PictureType};
use serde::Serialize;

use crate::{CyprusError, Result};

#[derive(Debug, Serialize, Clone)]
pub struct Book {
    name: String,
    author: String,
    duration: Duration,
    cover_art_path: Option<PathBuf>,
    files: Vec<BookFile>,
}

#[derive(Debug, Serialize, Clone)]
struct BookFile {
    chapters: Vec<Chapter>,
    path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct Chapter {
    title: String,
    duration: Duration,
}

#[derive(Debug)]
struct ChapterBuilder {
    // Later, if this is None, we'll replace it with the chapter number
    title: Option<String>,
    duration: Duration,
}

// Consume vec of ChapterBuilder to produce a vec of Chapter
fn build_chapters(chapter_builders: Vec<ChapterBuilder>) -> Vec<Chapter> {
    chapter_builders
        .iter()
        .enumerate()
        .map(|(idx, builder)| {
            let title = match &builder.title {
                Some(title) => title.to_owned(),
                None => format!("Chapter {}", idx + 1),
            };
            Chapter {
                title,
                duration: builder.duration,
            }
        })
        .collect()
}

// Sum up the chapter durations to recalculate total duration
fn calculate_duration(chapters: &[Chapter]) -> Duration {
    chapters.iter().map(|ch| ch.duration).sum()
}

// given the lofty Tag, parse the chapter title
fn get_chapter_title(tag: &lofty::Tag) -> String {
    tag
        .get_string(&lofty::ItemKey::TrackTitle)
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackTitleSortOrder))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackSubtitle))
        .unwrap_or("Unnamed Chapter")
        .into()
}

// given the lofty Tag, parse the author
fn get_author(tag: &lofty::Tag) -> String {    
    tag
        .get_string(&lofty::ItemKey::AlbumArtist)
        .or_else(|| tag.get_string(&lofty::ItemKey::AlbumArtistSortOrder))
        .or_else(|| tag.get_string(&lofty::ItemKey::OriginalArtist))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackArtist))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackArtistSortOrder))
        .unwrap_or("Uncredited Author")
        .into()
}

// given the lofty Tag, parse the book name
fn get_book_name(tag: &lofty::Tag) -> String {
    tag
        .get_string(&lofty::ItemKey::AlbumTitle)
        .or_else(|| tag.get_string(&lofty::ItemKey::AlbumTitleSortOrder))
        .or_else(|| tag.get_string(&lofty::ItemKey::OriginalAlbumTitle))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackTitle))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackTitleSortOrder))
        .or_else(|| tag.get_string(&lofty::ItemKey::TrackSubtitle))
        .unwrap_or("Untitled Book")
        .into()
}


fn cover_art_path(name: &str, book_file_path: &PathBuf) -> Result<PathBuf> {
    if !book_file_path.is_file() {
        return Err(CyprusError::NotFile);
    }
    Ok(match book_file_path.parent() {
        Some(dir) => {
            let mut p = PathBuf::from(dir);
            p.push(format!("cover_{}.jpg", name));
            p
        },
        None => { return Err(CyprusError::NoParent) },
    })
}

// Embedded cover art will get passed to the frontend by file path
// Check if we already extracted the cover art
fn locate_extracted_cover_art(name: &str, book_file_path: &PathBuf) -> Result<Option<PathBuf>> {
    let cover_art_path = cover_art_path(name, book_file_path)?;

    if cover_art_path.exists() {
        Ok(Some(cover_art_path))
    } else {
        Ok(None)
    }
}

// Try to extract the cover art and save it at the book's path, using cover_{name}.jpg as filename
fn extract_cover_art(tag: &lofty::Tag, name: &str, book_file_path: &PathBuf) -> Result<Option<PathBuf>> {
    let cover_art_path = cover_art_path(name, book_file_path)?;

    // First try some specific tags, fall back to taking what we can get
    let pic = tag
        .get_picture_type(PictureType::CoverFront)
        .or_else(|| tag.get_picture_type(PictureType::Other))
        .or_else(|| tag.get_picture_type(PictureType::Icon))
        .or_else(|| tag.get_picture_type(PictureType::Illustration))
        .or_else(|| {
            if tag.picture_count() > 0 {
                Some(&tag.pictures()[0])
            } else {
                None
            }
        });
    
    match pic {
        Some(pic) => {
            let mut file = match OpenOptions::new().create(true).write(true).open(&cover_art_path) {
                Ok(file) => file,
                Err(_) => {
                    println!("Error while opening a file to write cover art to, falling back to no cover art");
                    return Ok(None);
                }
            };
            match file.write_all(pic.data()) {
                Ok(_) => Ok(Some(cover_art_path)),
                Err(_) => {
                    println!("Error while writing cover art bytes to file, falling back to no cover art");
                    Ok(None)
                }
            }
        },
        None => Ok(None)
    }
}

impl Book {
    // flatten self.files into a Vec<&Chapter>
    fn chapters(&self) -> Vec<Chapter> {
        self.files.iter().map(|f| f.chapters.clone()).flatten().collect::<Vec<Chapter>>()
    }

    // Accept a path to a folder containing several files, each with a chapter
    // Intended for use with a collection of mp3s
    // Chapters will be sorted by file name
    pub fn from_folder_path<P: AsRef<Path>>(folder_path: P) -> Result<Self> {
        // Make sure the path passed is a dir
        let path = folder_path.as_ref().to_path_buf();
        if !path.is_dir() {
            return Err(CyprusError::NotDirectory);
        }

        // Get all of the mp3 files in this folder (not recursive)
        let mut file_paths = vec![];
        for entry in std::fs::read_dir(&path)? {
            let file_path = match entry {
                Ok(entry) => entry.path(),
                Err(_) => {
                    continue;
                }
            };

            if !file_path.is_file() {
                continue;
            }

            match file_path.extension() {
                Some(ext) => {
                    if ext != "mp3" {
                        continue;
                    }
                }
                None => {
                    continue;
                }
            };

            file_paths.push(file_path);
        }

        if file_paths.is_empty() {
            return Err(CyprusError::EmptyFolder);
        }

        // If there is only one file, collapse to from_file_path()
        if file_paths.len() == 1 {
            return Self::from_file_path(&file_paths[0]);
        }

        // Start by creating a Book from the first file
        let mut result = Self::from_file_path(&file_paths[0])?;

        // Now accumulate the remaining files into result
        for (idx, file_path) in file_paths.iter().enumerate() {
            // Skip the first item
            if idx == 0 {
                continue;
            }

            // Compare subsequent items with the first item
            let new_book = Self::from_file_path(file_path)?;
            // We don't want to mix chapter mp3s from several audio books into one, validate
            // by comparing the name and author
            if !((new_book.author == result.author) && (new_book.name == result.name)) {
                return Err(CyprusError::MixedFilesInFolder);
            }

            result.files.extend(new_book.files);
        }

        // At this point, the duration only reflects the first file. Recalculate it. 
        result.duration = calculate_duration(result.chapters().as_slice());

        Ok(result)
    }

    // Accept a path to a single file to construct a Book
    // Intended to be used for m4b files, which have all the chapters in one file
    fn from_file_path_mp4<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let path = file_path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        // lofty crate setup
        let tagged_file = lofty::read_from_path(&path)?;
        let tag = match tagged_file.primary_tag() {
            Some(tag) => tag,
            None => return Err(CyprusError::LoftyPrimaryTag),
        };
        // mp4 crate setup
        let file_size = file.metadata()?.len();
        let buf_reader = BufReader::new(file);
        let mut mp4_reader = mp4::Mp4Reader::read_header(buf_reader, file_size)?;

        // Find the first track where the media type gives an error
        // This track is a container for the metadata
        let track = match mp4_reader
            .tracks()
            .values()
            .find(|t| t.media_type().is_err())
        {
            Some(track) => track,
            None => return Err(CyprusError::Mp4Chapters),
        };
        let track_id = track.track_id();

        // Iterate on each sample in the metadata track
        let mut chapter_builders = vec![];
        for i in 1..=track.sample_count() {
            let sample = match mp4_reader.read_sample(track_id, i)? {
                Some(sample) => sample,
                None => return Err(CyprusError::Mp4Chapters),
            };

            // If the sample duration isn't provided we should fall back to using lofty
            let duration = match sample.duration {
                0 => tagged_file.properties().duration() - Duration::from_millis(sample.start_time),
                ms => Duration::from_millis(ms.into()),
            };

            // Parse the chapter title from bytes
            let title = if sample.bytes.len() > 0 {
                // Not sure if these offsets will apply to all books.
                let b = sample.bytes.slice(2..sample.bytes.len() - 12);
                match std::str::from_utf8(&b) {
                    Ok(s) => Some(s.into()),
                    Err(_) => {
                        println!(" ~ Warning: Indexing & decoding as utf8 sample.bytes for chapter title failed");
                        None
                    }
                }
            } else {
                // We don't want an empty chapter title, in the case bytes is empty
                None
            };

            chapter_builders.push(ChapterBuilder { title, duration })
        }

        let chapters = build_chapters(chapter_builders);
        let duration = calculate_duration(chapters.as_slice()); 
        let name = get_book_name(&tag);
        let author = get_author(&tag);

        // We'll pass the image to the frontend by file path.
        // Embedded cover art will get saved as cover_{Book.name}.jpg, check for it first
        // If not present, try to extract it
        // If no image to extract, leave as None
        let cover_art_path = match locate_extracted_cover_art(&name, &path)? {
            Some(cover_art_path) => Some(cover_art_path),
            None => extract_cover_art(&tag, &name, &path)?,
        };

        let book_file = BookFile { chapters, path };

        Ok(Self {
            name,
            author,
            duration,
            files: vec![book_file],
            cover_art_path,
        })
    }

    fn from_file_path_mp3<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let path = file_path.as_ref().to_path_buf();
        let tagged_file = lofty::read_from_path(&path)?;
        let tag = match tagged_file.primary_tag() {
            Some(tag) => tag,
            None => return Err(CyprusError::LoftyPrimaryTag),
        };

        let name = get_book_name(&tag);
        let author = get_author(&tag);
        let chapter_title = get_chapter_title(&tag);
        let duration = tagged_file.properties().duration();

        let book_file = BookFile {
            chapters: vec![Chapter {
                title: chapter_title,
                duration,
            }],
            path,
        };

        Ok(Self {
            name,
            author,
            duration,
            files: vec![book_file],
            cover_art_path: None, // TODO
        })
    }

    pub fn from_file_path<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        match Self::from_file_path_mp4(&file_path) {
            Ok(result) => Ok(result),
            Err(error) => {
                println!(
                    "Failed to parse as MP4 (Error: {}), falling back to parsing as MP3",
                    error
                );
                Self::from_file_path_mp3(&file_path)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initialize_book_from_file_m4b() {
        println!("I haven't gotten my hands on a public domain m4b with chapters quite yet, this test will fail.");
        let book = Book::from_file_path(
            "../static/books/Tress_of_the_Emerald_Sea_by_Brandon_Sanderson.m4b",
        )
        .unwrap();
        println!("{:?}", book);
    }

    #[test]
    fn initialize_book_from_file_mp3() {
        let book =
            Book::from_file_path("../static/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3")
                .unwrap();
        println!("{:?}", book);
    }

    #[test]
    fn initialize_book_from_folder() {
        let book = Book::from_folder_path("../static/books/amateur_1206_librivox").unwrap();
        println!("{:?}", book);
    }

    #[test]
    fn scratch() -> Result<()> {
        Ok(())
    }
}
