use crate::engine::audio_metadata::*;
use crate::engine::config::*;
use crate::engine::models::*;
use indicatif::ProgressBar;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use rusqlite::params;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fmt::Write;
use walkdir;
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

/// Clears the table with the given `table_name`.
/// # Examples
/// ```
/// use rusqlite::{Connection, Result};
/// let conn = Connection::open("my_database.db")?;
/// let table_name = "users";
/// clear_table(table_name);
/// ```
pub fn clear_table(table_name: String) {
    let conn = Connection::open(get_database_file_path());

    // drop the table
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
    create_table(song_paths_sql_query);
}

/// Creates the 'artists' table in the SQLite database.
pub fn create_artists_table() {
    let artists_sql_query = compile_artists_table();
    create_table(artists_sql_query)
}

/// Creates the 'albums' table in the SQLite database.
pub fn create_albums_table() {
    let albums_sql_query = compile_albums_table();
    create_table(albums_sql_query)
}

/// Creates all the tables in the SQLite database.
pub fn create_all_tables() {
    create_all_files();
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

/// Clears all the tables in the SQLite database.
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
/// ```
pub fn insert_song(song_table_data: SONG_TABLE_DATA) {
    let mut conn = Connection::open(get_database_file_path());

    // example query
    // let sql_query = "INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)";
    // INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27, ?27)
    // static SONGS: Lazy<Table> = Lazy::new(|| Table {
    // pub fn generate_insertion_sql(table: Lazy<Table>) -> String {

    // check if song_id already exists
    // if it does, dont dod anything

    let sql_query = generate_insertion_sql(&SONGS);
    
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

/// Insert a new play into the database
/// ```
/// use rusqlite::{Connection, Result};
/// use rusqlite::params;
/// use crate::database::get_database_file_path;
/// use crate::database::PLAY_TABLE_DATA;
/// use crate::database::insert_play;
/// use crate::database::PLAYS;
/// use crate::database::generate_insertion_sql;
///
/// let play_table_data = PLAY_TABLE_DATA {
///    play_id: "1234567890",
///    song_id: "1234567890",
///    song_title: "Best Song Ever",
///   main_artist: "John Doe",
///   filesize_bytes: 1234567890,
///  start_dt: "2022-01-01",
/// end_dt: "2022-01-01",
/// };
///
/// insert_play(play_table_data);
/// ```
pub fn insert_play(plays: PLAY_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYS);
    
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

// OK NO MORE USE BULLSHIT IN THESE DOCSTRINGS ITS GETTING OLD >:(

/// Insert a new playlist into the database
/// ```
/// let playlist_table_data = PLAYLIST_TABLE_DATA {
///   playlist_id: "1234567890",
///  playlist_name: "Best Playlist Ever",
/// playlist_desc: "Best Playlist Ever",
/// created_dt: "2022-01-01",
/// };
///
/// insert_playlist(playlist_table_data);
/// ```
pub fn insert_playlist(playlist: PLAYLIST_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYLISTS);
    
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

/// Insert a new playlist_song into the database. Make sure the playlist_id and song_id is real.
/// ```
/// let playlist_song_table_data = PLAYLIST_SONGS_TABLE_DATA {
///  playlist_id: "1234567890",
/// song_id: "1234567890",
/// added_dt: "2022-01-01",
/// };
///     
/// insert_playlist_song(playlist_song_table_data);
/// ```
pub fn insert_playlist_song(playlist_song: PLAYLIST_SONGS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&PLAYLIST_SONGS);
    
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

/// Insert a new song_artist into the database. Make sure the song_id is real.
/// ```
/// let song_artist_table_data = SONG_ARTISTS_TABLE_DATA {
/// artist_name: "John Doe",
/// song_id: "1234567890",
/// dt_added: "2022-01-01",
/// };
///
/// insert_song_artist(song_artist_table_data);
/// ```
pub fn insert_song_artist(song_artist: SONG_ARTISTS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&SONG_ARTISTS);
    
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    song_artist.artist_name,
                    song_artist.song_id,
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

/// Insert a new album_artist into the database. Make sure the song_id is real.
/// ```
/// let album_artist_table_data = ALBUM_ARTISTS_TABLE_DATA {
/// artist_name: "John Doe",
/// song_id: "1234567890",
/// dt_added: "2022-01-01",
/// };
///
/// insert_album_artist(album_artist_table_data);
/// ```
pub fn insert_album_artist(album_artist: ALBUM_ARTISTS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ALBUM_ARTISTS);
    
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![
                    album_artist.artist_name,
                    album_artist.song_id,
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

/// Insert a new composer into the database. Make sure the song_id is real.
/// ```
/// let composer_table_data = COMPOSERS_TABLE_DATA {
/// composer_name: "John Doe",
/// song_id: "1234567890",
/// dt_added: "2022-01-01",
/// };
///
/// insert_composer(composer_table_data);
/// ```
pub fn insert_composer(composer: COMPOSERS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&COMPOSERS);
    
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![composer.composer_name, composer.song_id, composer.dt_added,],
            )
            .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/// Insert a new genre into the database. Make sure the song_id is real.
/// ```
/// let genre_table_data = GENRES_TABLE_DATA {
/// genre_name: "John Doe",
/// song_id: "1234567890",
/// dt_added: "2022-01-01",
/// };
///
/// insert_genre(genre_table_data);
/// ```
pub fn insert_genre(genre: GENRES_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&GENRES);
    
    match conn {
        Ok(conn) => {
            conn.execute(
                &sql_query,
                params![genre.genre_name, genre.song_id, genre.dt_added,],
            )
            .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/// Insert a new songpath into the database. Make sure the song_id is real.
/// ```
/// let songpath_table_data = SONGPATHS_TABLE_DATA {
///     song_id: "1234567890",
///    song_path: "C:\\Users\\John\\Music\\John Doe\\John Doe - John Doe.mp3",
/// };
///
/// insert_songpath(songpath_table_data);
/// ```
pub fn insert_songpath(songpath: SONGPATHS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&SONGPATHS);
    
    match conn {
        Ok(conn) => {
            conn.execute(&sql_query, params![songpath.song_id, songpath.song_path,])
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/// Insert a new artist into the artist database (DIFFERENT FROM SONG AND ALBUM ARTIST). This table is used for caching and rendering stuff fast, so there might be some duplication.
/// ```
/// let artist_table_data = ARTISTS_TABLE_DATA {
///    artist_name: "John Doe",
///   artist_bio: "John Doe is a singer-songwriter from the United States.",
///  artist_photo_location: "C:\\Users\\John\\Pictures\\John Doe.jpg",
/// };
///
/// insert_artist(artist_table_data);
/// ```
pub fn insert_artist(artist: ARTISTS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ARTISTS);
    
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

/// Insert a new album into the database. Make sure the album_id is real.
pub fn insert_album(album: ALBUMS_TABLE_DATA) {
    let conn = Connection::open(get_database_file_path());

    let sql_query = generate_insertion_sql(&ALBUMS);
    
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
/// ```
/// let filepath = "C:\\Users\\John\\Music\\John Doe\\John Doe - John Doe.mp3";
/// let audioFile = AudioFileMP3::default(); // you have to decide which audioFile class to use by default! Use like an if statement or see populate_database() down below
/// audioFile.load_file(filepath);
///
/// insert_song_information(audioFile);
/// ```
pub fn insert_song_information<T: AudioFile>(song: T) {
    // lets get the important structs
    let song_table_data = song.get_song_table_data();
    let song_artists_table_data = song.get_song_artists_table_data();
    let album_artists_table_data = song.get_album_artists_table_data();
    let composers_table_data = song.get_composers_table_data();
    let genres_table_data = song.get_genres_table_data();

    // print composers
    // insert into the appropriate tables
    insert_song(song_table_data);

    for song_artist in song_artists_table_data {
        insert_song_artist(song_artist);
    }

    for album_artist in album_artists_table_data {
        insert_album_artist(album_artist);
    }

    for composer in composers_table_data {
        insert_composer(composer);
    }

    for genre in genres_table_data {
        insert_genre(genre);
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           RETRIEVE DATA MASS
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

// mass retrieval

/// Get all the table names in the database.
pub fn get_all_table_names() -> Vec<String> {
    let mut table_names: Vec<String> = Vec::new();
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

/// Get all the songs in the database.
pub fn get_all_songs() -> Vec<SONG_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut songs: Vec<SONG_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONGS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let song_iter = stmt
                .query_map([], |row| {
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
                })
                .unwrap();
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

/// Get all the plays in the database.
pub fn get_all_plays() -> Vec<PLAY_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut plays: Vec<PLAY_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let play_iter = stmt
                .query_map([], |row| {
                    Ok(PLAY_TABLE_DATA {
                        play_id: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        song_title: row.get(2).unwrap(),
                        main_artist: row.get(3).unwrap(),
                        filesize_bytes: row.get(4).unwrap(),
                        start_dt: row.get(5).unwrap(),
                        end_dt: row.get(6).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the playlists in the database.
pub fn get_all_playlists() -> Vec<PLAYLIST_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut playlists: Vec<PLAYLIST_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYLISTS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let playlist_iter = stmt
                .query_map([], |row| {
                    Ok(PLAYLIST_TABLE_DATA {
                        playlist_id: row.get(0).unwrap(),
                        playlist_name: row.get(1).unwrap(),
                        playlist_desc: row.get(2).unwrap(),
                        created_dt: row.get(3).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the playlist songs in the database.
pub fn get_all_playlist_songs() -> Vec<PLAYLIST_SONGS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut playlist_songs: Vec<PLAYLIST_SONGS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&PLAYLIST_SONGS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let playlist_song_iter = stmt
                .query_map([], |row| {
                    Ok(PLAYLIST_SONGS_TABLE_DATA {
                        playlist_id: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        added_dt: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the song artists in the database.
pub fn get_all_song_artists() -> Vec<SONG_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut song_artists: Vec<SONG_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONG_ARTISTS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let song_artist_iter = stmt
                .query_map([], |row| {
                    Ok(SONG_ARTISTS_TABLE_DATA {
                        artist_name: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        dt_added: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the album artists in the database.
pub fn get_all_album_artists() -> Vec<ALBUM_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut album_artists: Vec<ALBUM_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ALBUM_ARTISTS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let album_artist_iter = stmt
                .query_map([], |row| {
                    Ok(ALBUM_ARTISTS_TABLE_DATA {
                        artist_name: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        dt_added: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the composers in the database.
pub fn get_all_composers() -> Vec<COMPOSERS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut composers: Vec<COMPOSERS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&COMPOSERS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let composer_iter = stmt
                .query_map([], |row| {
                    Ok(COMPOSERS_TABLE_DATA {
                        composer_name: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        dt_added: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the genres in the database.
pub fn get_all_genres() -> Vec<GENRES_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut genres: Vec<GENRES_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&GENRES);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let genre_iter = stmt
                .query_map([], |row| {
                    Ok(GENRES_TABLE_DATA {
                        genre_name: row.get(0).unwrap(),
                        song_id: row.get(1).unwrap(),
                        dt_added: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the song paths in the database.
pub fn get_all_songpaths() -> Vec<SONGPATHS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut songpaths: Vec<SONGPATHS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&SONGPATHS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let songpath_iter = stmt
                .query_map([], |row| {
                    Ok(SONGPATHS_TABLE_DATA {
                        song_id: row.get(0).unwrap(),
                        song_path: row.get(1).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the artists in the database.
pub fn get_all_artists() -> Vec<ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut artists: Vec<ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ARTISTS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let artist_iter = stmt
                .query_map([], |row| {
                    Ok(ARTISTS_TABLE_DATA {
                        artist_name: row.get(0).unwrap(),
                        artist_bio: row.get(1).unwrap(),
                        artist_photo_location: row.get(2).unwrap(),
                    })
                })
                .unwrap();
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

/// Get all the albums in the database.
pub fn get_all_albums() -> Vec<ALBUMS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path());
    let mut albums: Vec<ALBUMS_TABLE_DATA> = Vec::new();

    // make sql query

    let sql_query = generate_select_all_sql(&ALBUMS);

    match conn {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql_query).unwrap();
            let album_iter = stmt
                .query_map([], |row| {
                    Ok(ALBUMS_TABLE_DATA {
                        album_id: row.get(0).unwrap(),
                        album_name: row.get(1).unwrap(),
                        artist_name: row.get(2).unwrap(),
                        album_description: row.get(3).unwrap(),
                        album_art_location: row.get(4).unwrap(),
                        album_release_date: row.get(5).unwrap(),
                    })
                })
                .unwrap();
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

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           RETRIEVE DATA SINGLE (TYPICALLY MORE USEFUL)
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

/// Get a single song from the database.
pub fn get_song_by_id(song_id: String) -> SONG_TABLE_DATA {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut song: SONG_TABLE_DATA = SONG_TABLE_DATA::default();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!("SELECT * FROM SONGS WHERE song_id = '{}'", song_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
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
        })
        .unwrap();

    for row in rows {
        song = row.unwrap();
    }

    song
}

/// Get a single play from the database by its id.
pub fn get_play_by_id(play_id: String) -> PLAY_TABLE_DATA {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut play: PLAY_TABLE_DATA = PLAY_TABLE_DATA::default();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!("SELECT * FROM PLAYS WHERE play_id = '{}'", play_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(PLAY_TABLE_DATA {
                play_id: row.get(0).unwrap(),
                song_id: row.get(1).unwrap(),
                song_title: row.get(2).unwrap(),
                main_artist: row.get(3).unwrap(),
                filesize_bytes: row.get(4).unwrap(),
                start_dt: row.get(5).unwrap(),
                end_dt: row.get(6).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        play = row.unwrap();
    }

    play
}

/// Get a single playlist from the database by its id.
pub fn get_playlist_by_id(playlist_id: String) -> PLAYLIST_TABLE_DATA {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut playlist: PLAYLIST_TABLE_DATA = PLAYLIST_TABLE_DATA::default();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!(
        "SELECT * FROM PLAYLISTS WHERE playlist_id = '{}'",
        playlist_id
    )
    .to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(PLAYLIST_TABLE_DATA {
                playlist_id: row.get(0).unwrap(),
                playlist_name: row.get(1).unwrap(),
                playlist_desc: row.get(2).unwrap(),
                created_dt: row.get(3).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        playlist = row.unwrap();
    }

    playlist
}

/// Get all the playlistt_songs from the database by its id. (NOT SONGS IN PLAYLIST, THAT'S BELOW METHOD)
pub fn get_playlist_songs_by_id(playlist_id: String) -> Vec<PLAYLIST_SONGS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut playlist_songs: Vec<PLAYLIST_SONGS_TABLE_DATA> = Vec::new();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!(
        "SELECT * FROM PLAYLIST_SONGS WHERE playlist_id = '{}'",
        playlist_id
    )
    .to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(PLAYLIST_SONGS_TABLE_DATA {
                playlist_id: row.get(0).unwrap(),
                song_id: row.get(1).unwrap(),
                added_dt: row.get(5).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        playlist_songs.push(row.unwrap());
    }

    playlist_songs
}

/// Get all the songs in a playlist by the playlist_id
pub fn get_songs_in_playlist(playlist_id: String) -> Vec<SONG_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut songs: Vec<SONG_TABLE_DATA> = Vec::new();

    // what we will do is: call get_playlist_songs_by_id and search for where playlist_id = playlist_id
    // then grab all the song_id values and search for where song_id = song_id in the songs table
    // then return the songs

    let playlist_songs = get_playlist_songs_by_id(playlist_id);

    for playlist_song in playlist_songs {
        let song = get_song_by_id(playlist_song.song_id);
        songs.push(song);
    }

    songs
}

/// Get all the song_artists with a given song_id
pub fn get_song_artists_by_song_id(song_id: String) -> Vec<SONG_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut song_artists: Vec<SONG_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!("SELECT * FROM SONG_ARTISTS WHERE song_id = '{}'", song_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(SONG_ARTISTS_TABLE_DATA {
                song_id: row.get(0).unwrap(),
                artist_name: row.get(2).unwrap(),
                dt_added: row.get(3).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        song_artists.push(row.unwrap());
    }

    song_artists
}

/// Get all the album_artists with a given song_id
pub fn get_album_artists_by_song_id(song_id: String) -> Vec<ALBUM_ARTISTS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut album_artists: Vec<ALBUM_ARTISTS_TABLE_DATA> = Vec::new();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query =
        format!("SELECT * FROM ALBUM_ARTISTS WHERE song_id = '{}'", song_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(ALBUM_ARTISTS_TABLE_DATA {
                song_id: row.get(0).unwrap(),
                artist_name: row.get(2).unwrap(),
                dt_added: row.get(3).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        album_artists.push(row.unwrap());
    }

    album_artists
}

/// Get all the genres with a given song_id
pub fn get_genres_by_song_id(song_id: String) -> Vec<GENRES_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut genres: Vec<GENRES_TABLE_DATA> = Vec::new();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!("SELECT * FROM GENRES WHERE song_id = '{}'", song_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(GENRES_TABLE_DATA {
                song_id: row.get(0).unwrap(),
                genre_name: row.get(2).unwrap(),
                dt_added: row.get(3).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        genres.push(row.unwrap());
    }

    genres
}

/// Get all the composers with a given song_id
pub fn get_composers_by_song_id(song_id: String) -> Vec<COMPOSERS_TABLE_DATA> {
    let conn = Connection::open(get_database_file_path()).expect("Could not open database");
    let mut composers: Vec<COMPOSERS_TABLE_DATA> = Vec::new();

    // make sql query

    // let sql_query =  "SELECT * FROM SONGS WHERE song_id = ?1".to_string();
    let sql_query = format!("SELECT * FROM COMPOSERS WHERE song_id = '{}'", song_id).to_string();


    let mut receiver = conn
        .prepare(&sql_query)
        .expect("Could not prepare statement");

    let mut rows = receiver
        .query_map([], |row| {
            Ok(COMPOSERS_TABLE_DATA {
                song_id: row.get(0).unwrap(),
                composer_name: row.get(2).unwrap(),
                dt_added: row.get(3).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        composers.push(row.unwrap());
    }

    composers
}

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           PRIMARY/OTHER FUNCTIONS
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------




/// This function is going to be used to get all the filepaths in a directory, including subdirectories.
pub fn get_all_filepaths_in_directory(dirpath: String) -> Vec<String> {
    let mut filepaths: Vec<String> = Vec::new();

    // we want to get all the filepaths in the directory, including subdirectories
    // we can use the walkdir crate for this
    for entry in walkdir::WalkDir::new(dirpath) {
        let entry = entry.unwrap();
        let filepath = entry.path().to_str().unwrap().to_string();
        filepaths.push(filepath);
    }
    filepaths
}
/// This is going to be used to populate the database with some data.
/// Given a directory, it will go through all the files in the directory, and call the insert_song_information function
pub fn populate_database(dirpath: String) {
    // first, we need to get all the filepaths in the directory
    let filepaths = get_all_filepaths_in_directory(dirpath);
    let songs = get_all_songs();
    let song_ids: Vec<String> = songs.iter().map(|song| song.song_id.clone()).collect();


    let total_files = filepaths.len();

    let bar = ProgressBar::new(total_files as u64);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    // now for each filepath, we need to make sure it's a soundfile, so ends with .mp3 or .flac
    // then we need to parse the metadata, and add it to the database
    for filepath in filepaths {
        // if filepath doesn't end with .mp3 or .flac, then we don't want to parse it

        // hash the filepath

        if !filepath.ends_with(".mp3") && !filepath.ends_with(".flac") {
            continue;
        }


        let fileExt = std::path::Path::new(&filepath)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap()
            .to_string();

        match fileExt.as_str() {
            "mp3" => {
                let mut afile = AudioFileMP3::default();
                afile.load_file(filepath);
                insert_song_information(afile);
            }
            "flac" => {
                let mut afile = AudioFileFLAC::default();
                afile.load_file(filepath);
                insert_song_information(afile);
            }
            _ => {
                println!("File extension not supported");
            }
        }

        bar.inc(1);
    }
    bar.finish();
}
