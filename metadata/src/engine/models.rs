// This file will hold all the different tables as models
// We'll have structs for each table and a compile method which turns the struct into an sql string

use once_cell::sync::Lazy;
use std::collections::HashMap;


// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                          TABLES
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------


pub struct Column {
    name: &'static str,
    data_type: &'static str,
    primary_key: bool,
    auto_increment: bool,
    notes: &'static str,
}

pub struct Table {
    name: &'static str,
    // make columns an array of the Column struct that is NOT a vector
    columns: Vec<Column>,
}



// make public function compile_table which takes a table and returns a valid SQL string for creating the table

pub fn compile_table(table: &Table) -> String {
    let mut sql_string = String::from("CREATE TABLE IF NOT EXISTS ");
    sql_string.push_str(table.name);
    sql_string.push_str(" (");
    for column in &table.columns {
        sql_string.push_str(column.name);
        sql_string.push_str(" ");
        sql_string.push_str(column.data_type);
        if column.primary_key {
            sql_string.push_str(" PRIMARY KEY");
        }
        if column.auto_increment {
            sql_string.push_str(" AUTOINCREMENT");
        }
        sql_string.push_str(", ");
    }
    sql_string.pop();
    sql_string.pop();
    sql_string.push_str(");");
    sql_string
}

// now make compiles for each table

pub fn compile_song_table() -> String {
    compile_table(&SONGS)
}

pub fn compile_plays_table() -> String {
    compile_table(&PLAYS)
}   

pub fn compile_playlists_table() -> String {
    compile_table(&PLAYLISTS)
}

pub fn compile_playlist_songs_table() -> String {
    compile_table(&PLAYLIST_SONGS)
}

pub fn compile_song_artists_table() -> String {
    compile_table(&SONG_ARTISTS)
}

pub fn compile_album_artists_table() -> String {
    compile_table(&ALBUM_ARTISTS)
}

pub fn compile_composers_table() -> String {
    compile_table(&COMPOSERS)
}

pub fn compile_genres_table() -> String {
    compile_table(&GENRES)
}

pub fn compile_song_paths_table() -> String {
    compile_table(&SONGPATHS)
}


// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                 INSERTION STRUCTS (HOW THE DATA SHOULD BE FORMATTED TO BE INSERTED INTO THE DATABASE)
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

pub struct SONG_TABLE_DATA  {
    pub song_id: String,
    pub main_artist: String,
    pub filesize_bytes: i64,
    pub padding_bytes: i64,
    pub album_artwork_bit_depth: i64,
    pub album_artwork_colors: i64,
    pub album_artwork_height: i64,
    pub album_artwork_width: i64,
    pub bit_depth: i64,
    pub bitrate: i64,
    pub channels: i64,
    pub duration: f64,
    pub sample_rate_khz: i64,
    pub album: String,
    pub barcode: String,
    pub date_created: String,
    pub disc_number: i64,
    pub disc_total: i64,
    pub isrc: String,
    pub itunesadvisory: String,
    pub length: i64,
    pub publisher: String,
    pub rating: i64,
    pub title: String,
    pub track_number: i64,
    pub track_total: i64,
    pub source: String,
}

pub struct PLAY_TABLE_DATA {
    pub play_id: String,
    pub song_id: String,
    pub song_title: String,
    pub main_artist: String,
    pub filesize_bytes: i64,
    pub start_dt: String,
    pub end_dt: String,
}

pub struct PLAYLIST_TABLE_DATA {
    pub playlist_id: String,
    pub playlist_name: String,
    pub playlist_desc: String,
    pub created_dt: String,
}

pub struct PLAYLIST_SONGS_TABLE_DATA {
    pub playlist_id: String,
    pub song_id: String,
    pub added_dt: String,
}

pub struct SONG_ARTISTS_TABLE_DATA {
    pub artist_name: String,
    pub song_id: String,
    pub dt_added: String,
}

pub struct ALBUM_ARTISTS_TABLE_DATA {
    pub artist_name: String,
    pub song_id: String,
    pub dt_added: String,
}

pub struct COMPOSERS_TABLE_DATA {
    pub composer_name: String,
    pub song_id: String,
    pub dt_added: String,
}

pub struct GENRES_TABLE_DATA {
    pub genre_name: String,
    pub song_id: String,
    pub dt_added: String,
}


pub struct SONGPATHS_TABLE_DATA {
    pub song_id: String,
    pub song_path: String,
}

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                                      BEGIN MODELS
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

// SONG TABLE
// "song_id": "N/A", # string
// "main_artist": "N/A", # string
// "filesize": -1, # in bytes
// "padding": -1, # in bytes
// "album_artwork_bit_depth": -1, # in bits
// "album_artwork_colors": -1, # int
// "album_artwork_height": -1, # in pixels
// "album_artwork_width": -1, # in pixels
// "bit_depth": -1, # in bits
// "bitrate": -1, # in bits, divide by 1000 to get Kbps
// "channels": -1, # int
// "duration": -1, # in seconds
// "sample_rate_khz": -1, # in KHz
// "album": "N/A", # string
// "barcode": "N/A", # string
// "date_created": "N/A", # in YYYY-MM-DD
// "disc_number": -1, # int
// "disc_total": -1, # int
// "isrc": "N/A", # string
// "itunesadvisory": "N/A", # string
// "length": -1, # int
// "publisher": "N/A", # string
// "rating": -1, # int
// "title": "N/A", # string
// "track_number": -1, # int
// "track_total": -1, # int
// "source": "N/A", # string
//     # }
pub static SONGS: Lazy<Table> = Lazy::new(|| Table {
    name: "songs",
    columns: vec![
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: true,
            auto_increment: false,
            notes: "The unique ID of the song",
        },
        Column {
            name: "main_artist",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The main artist of the song",
        },
        Column {
            name: "filesize_bytes",
            data_type: "BIGINT",
            primary_key: false,
            auto_increment: false,
            notes: "The size of the song in bytes",
        },
        Column {
            name: "padding_bytes",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The padding of the song in bytes",
        },
        Column {
            name: "album_artwork_bit_depth",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bit depth of the album artwork in bits",
        },
        Column {
            name: "album_artwork_colors",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The number of colors in the album artwork",
        },
        Column {
            name: "album_artwork_height",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The height of the album artwork in pixels",
        },
        Column {
            name: "album_artwork_width",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The width of the album artwork in pixels",
        },
        Column {
            name: "bit_depth",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bit depth of the song in bits",
        },
        Column {
            name: "bitrate",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bitrate of the song in bits, divide by 1000 to get Kbps",
        },
        Column {
            name: "channels",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The number of channels in the song",
        },
        Column {
            name: "duration",
            data_type: "FLOAT",
            primary_key: false,
            auto_increment: false,
            notes: "The duration of the song in seconds",
        },
        Column {
            name: "sample_rate_khz",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The sample rate of the song in KHz",
        },
        Column {
            name: "album",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The album of the song",
        },
        Column {
            name: "barcode",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The barcode of the song",
        },
        Column {
            name: "date_created",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date the song was created in YYYY-MM-DD",
        },
        Column {
            name: "disc_number",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The disc number of the song",
        },
        Column {
            name: "disc_total",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The total number of discs in the album",
        },
        Column {
            name: "isrc",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ISRC of the song",
        },
        Column {
            name: "itunesadvisory",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The iTunes advisory of the song",
        },
        Column {
            name: "length",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The length of the song",
        },
        Column {
            name: "publisher",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The publisher of the song",
        },
        Column {
            name: "rating",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The rating of the song",
        },
        Column {
            name: "title",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The title of the song",
        },
        Column {
            name: "track_number",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The track number of the song",
        },
        Column {
            name: "track_total",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The total number of tracks in the album",
        },
        Column {
            name: "source",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The source of the song",
        },
    ],
});

// // PLAYS TABLE
// // play_id INTEGER PRIMARY KEY AUTOINCREMENT,
// // song_title TEXT NOT NULL,
// // main_artist TEXT NOT NULL,
// // filesize BIGINT,
// // start_dt TEXT NOT NULL,
// // end_dt TEXT NOT NULL,
// // song_id TEXT NOT NULL

pub static PLAYS: Lazy<Table> = Lazy::new(|| Table {
    name: "plays",
    columns: vec![
        Column {
            name: "play_id",
            data_type: "INTEGER",
            primary_key: true,
            auto_increment: true,
            notes: "The unique ID of the play",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "song_title",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The title of the song",
        },
        Column {
            name: "main_artist",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The primary artist of the song",
        },
        Column {
            name: "filesize",
            data_type: "BIGINT",
            primary_key: false,
            auto_increment: false,
            notes: "The size of the song in bytes",
        },
        Column {
            name: "start_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The start date and time of the play in YYYY-MM-DD HH:MM:SS",
        },
        Column {
            name: "end_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The end date and time of the play in YYYY-MM-DD HH:MM:SS",
        }
    ],
});

