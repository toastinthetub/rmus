use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct SongMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub path: PathBuf,
}

impl SongMetadata {
    pub fn from_path(path: PathBuf) -> Self {
        let title = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string());
        let artist = path.parent().and_then(|p| p.file_name()).and_then(|a| a.to_str()).map(|s| s.to_string());
        let album = path.parent().and_then(|p| p.parent()).and_then(|a| a.file_name()).and_then(|a| a.to_str()).map(|s| s.to_string());

        SongMetadata {
            title,
            artist,
            album,
            path,
        }
    }
}

pub fn find_music_files(dir: &Path) -> Result<Vec<SongMetadata>, io::Error> {
    let mut music_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            music_files.extend(find_music_files(&path)?);
        } else if let Some(extension) = path.extension() {
            if extension == "mp3" || extension == "wav" || extension == "flac" {
                music_files.push(SongMetadata::from_path(path));
            }
        }
    }

    Ok(music_files)
} 

pub fn get_artists_albums_songs(dir: &Path) -> Result<(Vec<String>, Vec<String>, Vec<String>), io::Error> {
    let mut artists = Vec::new();
    let mut albums = Vec::new();
    let mut songs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let artist = path.file_name().and_then(|a| a.to_str()).map(|a| a.to_string()).unwrap_or_default();
            artists.push(artist.clone());

            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let album_path = entry.path();
                if album_path.is_dir() {
                    let album = album_path.file_name().and_then(|a| a.to_str()).map(|a| a.to_string()).unwrap_or_default();
                    albums.push(format!("{} - {}", artist, album));

                    let album_songs: Vec<String> = fs::read_dir(&album_path)?
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.path())
                        .filter(|path| path.is_file())
                        .filter_map(|path| path.file_name().and_then(|name| name.to_str()).map(|name| name.to_string()))
                        .collect();

                    songs.extend(album_songs);
                }
            }
        }
    }
    Ok((artists, albums, songs))
}