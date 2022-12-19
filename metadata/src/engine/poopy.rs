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
use audiotags::{Tag, Picture, MimeType};
use sha256::{digest, try_digest};
use std::collections::HashMap;
use std::string::String;

fn file_to_hash(path: String) -> String {
    let bytes = std::fs::read(path).unwrap();  // Vec<u8>
    let hash = digest(&bytes);

    hash
}
pub trait AudioFile {
    fn load_file(&self,filepath: String);
    fn get_song_table_data(&self) -> HashMap<String, String>;
    fn get_album_artist_data(&self) -> Vec<String>;
    fn get_song_artist_data(&self) -> Vec<String>;
    fn get_composer_data(&self) -> Vec<String>;
    fn get_genre_data(&self) -> Vec<String>;
}

/*
//
//              MP3 Support
//
*/

pub struct AudioFileMP3 {
    metadata: HashMap<String, String>,
    song_table_data: HashMap<String, String>,
    hash: String
}

impl AudioFileMP3 {
    fn new() -> AudioFileMP3 {
        AudioFileMP3 {}
    }
    fn load_metadata_params(&mut self, params: HashMap<String, String>) {
        self.song_table_data = params;
    }
    fn make_song_table_data(&mut self) {
    }
}

impl AudioFile for AudioFileMP3 {
    fn load_file(self: &AudioFileMP3, path: String) {
        
        self.hash = file_to_hash(path);
        self.metadata = Tag::new().read_from_path(path).unwrap();
    }
    fn get_song_table_data(&self) -> HashMap<String,String>;
    fn get_album_artist_data(&self) -> Vec<String>;
    fn get_song_artist_data(&self) -> Vec<String>;
    fn get_composer_data(&self) -> Vec<String>;
    fn get_genre_data(&self) -> Vec<String>;
}

/*
//
//              FLAC Support
//
*/

pub struct AudioFileFLAC {

}

pub impl AudioFileflac {
    fn new() -> AudioFileFLAC {
        AudioFileflac {}
    }
}

pub impl AudioFile for AudioFileFLAC {

}
