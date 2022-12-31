use decibl_metadata::engine::{
    analyticsdb::{self, clear_all_tables, create_all_tables, populate_database},
    audio_metadata::{AudioFile, AudioFileFLAC},
    config::{get_soundfiles_path_1, create_all_files},
    models::{
        default, ALBUM_ARTISTS_TABLE_DATA, COMPOSERS_TABLE_DATA, GENRES_TABLE_DATA,
        PLAYLIST_SONGS_TABLE_DATA, PLAYLIST_TABLE_DATA, PLAY_TABLE_DATA, SONGPATHS_TABLE_DATA,
        SONG_ARTISTS_TABLE_DATA, SONG_TABLE_DATA,
    },
};
#[cfg(test)]
// RUN cargo test --tests -- --nocapture
use serial_test::serial;

/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                                      Testing table interactions                                                                     */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */


#[test]
#[serial]
fn test_sanity() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[serial]
fn test_table_creation() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();

    let tables = analyticsdb::get_all_table_names();
    assert_eq!(tables.len(), 11);
}

#[test]
#[serial]
fn test_table_clear() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_song(SONG_TABLE_DATA::default());
    analyticsdb::insert_play(PLAY_TABLE_DATA::default());
    // check that the table is not empty
    let songs = analyticsdb::get_all_songs();
    assert_eq!(songs.len(), 1);

    let plays = analyticsdb::get_all_plays();
    assert_eq!(plays.len(), 1);

    // clear the table
    analyticsdb::clear_all_tables();
    let songs2 = analyticsdb::get_all_songs();
    assert_eq!(songs2.len(), 0);

    let plays2 = analyticsdb::get_all_plays();
    assert_eq!(plays2.len(), 0);
}

// now lets test the insertions
#[test]
#[serial]
fn test_insert_song() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_song(SONG_TABLE_DATA::default());
    let songs = analyticsdb::get_all_songs();

    // get the first song, see if is equal to SONG_TABLE_DATA::default()
    let song = songs.get(0).unwrap();

    assert_eq!(song.song_id, "".to_string());
    assert_eq!(song.main_artist, "".to_string());
    assert_eq!(song.filesize_bytes, -1);
    assert_eq!(song.padding_bytes, -1);
    assert_eq!(song.album_artwork_bit_depth, -1);
    assert_eq!(song.album_artwork_colors, -1);
    assert_eq!(song.album_artwork_height, -1);
    assert_eq!(song.album_artwork_width, -1);
    assert_eq!(song.bit_depth, -1);
    assert_eq!(song.bitrate, -1);
    assert_eq!(song.channels, -1);
    assert_eq!(song.duration, -1.0);
    assert_eq!(song.sample_rate, -1);
    assert_eq!(song.album, "".to_string());
    assert_eq!(song.barcode, "".to_string());
    assert_eq!(song.date_created, "".to_string());
    assert_eq!(song.disc_number, -1);
    assert_eq!(song.disc_total, -1);
    assert_eq!(song.isrc, "".to_string());
    assert_eq!(song.itunesadvisory, "".to_string());
    assert_eq!(song.length, -1);
    assert_eq!(song.publisher, "".to_string());
    assert_eq!(song.rating, -1);
    assert_eq!(song.title, "".to_string());
    assert_eq!(song.track_number, -1);
    assert_eq!(song.track_total, -1);
    assert_eq!(song.source, "".to_string());
    assert_eq!(song.filetype, "".to_string());
}

#[test]
#[serial]
fn test_insert_play() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_play(PLAY_TABLE_DATA::default());

    let plays = analyticsdb::get_all_plays();
    let play = plays.get(0).unwrap();

    assert_eq!(play.play_id, "".to_string());
    assert_eq!(play.song_id, "".to_string());
    assert_eq!(play.song_title, "".to_string());
    assert_eq!(play.main_artist, "".to_string());
    assert_eq!(play.filesize_bytes, -1);
    assert_eq!(play.start_dt, "".to_string());
    assert_eq!(play.end_dt, "".to_string());
}

#[test]
#[serial]
fn test_insert_playlist() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_playlist(PLAYLIST_TABLE_DATA::default());

    let playlists = analyticsdb::get_all_playlists();
    let playlist = playlists.get(0).unwrap();

    assert_eq!(playlist.playlist_id, "".to_string());
    assert_eq!(playlist.playlist_name, "".to_string());
    assert_eq!(playlist.playlist_desc, "".to_string());
    assert_eq!(playlist.created_dt, "".to_string());
}

#[test]
#[serial]
fn test_insert_playlist_song() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_playlist_song(PLAYLIST_SONGS_TABLE_DATA::default());

    let playlist_songs = analyticsdb::get_all_playlist_songs();
    let playlist_song = playlist_songs.get(0).unwrap();

    assert_eq!(playlist_song.playlist_id, "".to_string());
    assert_eq!(playlist_song.song_id, "".to_string());
    assert_eq!(playlist_song.added_dt, "".to_string());
}

