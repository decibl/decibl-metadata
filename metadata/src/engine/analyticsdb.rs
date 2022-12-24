use rusqlite::params;
use rusqlite::{Connection};
use crate::engine::config::*;
use crate::engine::models::*;
use crate::engine::audio_metadata::*;
use std::collections::HashMap;

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           CREATE TABLES
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

/// Creates a table in the SQLite database with the given `sql_query`.
///
/// # Examples
///
/// ```
/// use rusqlite::{Connection, Result};
///
/// let conn = Connection::open("my_database.db")?;
/// let sql_query = "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL)";
/// create_table(sql_query);
/// ```
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

pub fn clear_table(table_name: String) {
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            let sql_query = format!("DELETE FROM {}", table_name);
            conn.execute(&sql_query, []).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
/// Creates the 'songs' table in the SQLite database.
pub fn create_song_table() {
    let song_sql_query = compile_song_table();
    create_table(song_sql_query)
}

/// Creates the 'plays' table in the SQLite database.
pub fn create_plays_table() {
    let plays_sql_query = compile_plays_table();
    create_table(plays_sql_query)
}

/// Creates the 'playlists' table in the SQLite database.
pub fn create_playlist_table() {
    let playlist_sql_query = compile_playlists_table();
    create_table(playlist_sql_query)
}

/// Creates the 'playlist_songs' table in the SQLite database.
pub fn create_playlist_songs_table() {
    let playlist_song_sql_query = compile_playlist_songs_table();
    create_table(playlist_song_sql_query)
}

/// Creates the 'song_artists' table in the SQLite database.
pub fn create_song_artists_table() {
    let song_artists_sql_query = compile_song_artists_table();
    create_table(song_artists_sql_query)
}

/// Creates the 'album_artists' table in the SQLite database.
pub fn create_album_artists_table() {
    let album_artists_sql_query = compile_album_artists_table();
    create_table(album_artists_sql_query)
}

/// Creates the 'composers' table in the SQLite database.
pub fn create_composers_table() {
    let composers_sql_query = compile_composers_table();
    create_table(composers_sql_query)
}

/// Creates the 'genres' table in the SQLite database.
pub fn create_genres_table() {
    let genres_sql_query = compile_genres_table();
    create_table(genres_sql_query)
}

/// Creates the 'song_paths' table in the SQLite database.
pub fn create_song_paths_table() {
    let song_paths_sql_query = compile_song_paths_table();
    create_table(song_paths_sql_query)
}

pub fn create_artists_table() {
    let artists_sql_query = compile_artists_table();
    create_table(artists_sql_query)
}

pub fn create_albums_table() {
    let albums_sql_query = compile_albums_table();
    create_table(albums_sql_query)
}

pub fn create_all_tables() {
    create_song_table();
    create_plays_table();
    create_playlist_table();
    create_playlist_songs_table();
    create_song_artists_table();
    create_album_artists_table();
    create_composers_table();
    create_genres_table();
    create_song_paths_table();
    create_artists_table();
    create_albums_table();
}

pub fn clear_all_tables() {
    clear_table("songs".to_string());
    clear_table("plays".to_string());
    clear_table("playlists".to_string());
    clear_table("playlist_songs".to_string());
    clear_table("song_artists".to_string());
    clear_table("album_artists".to_string());
    clear_table("composers".to_string());
    clear_table("genres".to_string());
    clear_table("songpaths".to_string());
    clear_table("artists".to_string());
    clear_table("albums".to_string());
}
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           INSERT DATA
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

/// Inserts a song into the 'songs' table in the SQLite database.
/// 
/// Has to be compatible with this hashmap `pub static SONG_TABLE_DATA : Lazy<HashMap<&'static str, &'static str>>`
///
/// # Examples
///
/// ```
/// use rusqlite::{Connection, Result};
///
/// let conn = Connection::open("my_database.db")?;
/// let song_table_data = SONG_TABLE_DATA {
///     song_id: 1,
///     main_artist: "John Doe",
///     filesize_bytes: 1024,
///     padding_bytes: 0,
///     album_artwork_bit_depth: 24,
///     album_artwork_colors: 256,
///     album_artwork_height: 300,
///     album_artwork_width: 300,
///     bit_depth: 16,
///     bitrate: 128,
///     channels: 2,
///     duration: 180,
///     sample_rate_khz: 44.1,
///     album: "Greatest Hits",
///     barcode: "1234567890",
///     date_created: "2022-01-01",
///     disc_number: 1,
///     disc_total: 1,
///     isrc: "US1234567890",
///     itunesadvisory: "clean",
///     length: "3:00",
///     publisher: "John Doe Music",
///     rating: 5,
///     title: "Best Song Ever",
///     track_number: 1,
///     track_total: 10,
///     source: "CD",
/// };
/// insert_song(song_table_data);
pub fn insert_song(song_table_data: SONG_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    // example query
    // let sql_query = "INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)";
    // INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27)
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
                    song_table_data.sample_rate,
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
                    song_table_data.source,
                    song_table_data.filetype,
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

pub fn insert_artist(artist: ARTISTS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ARTISTS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    artist.artist_name,
                    artist.artist_bio,
                    artist.artist_photo_location,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

pub fn insert_album(album: ALBUMS_TABLE_DATA){
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ALBUMS);
    // println!("{}", sql_query);
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    album.album_id,
                    album.album_name,
                    album.artist_name,
                    album.album_description,
                    album.album_art_location,
                    album.album_release_date,
                ],
            )
            .unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}

