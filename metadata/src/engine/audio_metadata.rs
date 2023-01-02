/*
This class will deal with all the parsing of the physical music files. Like getting the "title" metadata from hello.mp3
The basic rundown is the following:
1. Make a trait that all the audio files will inherit from. This will have required functions that all audio files will have, so we can use them in the same way.
2. Make a class for each Audiofile type and make it inherit from the trait. This will have the functions that are specific to that audio file type, but same return type.

For example, if I have AudioFileFLAC and AudioFileMP3 and call get_title() on both, they will return the same type of data, but the implementation will be different.
*/

#![allow(non_snake_case)]
use ring::digest::{Context, Digest, SHA256};
use std::io::{BufReader, Read, Result, Write};
use std::fs::File;

use crate::engine::models::*;

use data_encoding::HEXUPPER;

use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::FormatReader;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

// include hashmap
// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use metaflac;
use mp3_metadata;
use std::collections::HashMap;

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                                  USEFUL FUNCTIONS
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

/// We want to use the AudioFIle trait b/c there's multiple possible AudioFile types (flac or mp3)
/// They're all guranteed to have this functionality.
pub trait AudioFile {
    fn get_song_table_data(&self) -> SONG_TABLE_DATA;
    fn get_song_artists_table_data(&self) -> Vec<SONG_ARTISTS_TABLE_DATA>;
    fn get_album_artists_table_data(&self) -> Vec<ALBUM_ARTISTS_TABLE_DATA>;
    fn get_composers_table_data(&self) -> Vec<COMPOSERS_TABLE_DATA>;
    fn get_genres_table_data(&self) -> Vec<GENRES_TABLE_DATA>;
    fn load_file(&mut self, filepath: String);
}

/// Used for hashing files. SHA256 is the algorithm used.
pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
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

/// Also used for hashing files
pub fn file_to_hash(path: String) -> Result<String> {
    let input = File::open(path.clone())?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}

/// Turns a string into a hash
pub fn string_to_hash(path: String) -> Result<String> {
    let reader = path.as_bytes();
    let digest = sha256_digest(reader)?;

    Ok(HEXUPPER.encode(digest.as_ref()))
}

/// Function that gets the metadata for a file using the symphonia library
/// Returns a Symphonia object that should be used with add_symphonia_data
pub fn get_symphonia_data(filepath: String, fileHint: String) -> Box<dyn FormatReader> {
    let path = filepath.clone();

    let src = std::fs::File::open(path).unwrap();

    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension(&fileHint);

    let meta_opts = MetadataOptions::default();
    let format_opts = FormatOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &meta_opts)
        .expect("failed to probe");

    let format = probed.format;

    format
}

/// Function that takes a Symphonia object and returns a hashmap of metadata
/// It's still raw metadata so it has to be parsed by other methods
pub fn add_symphonia_data(
    filepath: String,
    fileHint: String,
) -> HashMap<std::string::String, Vec<std::string::String>> {
    // yeah i'm gonne be 100% real wit u
    // i have no idea what half of this code does. I just saw it from the docs and copied and it works.

    let mut format = get_symphonia_data(filepath.clone(), fileHint.clone());

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
            let vec = metadata.get_mut(&key).unwrap();
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

    // check if bits_per_pixel is not None
    // check if length of vec_visual is not 0

    if vec_visual.len() == 0 || vec_visual[0].bits_per_pixel.is_none() {
        artwork_bit_depth_vec.push("0".to_string());
        metadata.insert("album_artwork_bit_depth".to_string(), artwork_bit_depth_vec);
    } else {
        let bit_depth = vec_visual[0].bits_per_pixel.expect("No bit depth");
        artwork_bit_depth_vec.push(bit_depth.to_string());
        metadata.insert("album_artwork_bit_depth".to_string(), artwork_bit_depth_vec);
    }

    if vec_visual.len() == 0 || vec_visual[0].dimensions.is_none() {
        artwork_height_vec.push("0".to_string());
        metadata.insert("album_artwork_height".to_string(), artwork_height_vec);

        artwork_width_vec.push("0".to_string());
        metadata.insert("album_artwork_width".to_string(), artwork_width_vec);
    } else {
        let height = vec_visual[0].dimensions.unwrap().height;
        artwork_height_vec.push(height.to_string());
        metadata.insert("album_artwork_height".to_string(), artwork_height_vec);

        let width = vec_visual[0].dimensions.unwrap().width;
        artwork_width_vec.push(width.to_string());
        metadata.insert("album_artwork_width".to_string(), artwork_width_vec);
    }

    metadata
}