#[test]
#[serial]
fn test_insert_song_artist() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_song_artist(SONG_ARTISTS_TABLE_DATA::default());

    let song_artists = analyticsdb::get_all_song_artists();
    let song_artist = song_artists.get(0).unwrap();

    assert_eq!(song_artist.song_id, "".to_string());
    assert_eq!(song_artist.artist_name, "".to_string());
    assert_eq!(song_artist.dt_added, "".to_string());
}

#[test]
#[serial]
fn test_insert_album_artist() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_album_artist(ALBUM_ARTISTS_TABLE_DATA::default());

    let album_artists = analyticsdb::get_all_album_artists();
    let album_artist = album_artists.get(0).unwrap();

    assert_eq!(album_artist.artist_name, "".to_string());
    assert_eq!(album_artist.artist_name, "".to_string());
    assert_eq!(album_artist.dt_added, "".to_string());
}

#[test]
#[serial]
fn test_insert_composer() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_composer(COMPOSERS_TABLE_DATA::default());

    let composers = analyticsdb::get_all_composers();
    let composer = composers.get(0).unwrap();

    assert_eq!(composer.song_id, "".to_string());
    assert_eq!(composer.composer_name, "".to_string());
    assert_eq!(composer.dt_added, "".to_string());
}

#[test]
#[serial]
fn test_insert_genre() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_genre(GENRES_TABLE_DATA::default());

    let genres = analyticsdb::get_all_genres();
    let genre = genres.get(0).unwrap();

    assert_eq!(genre.song_id, "".to_string());
    assert_eq!(genre.genre_name, "".to_string());
    assert_eq!(genre.dt_added, "".to_string());
}

#[test]
#[serial]
fn test_insert_songpaths() {
    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_songpath(SONGPATHS_TABLE_DATA::default());

    let songpaths = analyticsdb::get_all_songpaths();
    let songpath = songpaths.get(0).unwrap();

    assert_eq!(songpath.song_id, "".to_string());
    assert_eq!(songpath.song_path, "".to_string());
}

#[test]
#[serial]
fn test_insert_song_information() {
    // read in TEST_SOUNDFILES_PATH_1 / a.flac
    let filepath = format!("{}/a.flac", get_soundfiles_path_1());

    let mut afile = AudioFileFLAC::default();
    afile.load_file(filepath);

    analyticsdb::create_all_tables();
    analyticsdb::clear_all_tables();
    analyticsdb::insert_song_information(afile);

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                             testing song information                                                             */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    // { song_id: "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E", main_artist: "brakence", filesize_bytes: 15297020, padding_bytes: -1, album_artwork_bit_depth: 24, album_artwork_colors: -1, album_artwork_height: 800, album_artwork_width: 800, bit_depth: 16, bitrate: 1411200, channels: 2, duration: 161.0, sample_rate: 44100, album: "punk2", barcode: "886448554691", date_created: "2020-07-01", disc_number: 1, disc_total: 2, isrc: "USSM12003816", itunesadvisory:
    // "1", length: 161000, publisher: "Columbia", rating: -1, title: "brakence 2.0 freestyle (feat. Majent)", track_number: 10, track_total: 11, source: "Deezer", filetype: "flac" }

    let songs = analyticsdb::get_all_songs();
    let song = songs.get(0).unwrap();

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

    let song_artists_table_data = analyticsdb::get_all_song_artists();

    // artist names are brakence and Majent

    for artist in song_artists_table_data {
        assert!(
            artist.artist_name == "brakence".to_string()
                || artist.artist_name == "Majent".to_string()
        );
    }

    let album_artists_table_data = analyticsdb::get_all_album_artists();

    for artist in album_artists_table_data {
        assert_eq!(artist.artist_name, "brakence".to_string());
    }

    let composers_table_data = analyticsdb::get_all_composers();

    for composer in composers_table_data {
        assert!(
            composer.composer_name == "Majent".to_string()
                || composer.composer_name == "Randy Findell".to_string()
        );
    }

    let genres_table_data = analyticsdb::get_all_genres();

    for genre in genres_table_data {
        assert_eq!(genre.genre_name, "Pop".to_string());
    }
}

