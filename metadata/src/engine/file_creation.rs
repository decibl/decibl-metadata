use std::fs::File;
use std::io::prelude::*;
use std::path;
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

static APP_INFO: Lazy<ProjectDirs> = Lazy::new(|| ProjectDirs::from("com", "decibl", "desktop").unwrap());
static CONFIG_FILE: Lazy<path::PathBuf> = Lazy::new(|| APP_INFO.config_dir().join("config.yaml"));

pub struct Config {
    pub soundFilesPath: String,
}

pub fn create_all_files() {
    // create config dir
    let config_dir = APP_INFO.config_dir();
    let config_file_path = CONFIG_FILE.to_str().unwrap();
    std::fs::create_dir_all(config_dir).unwrap();
    let mut file = File::create(config_file_path).expect("Unable to create file");
    file.write_all("".as_bytes()).expect("Unable to write data");

    println!("The path is: {}", config_file_path);
}

pub fn write_whole_config(config: Config) {
    let mut map  = BTreeMap::new();
    map.insert("soundFilesPath", config.soundFilesPath);

    let yaml = serde_yaml::to_string(&map).unwrap();

    // write the yaml str to CONFIG_FILE
    let config_file_str = CONFIG_FILE.to_str().unwrap();
    let mut file = File::create(config_file_str).expect("Unable to create file");
    file.write_all(yaml.as_bytes()).expect("Unable to write data");

    println!("The path is: {}", yaml);

}






























// pub fn write_soundfiles_path(soundFilesPath: &str) {
//     // write the path to the file
//     // turn CONFIG_FILE into a string
//     let config_file_str = CONFIG_FILE.to_str().unwrap();
//     let mut file = File::create(config_file_str).expect("Unable to create file");
//     file.write_all(soundFilesPath.as_bytes()).expect("Unable to write data");

//     println!("The path is: {}", soundFilesPath);

//     // get all files in the directory
//     // let paths = std::fs::read_dir(soundFilesPath).unwrap();
//     // for path in paths {
//     //     println!("The path is: {}", path.unwrap().path().display());
//     // }

// }

// pub fn exists_soundfiles_path() -> bool {

//     let config_file_str = CONFIG_FILE.to_str().unwrap();
//     let mut file = File::open(config_file_str).expect("Unable to open file");
//     let mut contents = String::new();

//     file.read_to_string(&mut contents).expect("Unable to read string");
    
//     if contents.is_empty() {
//         false
//     } else {
//         true
//     }
// }

// pub fn get_soundfiles_path() -> String {
//     let config_file_str = CONFIG_FILE.to_str().unwrap();
//     let mut file = File::open(config_file_str).expect("Unable to open file");
//     let mut contents = String::new();

//     file.read_to_string(&mut contents).expect("Unable to read string");
//     contents
// }

pub fn cringeit() {
    println!("Cringe!");
}
