use metadata::engine;
use std::path;

fn main() {
    println!("Hello, world!");
    engine::cringeit();
    engine::create_all_files();
    let exConfig = engine::Config {
        soundFilesPath: String::from("C:\\Users\\james\\Documents\\GitHub\\cringeit\\metadata\\src\\test"),
    };

    engine::write_whole_config(exConfig);
    let config = engine::write_config_var("bruh", "bruhf");

    let contents = engine::get_config_as_str();

    println!("The contents are: {}", contents);

    let song_table_str = engine::compile_table(&engine::SONGS);
    println!("The song table is: {}", song_table_str);

    let soundFilesPath = engine::get_test_soundfiles_path();    
    println!("The soundFilesPath is: {}", soundFilesPath);

    let database_file_path = engine::get_database_file_path();
    println!("The database_file_path is: {}", database_file_path);

    engine::create_song_table();
    




}