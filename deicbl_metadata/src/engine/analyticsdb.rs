use rusqlite::params;
use rusqlite::{Connection};
use crate::engine::config::*;
use crate::engine::models::*;
use std::collections::HashMap;

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           CREATE TABLES
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

// make pub fn create_table() which accepts a function that returns a string

pub fn create_table(sql_query: String) {
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&sql_query, []).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_song_table() {
    let song_sql_query = compile_song_table();
    create_table(song_sql_query)
}

pub fn create_plays_table() {
    let plays_sql_query = compile_plays_table();
    create_table(plays_sql_query)
}

pub fn create_playlist_table() {
    let playlist_sql_query = compile_playlists_table();
    create_table(playlist_sql_query)
}

pub fn create_playlist_songs_table() {
    let playlist_song_sql_query = compile_playlist_songs_table();
    create_table(playlist_song_sql_query)
}

pub fn create_song_artists_table() {
    let song_artists_sql_query = compile_song_artists_table();
    create_table(song_artists_sql_query)
}

pub fn create_album_artists_table() {
    let album_artists_sql_query = compile_album_artists_table();
    create_table(album_artists_sql_query)
}

pub fn create_composers_table() {
    let composers_sql_query = compile_composers_table();
    create_table(composers_sql_query)
}

pub fn create_genres_table() {
    let genres_sql_query = compile_genres_table();
    create_table(genres_sql_query)
}

pub fn create_song_paths_table() {
    let song_paths_sql_query = compile_song_paths_table();
    create_table(song_paths_sql_query)
}

pub fn create_all_tables() {
    // check if tables exist
    // if not, create them
    create_song_table();
    create_plays_table();
    create_playlist_table();
    create_playlist_songs_table();
    create_song_artists_table();
    create_album_artists_table();
    create_composers_table();
    create_genres_table();
    create_song_paths_table();
}

