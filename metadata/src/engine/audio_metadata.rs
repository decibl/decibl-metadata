/*
make absract class
make audio metadata class
make classes with files that inherit from abstract class
*/

/*
//
//              AudioFile Trait
//
*/
use std::fs::File;
use crate::engine::models::*;
use crate::engine::config::*;
use sha256;

pub fn file_to_hash(filepath: String) -> String {
    let bytes = std::fs::read(filepath).unwrap();
    let hash = sha256::digest_bytes(&bytes);

    return hash;
}


/// We want to make a trait that has the following functions
/// * get_song_table_data returns SONG_TABLE_DATA struct
/// * get_song_artists_table_data returns SONG_ARTISTS_TABLE_DATA struct
/// * get_album_artists_table_data returns ALBUM_ARTISTS_TABLE_DATA struct
/// * get_composers_table_data returns COMPOSERS_TABLE_DATA struct
/// * get_genres_table_data returns GENRES_TABLE_DATA struct
/// * load_file(filepath) which loads the file and it's data
pub trait AudioFile {
    fn get_song_table_data(&self) -> SONG_TABLE_DATA;
    fn get_song_artists_table_data(&self) -> SONG_ARTISTS_TABLE_DATA;
    fn get_album_artists_table_data(&self) -> ALBUM_ARTISTS_TABLE_DATA;
    fn get_composers_table_data(&self) -> COMPOSERS_TABLE_DATA;
    fn get_genres_table_data(&self) -> GENRES_TABLE_DATA;
    fn load_file(&mut self, filepath:String);
}

pub struct AudioFileMP3 {
    song_table_data: SONG_TABLE_DATA,
    song_artists_table_data: SONG_ARTISTS_TABLE_DATA,
    album_artists_table_data: ALBUM_ARTISTS_TABLE_DATA,
    composers_table_data: COMPOSERS_TABLE_DATA,
    genres_table_data: GENRES_TABLE_DATA,
    filepath: String,
}

pub struct AudioFileFlac {
    song_table_data: SONG_TABLE_DATA,
    song_artists_table_data: SONG_ARTISTS_TABLE_DATA,
    album_artists_table_data: ALBUM_ARTISTS_TABLE_DATA,
    composers_table_data: COMPOSERS_TABLE_DATA,
    genres_table_data: GENRES_TABLE_DATA,
    filepath: String,
}