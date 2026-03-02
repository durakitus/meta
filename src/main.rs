use anyhow::Result;
use clap::Parser;
use lofty::config::WriteOptions;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::{Accessor, ItemKey, Tag, TagExt};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "meta",
    version,
    about = "This is a streamlined command line tool designed to read and update the metadata of your audio files with ease.",
    long_about = "You can use this utility to manage the internal information of your music collection by viewing or modifying tags like titles and artists. It is built to handle multiple files at once and can even pull lyrics directly from a text file to embed them into your tracks. The tool automatically detects the best way to store this information based on the specific format of each audio file to ensure your metadata remains consistent and accurate."
)]
struct Args {
    /// title
    #[arg(short, long)]
    title: Option<String>,

    /// artist
    #[arg(short, long)]
    artist: Option<String>,

    /// album
    #[arg(short, long)]
    disk: Option<String>,

    /// track number
    #[arg(short, long)]
    number: Option<String>,

    /// release year
    #[arg(short, long)]
    year: Option<String>,

    /// genre
    #[arg(short, long)]
    genre: Option<String>,

    /// lyrics text or path to a text file
    #[arg(short, long)]
    lyrics: Option<String>,

    /// audio files to process
    files: Vec<PathBuf>,
}

fn main() {
    let arguments = Args::parse();

    if arguments.files.is_empty() {
        println!("No files provided.");
        return;
    }

    let options_provided = arguments.title.is_some()
        || arguments.artist.is_some()
        || arguments.disk.is_some()
        || arguments.number.is_some()
        || arguments.year.is_some()
        || arguments.genre.is_some()
        || arguments.lyrics.is_some();

    for file_path in &arguments.files {
        if let Err(error) = process_file(file_path, &arguments, options_provided) {
            eprintln!("Error processing {:?}: {}", file_path, error);
        }
    }
}

fn process_file(path: &Path, arguments: &Args, update_mode: bool) -> Result<()> {
    let mut tagged_file = Probe::open(path)?.guess_file_type()?.read()?;

    let tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag,
        None => {
            if let Some(first_tag) = tagged_file.first_tag_mut() {
                first_tag
            } else {
                let tag_type = tagged_file.primary_tag_type();
                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        }
    };

    if update_mode {
        update_metadata(tag, arguments)?;
        tag.save_to_path(path, WriteOptions::default())?;
        println!("Updated: {:?}", path);
    } else {
        display_metadata(tag, path);
    }

    Ok(())
}

fn update_metadata(tag: &mut Tag, arguments: &Args) -> Result<()> {
    if let Some(ref title_text) = arguments.title {
        tag.set_title(title_text.clone());
    }
    if let Some(ref artist_name) = arguments.artist {
        tag.set_artist(artist_name.clone());
    }
    if let Some(ref disk_name) = arguments.disk {
        tag.set_album(disk_name.clone());
    }
    if let Some(ref genre_name) = arguments.genre {
        tag.set_genre(genre_name.clone());
    }

    if let Some(ref year_string) = arguments.year {
        tag.insert_text(ItemKey::Year, year_string.clone());
    }

    if let Some(ref number_string) = arguments.number {
        tag.insert_text(ItemKey::TrackNumber, number_string.clone());
    }

    if let Some(ref lyrics_input) = arguments.lyrics {
        let lyrics_content = if Path::new(lyrics_input).exists() {
            fs::read_to_string(lyrics_input)?
        } else {
            lyrics_input.clone()
        };
        tag.insert_text(ItemKey::Lyrics, lyrics_content);
    }

    Ok(())
}

fn display_metadata(tag: &Tag, path: &Path) {
    println!("File: {:?}", path);
    println!("TITLE: {}", tag.title().as_deref().unwrap_or(""));
    println!("ARTIST: {}", tag.artist().as_deref().unwrap_or(""));
    println!("DISK: {}", tag.album().as_deref().unwrap_or(""));

    let year_display = tag.get_string(ItemKey::Year).unwrap_or("");
    println!("YEAR: {}", year_display);

    println!("GENRE: {}", tag.genre().as_deref().unwrap_or(""));

    if let Some(lyrics_data) = tag.get_string(ItemKey::Lyrics) {
        println!("LYRICS: Found, {} characters", lyrics_data.len());
    }
    println!("---");
}
