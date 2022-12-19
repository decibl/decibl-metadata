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
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write,Result};
use std::collections::HashMap;
use std::string::String;
use audiotags::{Tag};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn file_to_hash(path:String) -> Result<String> {

    let mut output = File::create(path.clone())?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path.clone())?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}
pub trait AudioFile {
    
    fn load_file(&mut self);
    /*
    fn get_song_table_data(&mut self) -> HashMap<String, HashMapValues>;
    
    fn get_album_artist_data(&self) -> Vec<String>;
    fn get_song_artist_data(&self) -> Vec<String>;
    fn get_composer_data(&self) -> Vec<String>;
    fn get_genre_data(&self) -> Vec<String>;
    */
    
}

/*
//
//              MP3 Support
//
*/
//create default attributes for the struct
pub struct AudioFileMP3 {
    metadata: HashMap<String, String>,
    song_table_data: HashMap<String, String>,
    hash: String,
    filepath: String,
}


impl AudioFileMP3 {
    pub fn new(filepath: String) -> AudioFileMP3 {

        AudioFileMP3 {metadata: HashMap::new(), song_table_data: HashMap::new(), hash: String::new(),filepath: filepath}
    }
    /*
    fn make_song_table_data(&mut self) {
        self.song_table_data.insert("song_id".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("filepath".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("main_artist".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("filesize".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("padding".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("album_artwork_bit_depth".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("album_artwork_colors".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("album_artwork_height".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("album_artwork_width".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("bit_depth".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("bitrate".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("channels".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("duration".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("sample_rate".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("album".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("barcode".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("date_created".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("disc_number".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("disc_total".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("isrc".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("itunesadvisory".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("length".to_string(), HashMapValues::Int(0));
        self.song_table_data.insert("publisher".to_string(), HashMapValues::Str("".to_string()));
        self.song_table_data.insert("rating".to_string(), HashMapValues::Int(0));
    
    }
    */
}

impl AudioFile for AudioFileMP3 {
    fn load_file(&mut self) {
        self.hash = file_to_hash(self.filepath.clone()).unwrap();
        //print self.hash
        println!("hash: {}", self.hash);
        let mut tag = Tag::new().read_from_path(self.filepath.clone()).unwrap();
        //print tag
        println!("tag: {:?}", tag.title());
        //self.metadata.insert("title".to_string(), tag.title().unwrap().to_string());


    }
    /* 
    fn get_album_artist_data(&self) -> Vec<String>;
    fn get_song_artist_data(&self) -> Vec<String>;
    fn get_composer_data(&self) -> Vec<String>;
    fn get_genre_data(&self) -> Vec<String>;
    */
}

/*
//
//              FLAC Support
//
*/

pub struct AudioFileFLAC {

}

impl AudioFileFLAC {
    fn new() -> AudioFileFLAC {
        AudioFileFLAC {}
    }
}

/* 
impl AudioFile for AudioFileFLAC {

}
*/
