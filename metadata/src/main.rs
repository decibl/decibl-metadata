#![allow(warnings)]
use metadata::engine::{*, self};
use std::path;
use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // engine::cringeit();
    // engine::create_all_files();
    // let exConfig = engine::Config {
    //     soundFilesPath: String::from("C:\\Users\\james\\Documents\\GitHub\\cringeit\\metadata\\src\\test"),
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

    engine::create_all_tables();
    
    // make a SONG_TABLE_DATA object from struct engine::SONG_TABLE_DATA
    let mut song_table_data = engine::SONG_TABLE_DATA {
        song_id: String::from("b3ru2h"),
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
        sample_rate_khz: 0,
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
    // copy SONG_TABLE_DATA from engine::models into a new HashMap
    //let mut song_table_data = engine::clone_map(&engine::SONG_TABLE_DATA);

    // set song_table_data["song_id"] to "test"

    //song_table_data.insert("song_id", "test");


    //engine::insert_song(song_table_data);
    
    // let mut test = AudioFileMP3::new("./02 - Gemstone.flac".to_string());
    // test.load_file();
    




}