#![allow(warnings)]
use std::io::Write;

#[allow(non_camel_case_types)]
use decibl_metadata::engine::analyticsdb;
use decibl_metadata::engine::analyticsdb::*;
use decibl_metadata::engine::audio_metadata::*;
use decibl_metadata::engine::api_metadata::*;

use decibl_metadata::engine::config::*;
use decibl_metadata::engine::models::*;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

fn main() {

    let path = "C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/enemy.flac".to_string();

    let photo_data = get_symphonia_picture_data(path, "flac".to_string());

    let mut file = std::fs::File::create("C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/enemy.jpg").unwrap();
    file.write_all(&photo_data).unwrap();

    



}