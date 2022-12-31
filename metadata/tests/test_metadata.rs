use decibl_metadata::engine::{
    audio_metadata::{
        add_symphonia_data, file_to_hash, get_symphonia_data, string_to_hash, AudioFileFLAC,
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



/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                              testing mp3 files                                                              */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
