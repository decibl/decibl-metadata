#![allow(warnings)]

use decibl_metadata::engine::{*, self};
use std::path;
use std::collections::HashMap;
use audiotags::{Tag, Picture, MimeType};
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;


fn test_insert_song(){
    engine::create_all_tables();
    engine::clear_all_tables();
    
    // make a SONG_TABLE_DATA object from struct engine::SONG_TABLE_DATA
    let mut song_table_data = engine::SONG_TABLE_DATA {
        song_id: String::from("b3rsuXs2hz"),
        main_artist: String::from("bruh"),
        filesize_bytes: 0,
        padding_bytes: 0,
        album_artwork_bit_depth: 0,
        album_artwork_colors: 0,
        album_artwork_height: 0,
        album_artwork_width: 0,
        bit_depth: 0,
        bitrate: 0,
        channels: 0,
        duration: 0.0,
        sample_rate: 0,
        album: String::from("bruh"),
        barcode: String::from("bruh"),
        date_created: String::from("bruh"),
        disc_number: 0,
        disc_total: 0,
        isrc: String::from("bruh"),
        itunesadvisory: String::from("bruh"),
        length: 0,
        publisher: String::from("bruh"),
        rating: 0,
        title: String::from("bruh"),
        track_number: 0,
        track_total: 0,
        source: String::from("bruh"),
    };

    engine::insert_song(song_table_data);
}

fn test_config_shit(){
    engine::create_all_tables();
    let tablenames = engine::get_all_table_names();
    println!("The tablenames are: {:?}", tablenames);
    // engine::create_all_files();
    let retn = engine::write_config_var("soundfiles_paths", "C:/Users/drale/Music/music/FavoritesPL");
    let valuepair = engine::get_config_var("soundfiles_path");
    println!("The valuepair is: {:?}", valuepair);

}

// C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/src/enemy.flac

fn salshit() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = "C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/src/enemy.flac";   

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

    println!("The meta is: {:?}", meta);



}


fn main() {
    //sal this for u
    // salshit();
    let filepathJeff = "C:/Users/Jeffrey Ma/Documents/GitHub/decibl-metadata/metadata/enemy.flac";
    let filePathSal = "C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/enemy.flac";   
    let mut afile = engine::AudioFileFlac::default();
    afile.load_file(filePathSal.to_string());
    afile.get_song_table_data();

    // let mut hash = engine::file_to_hash(filepath.to_string());
    // println!("The hash is: {}", hash);

    // jeffshit();

    // println!("Hello, world!");
    // engine::cringeit();
    // engine::create_all_files();
    // let exConfig = engine::Config {
    //     soundFilesPath: String::from("C://Users//james//Documents//GitHub//cringeit//metadata//src//test"),
    // };

    // engine::write_whole_config(exConfig);
    // let config = engine::write_config_var("bruh", "bruhf");

    // let contents = engine::get_config_as_str();

    // println!("The contents are: {}", contents);

    // let song_table_str = engine::compile_table(&engine::SONGS);
    // println!("The song table is: {}", song_table_str);

    // let soundFilesPath = engine::get_test_soundfiles_path();    
    // println!("The soundFilesPath is: {}", soundFilesPath);

    //let database_file_path = engine::get_database_file_path();
    //println!("The database_file_path is: {}", database_file_path);


    // copy SONG_TABLE_DATA from engine::models into a new HashMap
    //let mut song_table_data = engine::clone_map(&engine::SONG_TABLE_DATA);

    // set song_table_data["song_id"] to "test"

    //song_table_data.insert("song_id", "test");


    //engine::insert_song(song_table_data);
    




}