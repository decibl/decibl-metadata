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

    let src = std::fs::File::open(path.clone()).unwrap();

    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("flac");

    let meta_opts = MetadataOptions::default();
    let format_opts = FormatOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &meta_opts)
        .expect("failed to probe");

    let format = probed.format;

    let mut format = get_symphonia_data(path.clone(), "flac".to_string());
    let binding = format.metadata();
    let visualTags = binding.current().unwrap().visuals();

    let mut album_art = None;

    for tag in visualTags {
        if tag.media_type == "image/jpeg" {
            album_art = Some(tag.data.to_vec());
        }
    }

    let mut album_art = album_art.unwrap();

    let mut file = std::fs::File::create("C:/Users/drale/Documents/GitHub/decibl-metadata/metadata/enemy.jpg").unwrap();
    file.write_all(&mut album_art).unwrap();

    



}