/*
make absract class
make audio metadata class
make classes with files that inherit from abstract class
*/

use std::io::Error;
/*
//
//              AudioFile Trait
//
*/
use std::fs::File;
use std::io;
use crate::engine::models::*;
use crate::engine::config::*;
use sha2::{Digest, Sha256};
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::FormatReader;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::Tag;
use symphonia::core::codecs;
use symphonia::core::probe::Hint;
// include hashmap
use std::collections::HashMap;

pub fn file_to_hash(filepath: String) -> Result<String,Error> {
    let mut hasher = Sha256::new();
    let mut file = File::open(filepath)?;
    
    let bytes_written = io::copy(&mut file, &mut hasher)?;
    let hash_bytes = hasher.finalize();
    let string = format!("{:x}", hash_bytes);
    Ok(string)
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
    raw_metadata: HashMap<String, Vec<String>>,
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

impl AudioFileFlac{
    fn get_symphonia_data(&self, filepath: String) -> Box<dyn FormatReader> {
        let args = std::env::args().collect::<Vec<String>>();
        let path = filepath.clone();

        let src = std::fs::File::open(path).unwrap();

        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("flac");

        let meta_opts = MetadataOptions::default();
        let format_opts = FormatOptions::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &meta_opts).expect("failed to probe");

        let mut format = probed.format;

        format
    }

    fn add_symphonia_data(&mut self, filepath: String){
        let mut format = self.get_symphonia_data(filepath.clone());

        let binding = format.metadata();
        let metaTags = binding.current().unwrap().tags();
        let visualTags = binding.current().unwrap().visuals();

        let vec_meta = metaTags.to_vec();
        let vec_visual = visualTags.to_vec();

        // Make a hashmap of <string, string> to store the metadata
        let mut metadata: HashMap<String, Vec<String>> = HashMap::new();

        for tag in vec_meta {
            let key = tag.key.to_string();
            let value = tag.value.to_string();

            // if key is already in the hashmap, push the value to the vector
            if metadata.contains_key(&key) {
                let mut vec = metadata.get_mut(&key).unwrap();
                vec.push(value);
            } else {
                // if key is not in the hashmap, create a new vector and push the value to it
                let mut vec = Vec::new();
                vec.push(value);
                metadata.insert(key, vec);
            }

        }

        // now add the following 
        // pub album_artwork_bit_depth: i64, // picture::depth
        // pub album_artwork_colors: i64, // picture::num_colors
        // pub album_artwork_height: i64, // picture::height
        // pub album_artwork_width: i64,

        let mut artwork_bit_depth_vec: Vec<String> = Vec::new();
        // let mut artwork_colors_vec: Vec<String> = Vec::new();
        let mut artwork_height_vec: Vec<String> = Vec::new();
        let mut artwork_width_vec: Vec<String> = Vec::new();

        
        let bit_depth = vec_visual[0].bits_per_pixel.expect("No bit depth");
        let height = vec_visual[0].dimensions.unwrap().height;
        let width = vec_visual[0].dimensions.unwrap().width;

        artwork_bit_depth_vec.push(bit_depth.to_string());
        artwork_height_vec.push(height.to_string());
        artwork_width_vec.push(width.to_string());

        metadata.insert("album_artwork_bit_depth".to_string(), artwork_bit_depth_vec);
        metadata.insert("album_artwork_height".to_string(), artwork_height_vec);
        metadata.insert("album_artwork_width".to_string(), artwork_width_vec);



        // println!("The colors are: {:?}", colors);
        self.raw_metadata = metadata;

    }

    
}

impl AudioFile for AudioFileFlac{

    fn load_file(&mut self, filepath: String) {
        
        self.add_symphonia_data(filepath.clone());
        println!("New metadata: {:?}", self.raw_metadata);

        self.filepath = filepath;


        // println!("The metaTags is: {:?}", metaTags);
    }

    fn default() -> Self {
        AudioFileFlac {
            song_table_data: SONG_TABLE_DATA::default(),
            song_artists_table_data: SONG_ARTISTS_TABLE_DATA::default(),
            album_artists_table_data: ALBUM_ARTISTS_TABLE_DATA::default(),
            composers_table_data: COMPOSERS_TABLE_DATA::default(),
            genres_table_data: GENRES_TABLE_DATA::default(),
            raw_metadata: HashMap::new(),
            filepath: String::new(),
        }
    }

    fn get_song_table_data(&self) -> SONG_TABLE_DATA {
        // make a new song table data struct
        let mut song_table_data = SONG_TABLE_DATA::default();
        song_table_data.song_id = file_to_hash(self.filepath.clone()).unwrap();
        println!("song_id: {:?}", song_table_data.song_id);
        

        // println!("hashmap: {:?}", self.raw_metadata);
        
        song_table_data
    }

}