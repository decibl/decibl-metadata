#![allow(warnings)]
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


    get_artist_profile_url_genius(&"Kanye West".to_string());
    get_artist_profile_url_genius(&"The Neighbourhood".to_string());
    save_artist_profile_url(&"Kanye West".to_string());
    // create_all_tables();
    // // clear_all_tables();
    // let path = "C:/Users/drale/Music/music/FavoritesPL".to_string();
    // populate_database(path);
}