/// Get the album artwork from a file using the symphonia library
/// Returns Vec<u8> of the album artwork
pub fn get_symphonia_picture_data(filepath: String, fileHint: String) -> Vec<u8> {
    let src = std::fs::File::open(filepath.clone()).unwrap();

    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("flac");

    let meta_opts = MetadataOptions::default();
    let format_opts = FormatOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &meta_opts)
        .expect("failed to probe");

    let format = probed.format;

    let mut format = get_symphonia_data(filepath.clone(), "flac".to_string());
    let binding = format.metadata();
    let visualTags = binding.current().unwrap().visuals();

    let mut album_art = None;

    for tag in visualTags {
        if tag.media_type == "image/jpeg" {
            album_art = Some(tag.data.to_vec());
        }
        if tag.media_type == "image/png" {
            album_art = Some(tag.data.to_vec());
        }
    }

    let album_art = album_art.unwrap();

    album_art
}

pub fn write_symphonia_picture_data(filepath: String, album_art: Vec<u8>) {
    
    let mut file = std::fs::File::open(filepath.clone()).unwrap();
    file.write_all(&album_art).unwrap();
}
/// We want to make a trait that has the following functions
/// * get_song_table_data returns SONG_TABLE_DATA struct
/// * get_song_artists_table_data returns SONG_ARTISTS_TABLE_DATA struct
/// * get_album_artists_table_data returns ALBUM_ARTISTS_TABLE_DATA struct
/// * get_composers_table_data returns COMPOSERS_TABLE_DATA struct
/// * get_genres_table_data returns GENRES_TABLE_DATA struct
/// * load_file(filepath) which loads the file and it's data

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                                               FLAC
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

/// We will be using the Symphonia library and metaflac to parse stuff.
/// After the load_file call, this is what the raw_metadata will look like:
/// ```
/// {"ALBUMARTIST": ["Imagine Dragons", "Arcane", "League Of Legends"], "bit_depth": ["16"], "album_artwork_bit_depth": ["24"], "RATING": ["100"], "sample_rate": ["44100"],
/// "PUBLISHER": ["KIDinaKORNER/Interscope Records"], "GENRE": ["Alternative"], "BARCODE": ["602445192335"],
/// "ARTIST": ["Imagine Dragons", "Arcane", "League Of Legends"], "DATE": ["2021-10-27"], "DISCTOTAL": ["1"], "ALBUM": ["Enemy (from the series Arcane League of Legends)"],
/// "ISRC": ["USUM72120989"], "DISCNUMBER": ["1"], "TITLE": ["Enemy (From the series Arcane League of Legends)"], "album_artwork_height": ["800"], "album_artwork_width": ["800"], "duration": ["173"], "ITUNESADVISORY": ["0"], "bitrate": ["1411200"],
/// "LENGTH": ["173000"], "TRACKNUMBER": ["1"], "TRACKTOTAL": ["1"],
///  "COMPOSER": ["Dan Reynolds", "Wayne Sermon", "Ben McKee", "Daniel Platzman", "Robin Fredriksson", "Mattias Larsson", "Justin Tranter", "Destin Route"], "SOURCE": ["Deezer"], "channels": ["2"], "SOURCEID": ["1543744602"]}
/// ```
#[derive(Debug, Clone)]
pub struct AudioFileFLAC {
    pub raw_metadata: HashMap<String, Vec<String>>,
    pub filepath: String,
}

impl AudioFileFLAC {
    pub fn default() -> Self {
        AudioFileFLAC {
            raw_metadata: HashMap::new(),
            filepath: String::new(),
        }
    }

    pub fn add_blank_data(&mut self) {
        // add all the above raw.metadata to the song_table_data so it doesnt throw an error
        // make a list of these keys so we can iterate over them
        let keys = vec![
            "song_id",
            "ARTIST",
            "filesize",
            "album_artwork_bit_depth",
            "album_artwork_colors",
            "album_artwork_height",
            "album_artwork_width",
            "bit_depth",
            "bitrate",
            "channels",
            "duration",
            "sample_rate",
            "ALBUM",
            "BARCODE",
            "DATE",
            "DISCNUMBER",
            "DISCTOTAL",
            "ISRC",
            "ITUNESADVISORY",
            "LENGTH",
            "PUBLISHER",
            "TITLE",
            "TRACKNUMBER",
            "TRACKTOTAL",
            "SOURCE",
            "album_artwork",
            "COMPOSER",
            "GENRE",
            "ALBUMARTIST",
        ];

        // go through each key and add it if it doesnt exist
        for key in keys {
            if !self.raw_metadata.contains_key(key) {
                self.raw_metadata
                    .insert(key.to_string(), vec!["-1".to_string()]);
            }
        }
    }

    pub fn get_metaflac_data(&mut self, filepath: String) -> metaflac::block::StreamInfo {
        let tag = metaflac::Tag::read_from_path(filepath).unwrap();
        let metadata = tag.get_streaminfo().unwrap();
        let retn = metadata.clone();

        return retn;
    }

    pub fn add_metaflac_data(&mut self, filepath: String) {
        let streaminfo = self.get_metaflac_data(filepath.clone());

        // we have the following keys we need in stream info:
        // sample_rate, channels, bit_depth,

        let mut sample_rate_vec: Vec<String> = Vec::new();
        let mut channels_vec: Vec<String> = Vec::new();
        let mut bit_depth_vec: Vec<String> = Vec::new();

        // StreamInfo { min_block_size: 4096, max_block_size: 4096, min_frame_size: 14, max_frame_size: 14705, sample_rate: 44100, num_channels: 2, bits_per_sample: 16, total_samples: 7646112, md5: 190b7e14f9e20550342fcef433e52313 }
        sample_rate_vec.push(streaminfo.sample_rate.to_string());
        channels_vec.push(streaminfo.num_channels.to_string());
        bit_depth_vec.push(streaminfo.bits_per_sample.to_string());

        // now we need to calculate bitrate, and duration
        let mut bitrate_vec: Vec<String> = Vec::new();
        let mut duration_vec: Vec<String> = Vec::new();

        // we have to calculate bitrate

        let bitrate = streaminfo.bits_per_sample as u64
            * streaminfo.sample_rate as u64
            * streaminfo.num_channels as u64;

        bitrate_vec.push(bitrate.to_string());

        let duration = streaminfo.total_samples / streaminfo.sample_rate as u64;

        duration_vec.push(duration.to_string());

        self.raw_metadata
            .insert("sample_rate".to_string(), sample_rate_vec);
        self.raw_metadata
            .insert("channels".to_string(), channels_vec);
        self.raw_metadata
            .insert("bit_depth".to_string(), bit_depth_vec);
        self.raw_metadata.insert("bitrate".to_string(), bitrate_vec);
        self.raw_metadata
            .insert("duration".to_string(), duration_vec);
    }
}

impl AudioFile for AudioFileFLAC {
    fn get_composers_table_data(&self) -> Vec<COMPOSERS_TABLE_DATA> {
        let mut composers_table_data_vec: Vec<COMPOSERS_TABLE_DATA> = Vec::new();
        // if there is no composer, we return an empty vector
        if !self.raw_metadata.contains_key("COMPOSER") {
            return composers_table_data_vec;
        }
        let composers = self.raw_metadata.get("COMPOSER").unwrap();
        for composer in composers {
            // if composer is -1, we skip it
            if composer == "-1" {
                continue;
            }
            let mut composers_table_data = COMPOSERS_TABLE_DATA::default();
            composers_table_data.composer_name = composer.clone();
            composers_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            composers_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            composers_table_data_vec.push(composers_table_data);
        }
        return composers_table_data_vec;
    }

    fn get_genres_table_data(&self) -> Vec<GENRES_TABLE_DATA> {
        let mut genres_table_data_vec: Vec<GENRES_TABLE_DATA> = Vec::new();
        let genres = self.raw_metadata.get("GENRE").unwrap();

        for genre in genres {
            let mut genres_table_data = GENRES_TABLE_DATA::default();
            genres_table_data.genre_name = genre.clone();
            genres_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            genres_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            genres_table_data_vec.push(genres_table_data);
        }
        return genres_table_data_vec;
    }