#[test]
#[serial]
fn test_populate_database() {
    let filepath = get_soundfiles_path_1();
    create_all_tables();
    clear_all_tables();
    populate_database(filepath);

    // now lets check the databases

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                                   testing songs                                                                  */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    let songs = analyticsdb::get_all_songs();

    // make sure the following IDS are in the database
    let song_ids_valid = vec![
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E".to_string(),
        "1300CF6C7AA9AF55B7238C69CD682FF390702B91BA0B90329A2572239BD603FE".to_string(),
        "0583580A97B0FEA4AB47AAC224D96FE9FC9BF71ACC53259DA294281DAC8CFE30".to_string(),
        "19A9F761DD5859057EC591315FBBF557DE91C2893D63F0A4F359851EC54B0A65".to_string(),
        "28A6A52780D7F4AA932F471AFE02B26062EA21D8E69B0D381DC4A703F1A78559".to_string(),
        "673738B05A45D5DE00AE08E007372E45DC941DCD2D386044EAAE3675E928854D".to_string(),
    ];

    // make sure the following filesize_bytes are in the database
    let filesize_bytes_valid = vec![15297020, 2973802, 24633586, 18797307, 10962548, 15063064];

    // make sure the following bitrate is right
    let bitrate_valid = vec![1411200, 128000];

    // make sure the following duration is right:
    let duration_valid = vec![161.0, 171.0, 230.0, 172.0, 120.0, 137.0];

    // make sure following album is right:
    let album_valid = vec![
        "punk2".to_string(),
        "Satin Panthers".to_string(),
        "Nostalgia".to_string(),
        "bhavana".to_string(),
        "Yuno new releases".to_string(),
    ];

    // make sure following title is right:
    let title_valid = vec![
        "brakence 2.0 freestyle (feat. Majent)".to_string(),
        "Cbat".to_string(),
        "Nostalgia".to_string(),
        "idontcare".to_string(),
        "fwb".to_string(),
        "Put The Money In The Bag".to_string(),
    ];

    for song in songs {
        assert!(song_ids_valid.contains(&song.song_id));
        assert!(filesize_bytes_valid.contains(&song.filesize_bytes));
        assert!(bitrate_valid.contains(&song.bitrate));
        assert!(duration_valid.contains(&song.duration));
        assert!(album_valid.contains(&song.album));
        assert!(title_valid.contains(&song.title));
    }

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                                 Checking song artists                                                                 */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    let artists = analyticsdb::get_all_song_artists();
    // make sure the following artists are in the database
    // brakence
    // Majent
    // Tae
    // Yuno Miles
    // Hudson Mohawke

    let artist_names_valid = vec![
        "brakence".to_string(),
        "Majent".to_string(),
        "Tae".to_string(),
        "Yuno Miles".to_string(),
        "Hudson Mohawke".to_string(),
    ];

    for artist in artists {
        assert!(artist_names_valid.contains(&artist.artist_name));
        assert!(song_ids_valid.contains(&artist.song_id));
    }

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                              Checking album artists                                                              */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    let album_artists = analyticsdb::get_all_album_artists();
    // make sure the following artists are in the database

    let album_artist_names_valid = vec![
        "brakence".to_string(),
        "Tae".to_string(),
        "Yuno Miles".to_string(),
        "Hudson Mohawke".to_string(),
    ];

    // make sure following song_ids are right:
    // 2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E
    // 673738B05A45D5DE00AE08E007372E45DC941DCD2D386044EAAE3675E928854D
    // 1300CF6C7AA9AF55B7238C69CD682FF390702B91BA0B90329A2572239BD603FE
    // 0583580A97B0FEA4AB47AAC224D96FE9FC9BF71ACC53259DA294281DAC8CFE30
    // 19A9F761DD5859057EC591315FBBF557DE91C2893D63F0A4F359851EC54B0A65
    // 28A6A52780D7F4AA932F471AFE02B26062EA21D8E69B0D381DC4A703F1A78559

    let album_artist_song_ids_valid = vec![
        "2916D1AF7C16DF259A98FEC02A984F10CD0AF370A339CE6C40670F14FB364E6E".to_string(),
        "673738B05A45D5DE00AE08E007372E45DC941DCD2D386044EAAE3675E928854D".to_string(),
        "1300CF6C7AA9AF55B7238C69CD682FF390702B91BA0B90329A2572239BD603FE".to_string(),
        "0583580A97B0FEA4AB47AAC224D96FE9FC9BF71ACC53259DA294281DAC8CFE30".to_string(),
        "19A9F761DD5859057EC591315FBBF557DE91C2893D63F0A4F359851EC54B0A65".to_string(),
        "28A6A52780D7F4AA932F471AFE02B26062EA21D8E69B0D381DC4A703F1A78559".to_string(),
    ];

    for artist in album_artists {
        assert!(album_artist_names_valid.contains(&artist.artist_name));
        assert!(album_artist_song_ids_valid.contains(&artist.song_id));

    }

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                              Checking song genres                                                               */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    let genres = analyticsdb::get_all_genres();

    // make sure the following genres are in the database

    let genre_names_valid = vec![
        "Pop".to_string(),
        "Alternative".to_string(),
        "Indie Pop".to_string(),
        "Indie Rock".to_string(),
        "Rap/Hip Hop".to_string(),
    ];

    // use song_ids_valid

    for genre in genres {
        assert!(genre_names_valid.contains(&genre.genre_name));
        assert!(song_ids_valid.contains(&genre.song_id));
    }

    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */
    /*                                                              Checking Composers                                                               */
    /* ------------------------------------------------------------------------------------------------------------------------------------------------ */

    let composers = analyticsdb::get_all_composers();

    // make sure the following composers are in the database
    let composer_names_valid = vec![
        "Majent".to_string(),
        "Randy Findell".to_string(),
        "Taylor Jordan Munck".to_string(),
        "Ron Johnson".to_string(),
    ];

    // use song_ids_valid

    for composer in composers {
        assert!(composer_names_valid.contains(&composer.composer_name));
        assert!(song_ids_valid.contains(&composer.song_id));
    }

}
