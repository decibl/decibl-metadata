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
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::Tag;
use symphonia::core::probe::Hint;

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
    // fn get_song_artists_table_data(&self) -> SONG_ARTISTS_TABLE_DATA;
    // fn get_album_artists_table_data(&self) -> ALBUM_ARTISTS_TABLE_DATA;
    // fn get_composers_table_data(&self) -> COMPOSERS_TABLE_DATA;
    // fn get_genres_table_data(&self) -> GENRES_TABLE_DATA;
    fn load_file(&mut self, filepath:String);
    fn default() -> Self;
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
    //list of Tag
    raw_metadata: Vec<Tag>,
    filepath: String,
}

// let args = std::env::args().collect::<Vec<String>>();
// let path = "C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/src/enemy.flac";   

// let src = std::fs::File::open(path).unwrap();

// let mss = MediaSourceStream::new(Box::new(src), Default::default());
// let mut hint = Hint::new();
// hint.with_extension("flac");

// let meta_opts = MetadataOptions::default();
// let format_opts = FormatOptions::default();

// let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &meta_opts).expect("failed to probe");

// let mut format = probed.format;

// let binding = format.metadata();
// let meta = binding.current().unwrap().tags();

// println!("The meta is: {:?}", meta);
impl AudioFile for AudioFileFlac{

    fn load_file(&mut self, filepath: String) {
        let args = std::env::args().collect::<Vec<String>>();
        let path = filepath;

        let src = std::fs::File::open(path).unwrap();

        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("flac");

        let meta_opts = MetadataOptions::default();
        let format_opts = FormatOptions::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &meta_opts).expect("failed to probe");

        let mut format = probed.format;

        let binding = format.metadata();
        let meta = binding.current().unwrap().tags();

        self.raw_metadata = meta.to_vec();


        // println!("The meta is: {:?}", meta);
    }

    fn default() -> Self {
        AudioFileFlac {
            song_table_data: SONG_TABLE_DATA::default(),
            song_artists_table_data: SONG_ARTISTS_TABLE_DATA::default(),
            album_artists_table_data: ALBUM_ARTISTS_TABLE_DATA::default(),
            composers_table_data: COMPOSERS_TABLE_DATA::default(),
            genres_table_data: GENRES_TABLE_DATA::default(),
            raw_metadata: Vec::new(),
            filepath: String::new(),
        }
    }

    fn get_song_table_data(&self) -> SONG_TABLE_DATA {
        // make a new song table data struct
        let mut song_table_data = SONG_TABLE_DATA::default();


        println!("The raw metadata is: {:?}", self.raw_metadata);
        
        song_table_data
    }

}