// // PLAYLISTS TABLE
// // playlist_id INTEGER PRIMARY KEY AUTOINCREMENT,
// // playlist_name TEXT NOT NULL,
// // playlist_desc TEXT,
// // created_dt TEXT NOT NULL

pub static PLAYLISTS: Lazy<Table> = Lazy::new(|| Table {
    name: "playlists",
    columns: vec![
        Column {
            name: "playlist_id",
            data_type: "INTEGER",
            primary_key: true,
            auto_increment: true,
            notes: "The unique ID of the playlist",
        },
        Column {
            name: "playlist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the playlist",
        },
        Column {
            name: "playlist_desc",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The description of the playlist",
        },
        Column {
            name: "created_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the playlist was created in YYYY-MM-DD HH:MM:SS",
        },
    ],
});

// // PLAYLIST_SONGS TABLE
// // playlist_id INTEGER NOT NULL,
// // song_id TEXT NOT NULL,
// // added_dt TEXT NOT NULL

pub static PLAYLIST_SONGS: Lazy<Table> = Lazy::new(|| Table {   
    name: "playlist_songs",
    columns: vec![
        Column {
            name: "playlist_id",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the playlist",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "added_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the song was added to the playlist in YYYY-MM-DD HH:MM:SS",
        },
    ],
});

// // SONG ARTISTS TABLE
// // artist_name TEXT NOT NULL,
// // song_id TEXT NOT NULL,
// // dt_added TEXT NOT NULL

pub static SONG_ARTISTS: Lazy<Table> = Lazy::new(|| Table {
    name: "song_artists",
    columns: vec![
        Column {
            name: "artist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the artist",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the artist was added to the song in YYYY-MM-DD HH:MM:SS",
        },
    ],
});

// // ALBUM ARTISTS TABLE
// // artist_name TEXT NOT NULL,
// // song_id TEXT NOT NULL,
// // dt_added TEXT NOT NULL

pub static ALBUM_ARTISTS: Lazy<Table> = Lazy::new(|| Table {
    name: "album_artists",
    columns: vec![
        Column {
            name: "artist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the artist",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the artist was added to the song in YYYY-MM-DD HH:MM:SS",
        },
    ],
});

// // COMPOSERS TABLE
// // composer_name TEXT NOT NULL,
// // song_id TEXT NOT NULL,
// // dt_added TEXT NOT NULL

pub static COMPOSERS: Lazy<Table> = Lazy::new(|| Table {
    name: "composers",
    columns: vec![
        Column {
            name: "composer_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the composer",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the composer was added to the song in YYYY-MM-DD HH:MM:SS",
        },
    ],
});


// // GENRES TABLE
// // genre_name TEXT NOT NULL,
// // song_id TEXT NOT NULL,
// // dt_added TEXT NOT NULL

pub static GENRES: Lazy<Table> = Lazy::new(|| Table {
    name: "genres",
    columns: vec![
        Column {
            name: "genre_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the genre",
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the genre was added to the song in YYYY-MM-DD HH:MM:SS",
        },
    ],
});


// // SONGPATHS TABLE
// // song_id TEXT NOT NULL,
// // song_path TEXT NOT NULL,

pub static SONGPATHS: Lazy<Table> = Lazy::new(|| Table {
    name: "songpaths",
    columns: vec![
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
        },
        Column {
            name: "song_path",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The path to the song",
        },
    ],
});

// make fn generate_insertion_sql that takes a table and returns the SQL for inserting into that table
// for example "INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate_khz, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)";
// static SONGS: Lazy<Table> = Lazy::new(|| Table {
pub fn generate_insertion_sql(table: &Table) -> String {
    let mut sql = String::from("INSERT INTO ");
    sql.push_str(&table.name);
    sql.push_str(" (");
    for column in &table.columns {
        sql.push_str(&column.name);
        sql.push_str(", ");
    }
    sql.pop();
    sql.pop();
    sql.push_str(") VALUES (");
    for _ in &table.columns {
        sql.push_str("?, ");

    }
    sql.pop();
    sql.pop();
    sql.push_str(")");
    sql
}