    fn get_album_artists_table_data(&self) -> Vec<ALBUM_ARTISTS_TABLE_DATA> {
        let mut album_artists_table_data_vec: Vec<ALBUM_ARTISTS_TABLE_DATA> = Vec::new();
        let album_artists = self.raw_metadata.get("ALBUMARTIST").unwrap();
        for artist in album_artists {
            let mut album_artists_table_data = ALBUM_ARTISTS_TABLE_DATA::default();
            album_artists_table_data.artist_name = artist.clone();
            album_artists_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            album_artists_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            album_artists_table_data_vec.push(album_artists_table_data);
        }
        return album_artists_table_data_vec;
    }

    /// We need to return all the song artists for the song
    fn get_song_artists_table_data(&self) -> Vec<SONG_ARTISTS_TABLE_DATA> {
        let mut song_artists_table_data_vec: Vec<SONG_ARTISTS_TABLE_DATA> = Vec::new();
        let song_artists = self.raw_metadata.get("ARTIST").unwrap();
        for artist in song_artists {
            let mut song_artists_table_data = SONG_ARTISTS_TABLE_DATA::default();
            song_artists_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            song_artists_table_data.artist_name = artist.clone();
            song_artists_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            song_artists_table_data_vec.push(song_artists_table_data);
        }
        return song_artists_table_data_vec;
    }

    fn get_song_table_data(&self) -> SONG_TABLE_DATA {
        let mut song_table_data = SONG_TABLE_DATA::default();
        // if there is no song_id, write "unknown" to the database
        song_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
        song_table_data.main_artist = self.raw_metadata.get("ARTIST").unwrap()[0].clone();
        let temp_filesize_bytes = self.raw_metadata.get("filesize").unwrap()[0]
            .parse::<i64>();
        
        match temp_filesize_bytes {
            Ok(filesize_bytes) => song_table_data.filesize_bytes = filesize_bytes,
            Err(_) => song_table_data.filesize_bytes = -1,  
        }
        
        song_table_data.padding_bytes = -1; // just set this to -1 for now
        
        let temp_album_artwork_bit_depth = self.raw_metadata.get("album_artwork_bit_depth").unwrap()[0]
            .parse::<i64>();
        
        match temp_album_artwork_bit_depth {
            Ok(album_artwork_bit_depth) => song_table_data.album_artwork_bit_depth = album_artwork_bit_depth,
            Err(_) => song_table_data.album_artwork_bit_depth = -1,  
        }
        
        song_table_data.album_artwork_colors = -1; // just set this to -1 for now
        
        let temp_album_artwork_height = self.raw_metadata.get("album_artwork_height").unwrap()[0]
            .parse::<i64>();
        
        match temp_album_artwork_height {
            Ok(album_artwork_height) => song_table_data.album_artwork_height = album_artwork_height,
            Err(_) => song_table_data.album_artwork_height = -1,  
        }
        
        let temp_album_artwork_width = self.raw_metadata.get("album_artwork_width").unwrap()[0]
            .parse::<i64>();
        
        match temp_album_artwork_width {
            Ok(album_artwork_width) => song_table_data.album_artwork_width = album_artwork_width,
            Err(_) => song_table_data.album_artwork_width = -1,  
        }
        
        let temp_bit_depth = self.raw_metadata.get("bit_depth").unwrap()[0]
            .parse::<i64>();
        
        match temp_bit_depth {
            Ok(bit_depth) => song_table_data.bit_depth = bit_depth,
            Err(_) => song_table_data.bit_depth = -1,  
        }
        
        let temp_bitrate = self.raw_metadata.get("bitrate").unwrap()[0]
            .parse::<i64>();
        
        match temp_bitrate {
            Ok(bitrate) => song_table_data.bitrate = bitrate,
            Err(_) => song_table_data.bitrate = -1,  
        }
        
        let temp_channels = self.raw_metadata.get("channels").unwrap()[0]
            .parse::<i64>();
        
        match temp_channels {
            Ok(channels) => song_table_data.channels = channels,
            Err(_) => song_table_data.channels = -1,  
        }
        
        let temp_duration = self.raw_metadata.get("duration").unwrap()[0]
            .parse::<f64>();
        
        match temp_duration {
            Ok(duration) => song_table_data.duration = duration,
            Err(_) => song_table_data.duration = -1.0,  
        }
        
        let temp_sample_rate = self.raw_metadata.get("sample_rate").unwrap()[0]
            .parse::<i64>();
        
        match temp_sample_rate {
            Ok(sample_rate) => song_table_data.sample_rate = sample_rate,
            Err(_) => song_table_data.sample_rate = -1,  
        }
        
        song_table_data.album = self.raw_metadata.get("ALBUM").unwrap()[0].clone();
        song_table_data.barcode = self.raw_metadata.get("BARCODE").unwrap()[0].clone();
        song_table_data.date_created = self.raw_metadata.get("DATE").unwrap()[0].clone();
        
        let temp_disc_number = self.raw_metadata.get("DISCNUMBER").unwrap()[0]
            .parse::<i64>();
        
        match temp_disc_number {
            Ok(disc_number) => song_table_data.disc_number = disc_number,
            Err(_) => song_table_data.disc_number = -1,  
        }
        
        let temp_disc_total = self.raw_metadata.get("DISCTOTAL").unwrap()[0]
            .parse::<i64>();
        
        match temp_disc_total {
            Ok(disc_total) => song_table_data.disc_total = disc_total,
            Err(_) => song_table_data.disc_total = -1,  
        }
        
        song_table_data.isrc = self.raw_metadata.get("ISRC").unwrap()[0].clone();
        song_table_data.itunesadvisory = self.raw_metadata.get("ITUNESADVISORY").unwrap()[0].clone();
        
        let temp_length = self.raw_metadata.get("LENGTH").unwrap()[0]
            .parse::<i64>();
        
        match temp_length {
            Ok(length) => song_table_data.length = length,
            Err(_) => song_table_data.length = -1,  
        }
        
        song_table_data.publisher = self.raw_metadata.get("PUBLISHER").unwrap()[0].clone();
        song_table_data.title = self.raw_metadata.get("TITLE").unwrap()[0].clone();
        
        let temp_track_number = self.raw_metadata.get("TRACKNUMBER").unwrap()[0]
            .parse::<i64>();
        
        match temp_track_number {
            Ok(track_number) => song_table_data.track_number = track_number,
            Err(_) => song_table_data.track_number = -1,  
        }
        
        let temp_track_total = self.raw_metadata.get("TRACKTOTAL").unwrap()[0]
            .parse::<i64>();
        
        match temp_track_total {
            Ok(track_total) => song_table_data.track_total = track_total,
            Err(_) => song_table_data.track_total = -1,  
        }
        
        song_table_data.source = self.raw_metadata.get("SOURCE").unwrap()[0].clone();
        song_table_data.filetype = "flac".to_string();
        
        song_table_data
    }

