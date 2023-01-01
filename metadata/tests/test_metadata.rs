use decibl_metadata::engine::{
    audio_metadata::{
        add_symphonia_data, file_to_hash, get_symphonia_data, string_to_hash, AudioFileFLAC, AudioFile,
    },
    config::get_soundfiles_path_1,
    models::default,
};


#[cfg(test)]
// RUN cargo test --tests -- --nocapture
use serial_test::serial;

#[test]
#[serial]
fn test_sanity() {
    assert_eq!(1, 1);
}

/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                              testing misc functions                                                              */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */

#[test]
#[serial]
fn test_file_to_hash() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());
    let hash = file_to_hash(filepath).unwrap();

    assert_eq!(
        hash,
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E"
    );
}

#[test]
#[serial]
fn test_string_to_hash() {
    let strin = "hello world";
    let hash = string_to_hash(strin.to_string()).unwrap();

    assert_eq!(
        hash,
        "B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"
    );
}

/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                              testing flac files                                                              */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */

#[test]
#[serial]
fn test_symphonia_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());
    let filehint = "flac".to_string();

    let data = add_symphonia_data(filepath, filehint);

    // we need to make sure the file has these values!

    let valid_artists = vec!["brakence", "Majent"];
    let valid_composers = vec!["Majent", "Randy Findell"];

    assert_eq!(data.get("ARTIST").unwrap(), &valid_artists);
    assert_eq!(data.get("COMPOSER").unwrap(), &valid_composers);
    assert_eq!(data.get("ALBUM").unwrap(), &vec!["punk2"]);
    assert_eq!(data.get("ALBUMARTIST").unwrap(), &vec!["brakence"]);
    assert_eq!(
        data.get("TITLE").unwrap(),
        &vec!["brakence 2.0 freestyle (feat. Majent)"]
    );
    assert_eq!(data.get("GENRE").unwrap(), &vec!["Pop"]);
    assert_eq!(data.get("DATE").unwrap(), &vec!["2020-07-01"]);
    assert_eq!(data.get("TRACKNUMBER").unwrap(), &vec!["10"]);
    assert_eq!(data.get("TRACKTOTAL").unwrap(), &vec!["11"]);
    assert_eq!(data.get("DISCNUMBER").unwrap(), &vec!["1"]);
    assert_eq!(data.get("DISCTOTAL").unwrap(), &vec!["2"]);
    assert_eq!(data.get("LENGTH").unwrap(), &vec!["161000"]);
    assert_eq!(data.get("BARCODE").unwrap(), &vec!["886448554691"]);
    assert_eq!(data.get("ISRC").unwrap(), &vec!["USSM12003816"]);
    assert_eq!(data.get("PUBLISHER").unwrap(), &vec!["Columbia"]);
    assert_eq!(data.get("SOURCE").unwrap(), &vec!["Deezer"]);
    assert_eq!(data.get("SOURCEID").unwrap(), &vec!["1003970742"]);
    assert_eq!(data.get("ITUNESADVISORY").unwrap(), &vec!["1"]);
    assert_eq!(data.get("album_artwork_height").unwrap(), &vec!["800"]);
    assert_eq!(data.get("album_artwork_width").unwrap(), &vec!["800"]);
    assert_eq!(data.get("album_artwork_bit_depth").unwrap(), &vec!["24"]);
}

#[test]
#[serial]
fn test_metaflac_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.add_metaflac_data(filepath);

    // print afile.raw_metadata 
    // {"duration": ["161"], "channels": ["2"], "sample_rate": ["44100"], "bit_depth": ["16"], "bitrate": ["1411200"]}

    assert_eq!(afile.raw_metadata.get("duration").unwrap(), &vec!["161"]);
    assert_eq!(afile.raw_metadata.get("channels").unwrap(), &vec!["2"]);
    assert_eq!(afile.raw_metadata.get("sample_rate").unwrap(), &vec!["44100"]);
    assert_eq!(afile.raw_metadata.get("bit_depth").unwrap(), &vec!["16"]);
    assert_eq!(afile.raw_metadata.get("bitrate").unwrap(), &vec!["1411200"]);


}

#[test]
#[serial]
fn test_flac_get_composers_table_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    let data = afile.get_composers_table_data();
// [COMPOSERS_TABLE_DATA { composer_name: "Majent", song_id: "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E", dt_added: "2022-12-31 10:04:06.264219900" }, COMPOSERS_TABLE_DATA { composer_name: "Randy Findell", song_id: "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E", dt_added: "2022-12-31 10:04:06.264245700" }]

    let valid_composers = vec!["Majent", "Randy Findell"];
    let valid_ids = vec![
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E",
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E",
    ];

    for (i, composer) in data.iter().enumerate() {
        assert_eq!(composer.composer_name, valid_composers[i]);
        assert_eq!(composer.song_id, valid_ids[i]);
    }
    
}

#[test]
#[serial]
fn test_flac_get_genres_table_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    let data = afile.get_genres_table_data();
    
    // genre is Pop
    assert_eq!(data[0].genre_name, "Pop");
}

#[test]
#[serial]
fn test_flac_get_album_artists_table_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    let data = afile.get_album_artists_table_data();
    
    // artist is brakence
    assert_eq!(data[0].artist_name, "brakence");
}

#[test]
#[serial]
fn test_flac_get_song_artists_table_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    let data = afile.get_song_artists_table_data();
    
    // artist is brakence
    assert_eq!(data[0].artist_name, "brakence");
}

#[test]
#[serial]
fn test_get_song_table_data() {
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    let song = afile.get_song_table_data();

    assert_eq!(
        song.song_id,
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E".to_string()
    );
    assert_eq!(song.main_artist, "brakence".to_string());
    assert_eq!(song.filesize_bytes, 15297020);
    assert_eq!(song.padding_bytes, -1);
    assert_eq!(song.album_artwork_bit_depth, 24);
    assert_eq!(song.album_artwork_colors, -1);
    assert_eq!(song.album_artwork_height, 800);
    assert_eq!(song.album_artwork_width, 800);
    assert_eq!(song.bit_depth, 16);
    assert_eq!(song.bitrate, 1411200);
    assert_eq!(song.channels, 2);
    assert_eq!(song.duration, 161.0);
    assert_eq!(song.sample_rate, 44100);
    assert_eq!(song.album, "punk2".to_string());
    assert_eq!(song.barcode, "886448554691".to_string());
    assert_eq!(song.date_created, "2020-07-01".to_string());
    assert_eq!(song.disc_number, 1);
    assert_eq!(song.disc_total, 2);
    assert_eq!(song.isrc, "USSM12003816".to_string());
    assert_eq!(song.itunesadvisory, "1".to_string());
    assert_eq!(song.length, 161000);
    assert_eq!(song.publisher, "Columbia".to_string());
    assert_eq!(song.rating, -1);
    assert_eq!(
        song.title,
        "brakence 2.0 freestyle (feat. Majent)".to_string()
    );
    assert_eq!(song.track_number, 10);
    assert_eq!(song.track_total, 11);
    assert_eq!(song.source, "Deezer".to_string());
    assert_eq!(song.filetype, "flac".to_string());
}
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                              testing mp3 files                                                              */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */

