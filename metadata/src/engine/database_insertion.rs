use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
// use models and config from current directory
use crate::engine::models::*;
use crate::engine::config::*;
use std::path;
use std::fs;

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                           CREATE TABLES
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

pub fn create_song_table(){
    let song_sql_query = compile_song_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&song_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_plays_table(){
    let plays_sql_query = compile_plays_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&plays_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_playlist_table(){
    let playlist_sql_query = compile_playlists_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&playlist_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_playlist_songs_table(){
    let playlist_song_sql_query = compile_playlist_songs_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&playlist_song_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_song_artists_table(){
    let song_artists_sql_query = compile_song_artists_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&song_artists_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_album_artists_table(){
    let album_artists_sql_query = compile_album_artists_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&album_artists_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_composers_table(){
    let composers_sql_query = compile_composers_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&composers_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_genres_table(){
    let genres_sql_query = compile_genres_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&genres_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_song_paths_table(){
    let song_paths_sql_query = compile_song_paths_table();
    let conn = Connection::open(get_database_file_path());

    match conn {
        Ok(conn) => {
            conn.execute(&song_paths_sql_query, NO_PARAMS).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn create_all_tables(){
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

pub fn get_all_table_names() -> Vec<String>{
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

pub fn clear_all_tables(){
    let table_names = get_all_table_names();
    for table_name in table_names {
        let conn = Connection::open(get_database_file_path());
        match conn {
            Ok(conn) => {
                conn.execute(&format!("DELETE FROM {}", table_name), NO_PARAMS).unwrap();
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}