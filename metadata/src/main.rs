use metadata::engine;
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

    let database_file_path = engine::get_database_file_path();
    println!("The database_file_path is: {}", database_file_path);

    // pub static SONG_TABLE_DATA : Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    
    // copy SONG_TABLE_DATA from engine::models into a new HashMap
    let mut song_table_data = engine::clone_map(&engine::SONG_TABLE_DATA);

    // set song_table_data["song_id"] to "test"

    song_table_data.insert("song_id", "test");


    engine::insert_song(song_table_data);
    




}