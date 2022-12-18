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
    let song_sql_query = compile_songs_table();
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