    fn load_file(&mut self, filepath: String) {
        // add all the data from the symphonia library
        let metadata = add_symphonia_data(filepath.clone(), "flac".to_string());
        self.raw_metadata = metadata;
        self.add_blank_data();

        // // add all the data from the metaflac library
        // // THIS HAS TO BE CALLED SECOND
        self.add_metaflac_data(filepath.clone());

        self.filepath = filepath;

        // add the filesize to the metadata
        let file = File::open(&self.filepath).unwrap();
        let metadata = file.metadata().unwrap();
        let filesize = metadata.len();

        let mut filesize_vec: Vec<String> = Vec::new();
        filesize_vec.push(filesize.to_string());

        self.raw_metadata
            .insert("filesize".to_string(), filesize_vec);

        // make the song_id the hash of song title + artist + album
        let mut song_id = String::new();
        song_id.push_str(&self.raw_metadata.get("TITLE").unwrap()[0]);  
        song_id.push_str(&self.raw_metadata.get("filesize").unwrap()[0]);
        song_id.push_str(&self.raw_metadata.get("ALBUM").unwrap()[0]);
        self.raw_metadata.insert(
            "song_id".to_string(),
            vec![string_to_hash(song_id).unwrap()],
        );

    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                                               MP3
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


// OK I'M NOT GONNA LIE
// Everything involving MP3s is a mess
// It's not my fault, the support for MP3s in Rust is just not there
// You'll see some random if statements checking if something is None or not,
// that's because the mp3-metadata library doesn't always return the same data
// and is inconsistent sometimes

#[derive(Debug, Clone)]
pub struct AudioFileMP3 {
    raw_metadata: HashMap<String, Vec<String>>,
    filepath: String,
}

impl AudioFileMP3 {
    pub fn default() -> Self {
        Self {
            filepath: "".to_string(),
            raw_metadata: HashMap::new(),
        }
    }
    /// Adds a ton of data from the mp3-metadata library to the raw_metadata hashmap
    /// We can get the following data from this library:
    /// 1. title
    /// 2. artist
    /// 3. album
    /// 4. year
    /// 5. genre
    /// 6. duration
    /// 7. sample_rate (sampling freq)
    /// 8. bitrate
    /// 9. channels (channel type)
    /// 10. composers
    /// 11. publisher
    pub fn add_id3_data(&mut self, filepath: String) {
        let metadata = mp3_metadata::read_from_file(filepath).unwrap();
        let audiotag = metadata.tag;
        let mut id3_data: HashMap<String, Vec<String>> = HashMap::new();

        // if audioTag is None, then there is no ID3 data, so we can just return
        if !audiotag.is_none() {
            // if audioTag is not None, then we can unwrap it
            let audiotag = audiotag.unwrap();

            // make hashmap of all the data

            // add the title
            let mut title_vec: Vec<String> = Vec::new();
            let title = audiotag.title.to_string();
            // remove the trailing \0
            let title = title.trim_end_matches('\0').to_string();
            title_vec.push(title);
            id3_data.insert("title".to_string(), title_vec);

            // add the artist
            let mut artist_vec: Vec<String> = Vec::new();
            let artist = audiotag.artist.to_string();
            // remove the trailing \0
            let artist = artist.trim_end_matches('\0').to_string();
            artist_vec.push(artist);
            id3_data.insert("main_artist".to_string(), artist_vec);

            // add the album
            let mut album_vec: Vec<String> = Vec::new();
            let album = audiotag.album.to_string();
            // remove the trailing \0
            let album = album.trim_end_matches('\0').to_string();
            album_vec.push(album);
            id3_data.insert("album".to_string(), album_vec);

            // add the year
            let mut year_vec: Vec<String> = Vec::new();
            year_vec.push(audiotag.year.to_string());
            id3_data.insert("year".to_string(), year_vec);

            // add the genre
            let mut genre_vec: Vec<String> = Vec::new();
            // get the output of printing audiotag.genre
            let genre_string = format!("{:?}", audiotag.genre);
            genre_vec.push(genre_string);
            id3_data.insert("genre".to_string(), genre_vec);
        }

        // add the duration
        let mut duration_vec: Vec<String> = Vec::new();
        let duration = metadata.duration.as_secs();
        duration_vec.push(duration.to_string());
        id3_data.insert("duration".to_string(), duration_vec);

        // the sample_rate, bitrate, and channels should be taken from metadata.frames[0]
        // add the sample_rate
        let mut sample_rate_vec: Vec<String> = Vec::new();
        let sample_rate = metadata.frames[0].sampling_freq;
        sample_rate_vec.push(sample_rate.to_string());
        id3_data.insert("sample_rate".to_string(), sample_rate_vec);

        // add the bitrate
        let mut bitrate_vec: Vec<String> = Vec::new();
        let bitrate_u16 = metadata.frames[0].bitrate;
        // bitrate is in kbps, we want it in bps
        let bitrate: u64 = (bitrate_u16 as u64) * 1000;
        bitrate_vec.push(bitrate.to_string());
        id3_data.insert("bitrate".to_string(), bitrate_vec);

        // add the channels
        let mut channels_vec: Vec<String> = Vec::new();
        let channels = metadata.frames[0].chan_type;

        // possible values are: Stereo, Joint Stereo, Dual Channel, Single Channel, Unknown
        // convert these into 2, 2, 2, 1, 0
        let channels = match channels {
            mp3_metadata::ChannelType::Stereo => 2,
            mp3_metadata::ChannelType::JointStereo => 2,
            mp3_metadata::ChannelType::DualChannel => 2,
            mp3_metadata::ChannelType::SingleChannel => 1,
            mp3_metadata::ChannelType::Unknown => 0,
        };

        channels_vec.push(channels.to_string());
        id3_data.insert("channels".to_string(), channels_vec);

        // Composers and publishers will be found in metadata.optional_info

        // add the composers
        // let composers = &metadata.optional_info[0];
        // let composers_vec = composers.composers.clone();
        // id3_data.insert("composers".to_string(), composers_vec);

        // make sure the composer exists, check if len of metadata.optional_info is 0

        if metadata.optional_info.len() == 0 {
            let composers_vec: Vec<String> = Vec::new();
            id3_data.insert("composers".to_string(), composers_vec);
        }
        else {
            // if there is a composer,
            let composers = &metadata.optional_info[0].composers;
            let composers_vec = composers.clone();
            id3_data.insert("composers".to_string(), composers_vec);
        }

        // if there is no publisher, then we need to add an empty vector
        if metadata.optional_info.len() == 0 {
            let publisher_vec: Vec<String> = Vec::new();
            id3_data.insert("publisher".to_string(), publisher_vec);
        }
        else {
            // if there is a publisher,
            // make sure publisher is not None
            let publisher = &metadata.optional_info[0].publisher.clone();
            // let publisher = publisher.trim_end_matches('\0').to_string();
            // let publisher_vec = vec![publisher];

            // id3_data.insert("publisher".to_string(), publisher_vec);
            match publisher {
                Some(publisher) => {
                    let publisher = publisher.trim_end_matches('\0').to_string();
                    let publisher_vec = vec![publisher];
                    id3_data.insert("publisher".to_string(), publisher_vec);
                },
                None => {
                    let publisher_vec: Vec<String> = Vec::new();
                    id3_data.insert("publisher".to_string(), publisher_vec);
                }
            }
        }
        // print id3_data
        self.raw_metadata = id3_data;
    }
}

impl AudioFile for AudioFileMP3 {
    fn get_song_artists_table_data(&self) -> Vec<SONG_ARTISTS_TABLE_DATA> {
        let mut song_artists_vec: Vec<SONG_ARTISTS_TABLE_DATA> = Vec::new();

        // go through each artist in self.raw_metadata.get("main_artist")
        for artist in self.raw_metadata.get("main_artist").unwrap() {
            let mut song_artists_table_data = SONG_ARTISTS_TABLE_DATA::default();
            song_artists_table_data.artist_name = artist.clone();
            song_artists_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            song_artists_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            song_artists_vec.push(song_artists_table_data);
        }

        song_artists_vec
    }
    fn get_album_artists_table_data(&self) -> Vec<ALBUM_ARTISTS_TABLE_DATA> {
        let mut album_artists_vec: Vec<ALBUM_ARTISTS_TABLE_DATA> = Vec::new();

        // go through each artist in self.raw_metadata.get("album_artists")
        for artist in self.raw_metadata.get("main_artist").unwrap() {
            let mut album_artists_table_data = ALBUM_ARTISTS_TABLE_DATA::default();
            album_artists_table_data.artist_name = artist.clone();
            album_artists_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            album_artists_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            album_artists_vec.push(album_artists_table_data);
        }

        album_artists_vec
    }
    fn get_composers_table_data(&self) -> Vec<COMPOSERS_TABLE_DATA> {
        let mut composers_vec: Vec<COMPOSERS_TABLE_DATA> = Vec::new();

        // go through each composer in self.raw_metadata.get("composers")
        for composer in self.raw_metadata.get("composers").unwrap() {
            // if composer_name is -1, then there is no composer
            let mut composers_table_data = COMPOSERS_TABLE_DATA::default();
            composers_table_data.composer_name = composer.clone();
            composers_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            composers_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            // if composer_name is -1, then there is no composer
            composers_vec.push(composers_table_data);
        }

        composers_vec
    }
    fn get_genres_table_data(&self) -> Vec<GENRES_TABLE_DATA> {
        let mut genres_vec: Vec<GENRES_TABLE_DATA> = Vec::new();

        // go through each genre in self.raw_metadata.get("genre")
        for genre in self.raw_metadata.get("genre").unwrap() {
            // if genre is -1, then there is no genre
            if genre == "-1" {
                continue;
            }
            let mut genres_table_data = GENRES_TABLE_DATA::default();
            genres_table_data.genre_name = genre.clone();
            genres_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
            genres_table_data.dt_added = chrono::Utc::now().naive_utc().to_string();
            genres_vec.push(genres_table_data);
        }

        genres_vec
    }

    /// we want to return the following stuff. Yeah there isn't that much data we can get from mp3 files reliably :<
    /// 1. title
    /// 2. artist
    /// 3. album
    /// 4. year
    /// 6. duration
    /// 7. sample_rate (sampling freq)
    /// 8. bitrate
    /// 9. channels (channel type)
    fn get_song_table_data(&self) -> SONG_TABLE_DATA {
        let mut song_table_data = SONG_TABLE_DATA::default();

        song_table_data.song_id = self.raw_metadata.get("song_id").unwrap()[0].clone();
        song_table_data.filesize_bytes = self.raw_metadata.get("filesize").unwrap()[0]
            .parse::<i64>()
            .unwrap();

        // if self.raw_metadata.get("title")
        song_table_data.title = self.raw_metadata.get("title").unwrap()[0].clone();
        song_table_data.main_artist = self.raw_metadata.get("main_artist").unwrap()[0].clone();
        song_table_data.album = self.raw_metadata.get("album").unwrap()[0].clone();

        // if year is -1, then there is no year

        let new_year = self.raw_metadata.get("year").unwrap()[0]
            .parse::<i64>();

        match new_year {
            Ok(year) => {
                song_table_data.date_created = year.to_string();
            }
            Err(_) => {
                song_table_data.date_created = "".to_string();
            }
        }

        song_table_data.duration = self.raw_metadata.get("duration").unwrap()[0]
            .parse::<i64>()
            .unwrap() as f64;
        song_table_data.sample_rate = self.raw_metadata.get("sample_rate").unwrap()[0]
            .parse::<i64>()
            .unwrap();
        song_table_data.bitrate = self.raw_metadata.get("bitrate").unwrap()[0]
            .parse::<i64>()
            .unwrap();
        song_table_data.channels = self.raw_metadata.get("channels").unwrap()[0]
            .clone()
            .parse::<i64>()
            .unwrap();
        song_table_data.filetype = self.raw_metadata.get("filetype").unwrap()[0].clone();
        song_table_data
    }

    fn load_file(&mut self, filepath: String) {
        self.filepath = filepath.clone();

        let defaultMap = SONG_TABLE_DATA::default();

        // now we need to make default values for all the metadata.

        self.add_id3_data(filepath.clone());

        // now go through self.raw_metadata and add the default values for the ones that are missing, i.e. if it's None
        let keys = [
            "title",
            "main_artist",
            "album",
            "year",
            "duration",
            "sample_rate",
            "bitrate",
            "channels",
            "composers",
            "genre",
            "filesize",
            "song_id",
            "filetype",
        ];
        for key in keys.iter() {
            if self.raw_metadata.get(key.clone()).is_none() {
                match key {
                    &"title" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.title.clone()]),
                    &"main_artist" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.main_artist.clone()]),
                    &"album" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.album.clone()]),
                    &"year" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.date_created.clone()]),
                    &"duration" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.duration.to_string()]),
                    &"sample_rate" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.sample_rate.to_string()]),
                    &"bitrate" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.bitrate.to_string()]),
                    &"channels" => self
                        .raw_metadata
                        .insert(key.to_string(), vec![defaultMap.channels.to_string()]),
                    &"composers" => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                    &"genre" => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                    &"filesize" => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                    &"song_id" => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                    &"filetype" => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                    _ => self
                        .raw_metadata
                        .insert(key.to_string(), vec!["-1".to_string()]),
                };
            }
        }

        // add the filesize to the metadata
        let file = File::open(&self.filepath).unwrap();
        let metadata = file.metadata().unwrap();
        let filesize = metadata.len();

        let mut filesize_vec: Vec<String> = Vec::new();
        filesize_vec.push(filesize.to_string());

        self.raw_metadata
            .insert("filesize".to_string(), filesize_vec);
        self.raw_metadata.insert(
            "song_id".to_string(),
            vec![file_to_hash(self.filepath.clone()).unwrap()],
        );
        self.raw_metadata
            .insert("filetype".to_string(), vec!["mp3".to_string()]);
    }
}