pub fn clear_all_tables() {
    let table_names = get_all_table_names();
    for table_name in table_names {
        let conn = Connection::open(get_database_file_path());
        match conn {
            Ok(conn) => {
                conn.execute(&format!("DELETE FROM {}", table_name), [])
                    .unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           INSERT DATA
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

// make function insert_song which inserts a file into a song database.
// has to be compatible with this hashmap pub static SONG_TABLE_DATA : Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {

pub fn insert_song(song_table_data: SONG_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    // example query
    // let sql_query = "INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate_khz, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)";
    // INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate_khz, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27)
    // static SONGS: Lazy<Table> = Lazy::new(|| Table {
    // pub fn generate_insertion_sql(table: Lazy<Table>) -> String {

    let sql_query = generate_insertion_sql(&SONGS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    song_table_data.song_id,
                    song_table_data.main_artist,
                    song_table_data.filesize_bytes,
                    song_table_data.padding_bytes,
                    song_table_data.album_artwork_bit_depth,
                    song_table_data.album_artwork_colors,
                    song_table_data.album_artwork_height,
                    song_table_data.album_artwork_width,
                    song_table_data.bit_depth,
                    song_table_data.bitrate,
                    song_table_data.channels,
                    song_table_data.duration,
                    song_table_data.sample_rate_khz,
                    song_table_data.album,
                    song_table_data.barcode,
                    song_table_data.date_created,
                    song_table_data.disc_number,
                    song_table_data.disc_total,
                    song_table_data.isrc,
                    song_table_data.itunesadvisory,
                    song_table_data.length,
                    song_table_data.publisher,
                    song_table_data.rating,
                    song_table_data.title,
                    song_table_data.track_number,
                    song_table_data.track_total,
                    song_table_data.source
                ],
            )
            .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn insert_play(plays: PLAY_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    plays.play_id,
                    plays.song_id,
                    plays.song_title,
                    plays.main_artist,
                    plays.filesize_bytes,
                    plays.start_dt,
                    plays.end_dt
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn insert_playlist(playlist: PLAYLIST_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYLISTS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    playlist.playlist_id,
                    playlist.playlist_name,
                    playlist.playlist_desc,
                    playlist.created_dt,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_playlist_song(playlist_song: PLAYLIST_SONGS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYLIST_SONGS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    playlist_song.playlist_id,
                    playlist_song.song_id,
                    playlist_song.added_dt,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_song_artist(song_artist: SONG_ARTISTS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&SONG_ARTISTS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    song_artist.song_id,
                    song_artist.artist_name,
                    song_artist.dt_added,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_album_artist(album_artist: ALBUM_ARTISTS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ALBUM_ARTISTS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    album_artist.song_id,
                    album_artist.artist_name,
                    album_artist.dt_added,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_composer(composer: COMPOSERS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&COMPOSERS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    composer.song_id,
                    composer.composer_name,
                    composer.dt_added,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_genre(genre: GENRES_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&GENRES);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    genre.song_id,
                    genre.genre_name,
                    genre.dt_added,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_songpath(songpath: SONGPATHS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&SONGPATHS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    songpath.song_id,
                    songpath.song_path,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}


// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           RETRIEVE DATA
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

pub fn get_all_table_names() -> Vec<String> {
    let mut table_names: Vec<String> = Vec::new();
    table_names.push("songs".to_string());
    table_names.push("plays".to_string());
    table_names.push("playlists".to_string());
    table_names.push("playlist_songs".to_string());
    table_names.push("song_artists".to_string());
    table_names.push("album_artists".to_string());
    table_names.push("composers".to_string());
    table_names.push("genres".to_string());
    table_names.push("songpaths".to_string());
    table_names
}

// make queries to get all data from a table

pub fn get_all_songs() -> Vec<SONG_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut songs: Vec<SONG_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONGS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let song_iter = stmt.query_map([], |row| {
                Ok(SONG_TABLE_DATA {
                    song_id: row.get(0).unwrap(),
                    main_artist: row.get(1).unwrap(),
                    filesize_bytes: row.get(2).unwrap(),
                    padding_bytes: row.get(3).unwrap(),
                    album_artwork_bit_depth: row.get(4).unwrap(),
                    album_artwork_colors: row.get(5).unwrap(),
                    album_artwork_height: row.get(6).unwrap(),
                    album_artwork_width: row.get(7).unwrap(),
                    bit_depth: row.get(8).unwrap(),
                    bitrate: row.get(9).unwrap(),
                    channels: row.get(10).unwrap(),
                    duration: row.get(11).unwrap(),
                    sample_rate_khz: row.get(12).unwrap(),
                    album: row.get(13).unwrap(),
                    barcode: row.get(14).unwrap(),
                    date_created: row.get(15).unwrap(),
                    disc_number: row.get(16).unwrap(),
                    disc_total: row.get(17).unwrap(),
                    isrc: row.get(18).unwrap(),
                    itunesadvisory: row.get(19).unwrap(),
                    length: row.get(20).unwrap(),
                    publisher: row.get(21).unwrap(),
                    rating: row.get(22).unwrap(),
                    title: row.get(23).unwrap(),
                    track_number: row.get(24).unwrap(),
                    track_total: row.get(25).unwrap(),
                    source: row.get(26).unwrap(),
                })
            }).unwrap();
            for song in song_iter {
                songs.push(song.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    songs
}

pub fn get_all_plays() -> Vec<PLAY_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut plays: Vec<PLAY_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let play_iter = stmt.query_map([], |row| {
                Ok(PLAY_TABLE_DATA {
                    play_id: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    song_title: row.get(2).unwrap(),
                    main_artist: row.get(3).unwrap(),
                    filesize_bytes: row.get(4).unwrap(),
                    start_dt: row.get(5).unwrap(),
                    end_dt: row.get(6).unwrap(),

                })
            }).unwrap();
            for play in play_iter {
                plays.push(play.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    plays
}