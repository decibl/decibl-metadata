#![allow(non_snake_case)]

use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path;

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                          GLOBAL PATHS
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

pub static APP_INFO: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("com", "decibl", "desktop").unwrap());
pub static CONFIG_FILE_PATH: Lazy<path::PathBuf> =
    Lazy::new(|| APP_INFO.config_dir().join("config.yaml"));
pub static DATABASE_FILE_PATH: Lazy<path::PathBuf> =
    Lazy::new(|| APP_INFO.config_dir().join("analytics.db"));
pub static ARTIST_PHOTO_PATH: Lazy<path::PathBuf> =
    Lazy::new(|| APP_INFO.config_dir().join("artists"));

pub static TEST_SOUNDFILES_PATH: Lazy<path::PathBuf> = Lazy::new(|| {
    path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("test_soundfiles")
});

pub static TEST_SOUNDFILES_PATH_1: Lazy<path::PathBuf> =
    Lazy::new(|| TEST_SOUNDFILES_PATH.join("1/"));

pub fn get_config_file_path() -> String {
    CONFIG_FILE_PATH.to_str().unwrap().to_string()
}

pub fn get_database_file_path() -> String {
    DATABASE_FILE_PATH.to_str().unwrap().to_string()
}

pub fn get_test_soundfiles_path() -> String {
    TEST_SOUNDFILES_PATH.to_str().unwrap().to_string()
}

pub fn get_artist_photo_path() -> String {
    ARTIST_PHOTO_PATH.to_str().unwrap().to_string()
}

pub fn get_album_photo_path(artist_name: &str, album_name: &str) -> String {
    let mut path = APP_INFO.config_dir().join("artists");
    // we want the apth to be config_dir() / "artists" / artist_name / album_name
    path.push(artist_name);
    path.push(album_name);
    path.to_str().unwrap().to_string()
}

pub fn get_soundfiles_path() -> String {
    get_config_var("soundfiles_paths").1
}

pub fn get_soundfiles_path_1() -> String {
    TEST_SOUNDFILES_PATH_1.to_str().unwrap().to_string()
}
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           CONFIG FILE
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

pub struct Config {
    pub soundFilesPath: String,
}

pub fn create_all_files() {
    // create config dir
    let config_dir = APP_INFO.config_dir();
    let config_file_path = CONFIG_FILE_PATH.to_str().unwrap();

    std::fs::create_dir_all(config_dir).unwrap();
    std::fs::create_dir_all(ARTIST_PHOTO_PATH.to_str().unwrap()).unwrap();
    // get_album_photo_path("artist", "album");
    std::fs::create_dir_all(get_album_photo_path("artist", "album")).unwrap();

    // only create the config file if it doesn't exist
    if std::path::Path::new(config_file_path).exists() {
        return;
    }
    let mut file = File::create(config_file_path).expect("Unable to create file");
    file.write_all("".as_bytes()).expect("Unable to write data");

    println!("The path is: {}", config_file_path);
}

pub fn write_whole_config(config: Config) {
    let mut map = BTreeMap::new();
    map.insert("soundFilesPath", config.soundFilesPath);

    let yaml = serde_yaml::to_string(&map).unwrap();

    // write the yaml str to CONFIG_FILE_PATH
    let config_file_str = CONFIG_FILE_PATH.to_str().unwrap();
    let mut file = File::create(config_file_str).expect("Unable to create file");
    file.write_all(yaml.as_bytes())
        .expect("Unable to write data");

}

// make function write_config_var which accepts a string and a string and writes the string to the config file with the key being the string

pub fn write_config_var(key: &str, value: &str) {
    // write the path to the file
    // turn CONFIG_FILE_PATH into a string
    let config_file_str = CONFIG_FILE_PATH.to_str().unwrap();

    // now open the yaml file
    let mut file = File::open(config_file_str).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read string");

    // if the file is empty, create a new map
    if contents.is_empty() {
        let mut map = BTreeMap::new();
        map.insert(key.to_string(), value.to_string());

        let yaml = serde_yaml::to_string(&map).unwrap();

        // write the yaml str to CONFIG_FILE_PATH

        let mut writeFile = File::create(config_file_str).expect("Unable to create file");
        writeFile
            .write_all(yaml.as_bytes())
            .expect("Unable to write data");


        return;
    }

    let mut deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&contents).unwrap();

    // if the key already exists, overwrite it
    if deserialized_map.contains_key(key) {
        deserialized_map.insert(key.to_string(), value.to_string());
    } else {
        deserialized_map.insert(key.to_string(), value.to_string());
    }

    let yaml = serde_yaml::to_string(&deserialized_map).unwrap();

    // write the yaml str to CONFIG_FILE_PATH

    let mut writeFile = File::create(config_file_str).expect("Unable to create file");
    writeFile
        .write_all(yaml.as_bytes())
        .expect("Unable to write data");

}

/// Returns a tuple of the key and value of the config file for the given key
pub fn get_config_var(key: &str) -> (String, String) {
    let config_file_str = CONFIG_FILE_PATH.to_str().unwrap();
    let mut file = File::open(config_file_str).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read string");

    let deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&contents).unwrap();

    let value = deserialized_map.get(key).unwrap();

    (key.to_string(), value.to_string())
}

pub fn get_config_as_str() -> String {
    let config_file_str = CONFIG_FILE_PATH.to_str().unwrap();
    let mut file = File::open(config_file_str).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read string");

    contents
}

pub fn cringeit() {
    println!("Cringe!");
}