/// Important function: Pass in an object with trait AudioFile and it will insert the important information in the following tables:
/// songs, song_artists, album_artists, composers, genres
pub fn insert_song_information<T: AudioFile>(song: T){

    // lets get the important structs
    let song_table_data = song.get_song_table_data();
    let song_artists_table_data = song.get_song_artists_table_data();
    let album_artists_table_data = song.get_album_artists_table_data();
    let composers_table_data = song.get_composers_table_data();
    let genres_table_data = song.get_genres_table_data();

    // insert into the appropriate tables
    insert_song(song_table_data);
    
    for song_artist in song_artists_table_data{
        insert_song_artist(song_artist);
    }

    for album_artist in album_artists_table_data{
        insert_album_artist(album_artist);
    }

    for composer in composers_table_data{
        insert_composer(composer);
    }

    for genre in genres_table_data{
        insert_genre(genre);
    }
}



// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           RETRIEVE DATA 
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

// mass retrieval

pub fn get_all_table_names() -> Vec<String> {
    let mut table_names: Vec<String> = Vec::new();
    // table_names.push("songs".to_string());
    // table_names.push("plays".to_string());
    // table_names.push("playlists".to_string());
    // table_names.push("playlist_songs".to_string());
    // table_names.push("song_artists".to_string());
    // table_names.push("album_artists".to_string());
    // table_names.push("composers".to_string());
    // table_names.push("genres".to_string());
    // table_names.push("songpaths".to_string());
    // table_names.push("artists".to_string());
    // table_names.push("albums".to_string());
    table_names.push(SONGS.name.to_string());
    table_names.push(PLAYS.name.to_string());
    table_names.push(PLAYLISTS.name.to_string());
    table_names.push(PLAYLIST_SONGS.name.to_string());
    table_names.push(SONG_ARTISTS.name.to_string());
    table_names.push(ALBUM_ARTISTS.name.to_string());
    table_names.push(COMPOSERS.name.to_string());
    table_names.push(GENRES.name.to_string());
    table_names.push(SONGPATHS.name.to_string());
    table_names.push(ARTISTS.name.to_string());
    table_names.push(ALBUMS.name.to_string());
    table_names
}

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
                    sample_rate: row.get(12).unwrap(),
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
                    filetype: row.get(27).unwrap(),
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

