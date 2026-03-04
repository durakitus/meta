# Meta

> A straightforward CLI utility for audio metadata edition.

## Overview
Meta is a simple, easy to use CLI tool that allows quick edition of metadata values in audio files. It supports MP3 and Ogg Opus formats, and can modify the most essential metadata so you can keep your personal audio library organized very quickly.

## Features
- **Magic Bytes Check:** It relies on file signature verification to identify the file type. This means it can tell the audio codec apart even in ambiguous container formats that could have several codecs.
- **Batch Support:** The tool supports batch edition of audio files through typical CLI methods such as loops or wildcards.
- **Lyrics:** Additionally, it supports embedding lyrics directly into the audio. It can accept the text from standard command input — via command output capture — or read it from a text file.

## Build
To build it from source, clone the repo:

```
git clone https://github.com/durakitus/meta.git
cd meta
cargo build --release
```

## Usage
Some example commands could be:

`meta -t <title> -a <artist> -n <track_number> <file>` — adding/editing some basic metadata.
`meta -a <artist> -d <album> *` — batch edition to organize a collection by album and artist.
`meta -l <lyrics_file> <audio_file>` — adding lyrics to an audio file from a given text file.

Run `meta -h` — or use `cargo run -- -h` inside the project folder if it's not in your `PATH` — for more usage details, if you decide to build it.