pub fn get_all_playlists() -> Vec<PLAYLIST_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut playlists: Vec<PLAYLIST_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYLISTS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let playlist_iter = stmt.query_map([], |row| {
                Ok(PLAYLIST_TABLE_DATA {
                    playlist_id: row.get(0).unwrap(),
                    playlist_name: row.get(1).unwrap(),
                    playlist_desc: row.get(2).unwrap(),
                    created_dt: row.get(3).unwrap(),
                })
            }).unwrap();
            for playlist in playlist_iter {
                playlists.push(playlist.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    playlists
}

pub fn get_all_playlist_songs() -> Vec<PLAYLIST_SONGS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut playlist_songs: Vec<PLAYLIST_SONGS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYLIST_SONGS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let playlist_song_iter = stmt.query_map([], |row| {
                Ok(PLAYLIST_SONGS_TABLE_DATA {
                    playlist_id: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    added_dt: row.get(2).unwrap(),
                })
            }).unwrap();
            for playlist_song in playlist_song_iter {
                playlist_songs.push(playlist_song.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    playlist_songs
}

pub fn get_all_song_artists() -> Vec<SONG_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut song_artists: Vec<SONG_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONG_ARTISTS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let song_artist_iter = stmt.query_map([], |row| {
                Ok(SONG_ARTISTS_TABLE_DATA {
                    artist_name: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    dt_added: row.get(2).unwrap(),
                })
            }).unwrap();
            for song_artist in song_artist_iter {
                song_artists.push(song_artist.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    song_artists
}

pub fn get_all_album_artists() -> Vec<ALBUM_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut album_artists: Vec<ALBUM_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ALBUM_ARTISTS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let album_artist_iter = stmt.query_map([], |row| {
                Ok(ALBUM_ARTISTS_TABLE_DATA {
                    artist_name: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    dt_added: row.get(2).unwrap(),
                })
            }).unwrap();
            for album_artist in album_artist_iter {
                album_artists.push(album_artist.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    album_artists
}

pub fn get_all_composers() -> Vec<COMPOSERS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut composers: Vec<COMPOSERS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&COMPOSERS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let composer_iter = stmt.query_map([], |row| {
                Ok(COMPOSERS_TABLE_DATA {
                    composer_name: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    dt_added: row.get(2).unwrap(),
                })
            }).unwrap();
            for composer in composer_iter {
                composers.push(composer.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    composers
}

pub fn get_all_genres() -> Vec<GENRES_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut genres: Vec<GENRES_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&GENRES);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let genre_iter = stmt.query_map([], |row| {
                Ok(GENRES_TABLE_DATA {
                    genre_name: row.get(0).unwrap(),
                    song_id: row.get(1).unwrap(),
                    dt_added: row.get(2).unwrap(),
                })
            }).unwrap();
            for genre in genre_iter {
                genres.push(genre.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    genres
}

pub fn get_all_songpaths() -> Vec<SONGPATHS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut songpaths: Vec<SONGPATHS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONGPATHS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let songpath_iter = stmt.query_map([], |row| {
                Ok(SONGPATHS_TABLE_DATA {
                    song_id: row.get(0).unwrap(),
                    song_path: row.get(1).unwrap(),
                })
            }).unwrap();
            for songpath in songpath_iter {
                songpaths.push(songpath.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    songpaths
}

pub fn get_all_artists() -> Vec<ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut artists: Vec<ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ARTISTS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let artist_iter = stmt.query_map([], |row| {
                Ok(ARTISTS_TABLE_DATA {
                    artist_name: row.get(0).unwrap(),
                    artist_bio: row.get(1).unwrap(),
                    artist_photo_location: row.get(2).unwrap(),
                })
            }).unwrap();
            for artist in artist_iter {
                artists.push(artist.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    artists
}

pub fn get_all_albums() -> Vec<ALBUMS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut albums: Vec<ALBUMS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ALBUMS);
    
    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let album_iter = stmt.query_map([], |row| {
                Ok(ALBUMS_TABLE_DATA {
                    album_id: row.get(0).unwrap(),
                    album_name: row.get(1).unwrap(),
                    artist_name: row.get(2).unwrap(),
                    album_description: row.get(3).unwrap(),
                    album_art_location: row.get(4).unwrap(),
                    album_release_date: row.get(5).unwrap(),
                })
            }).unwrap();
            for album in album_iter {
                albums.push(album.unwrap());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    albums
}

// now some more, useful functions

/// This is going to be used to populate the database with some data.
/// Look into config.get_soundfiles_path() to find all the soundfiles. Iterate through each one, parse their metadata, etc.
pub fn populate_database(){
    return;
}