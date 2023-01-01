#![allow(non_camel_case_types)]
// This file will hold all the different tables as models
// We'll have structs for each table and a compile method which turns the struct into an sql string
use once_cell::sync::Lazy;

// This is how it's broken down:
// 1. Make our accessory shit like Columns and Table structs which will be used to make the tables
// 2. Make the format (TABLE_DATA) that our other functions will expect the data to be in so that we can insert it into the database
// 3. Make the actual tables and functions that compile our models into sql strings which we can use to create the tables
// Pretty simple right? right?

// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                          TABLES
// --------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------------------------

pub struct Column {
    pub name: &'static str,
    pub data_type: &'static str,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub notes: &'static str,
    pub is_unique: bool,
}

pub struct Table {
    pub name: &'static str,
    // make columns an array of the Column struct that is NOT a vector
    pub columns: Vec<Column>,
}

// make public function compile_table which takes a table and returns a valid SQL string for creating the table

pub fn compile_table(table: &Table) -> String {
    let mut sql_string = String::from("CREATE TABLE IF NOT EXISTS ");

    // make a vector to store the unique columns
    let mut unique_columns: Vec<&str> = Vec::new();

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
        if column.is_unique {
            unique_columns.push(column.name);
        }
        sql_string.push_str(", ");
    }
    if unique_columns.len() > 0 {
        sql_string.push_str("UNIQUE (");
        for column in unique_columns {
            sql_string.push_str(column);
            sql_string.push_str(", ");
        }
        sql_string.pop();
        sql_string.pop();
        sql_string.push_str("));");
        sql_string
    } else {
        sql_string.pop();
        sql_string.pop();
        sql_string.push_str(");");
        sql_string
    }
}

// now make compiles for each table
// you know, looking at this code reminds me of the internships i've done
// and how utterly lost I would feel if I was someone else having to read this
// and I'm the one who wrote it
//  I'm not sure if I should be proud or ashamed
// LOL

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

pub fn compile_artists_table() -> String {
    compile_table(&ARTISTS)
}

pub fn compile_albums_table() -> String {
    compile_table(&ALBUMS)
}

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                 INSERTION STRUCTS (HOW THE DATA SHOULD BE FORMATTED TO BE INSERTED INTO THE DATABASE)
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

pub trait default {
    fn default() -> Self;
}

// DERIVE DEBUG MEANS WE CAN PRINT IT
#[derive(Debug)]
pub struct SONG_TABLE_DATA {
    pub song_id: String,
    pub main_artist: String,          // yes
    pub filesize_bytes: i64,          // yes *
    pub padding_bytes: i64,           // flack pack yes
    pub album_artwork_bit_depth: i64, // yes
    pub album_artwork_colors: i64,    // yes
    pub album_artwork_height: i64,    //  yes
    pub album_artwork_width: i64,     // yes
    pub bit_depth: i64,               // flac bits_per_sample
    pub bitrate: i64,                 // flac can calculate
    pub channels: i64,                // flac pack
    pub duration: f64,                // can calculate
    pub sample_rate: i64,             // flac pack
    pub album: String,                // yes
    pub barcode: String,              // yes
    pub date_created: String,         // yes
    pub disc_number: i64,             // yes
    pub disc_total: i64,              // yes
    pub isrc: String,                 // yes
    pub itunesadvisory: String,       // yes
    pub length: i64,                  // yes
    pub publisher: String,            // yes
    pub rating: i64,                  // yes
    pub title: String,                // yes
    pub track_number: i64,            // yes
    pub track_total: i64,             // yes
    pub source: String,               // yes
    pub filetype: String,             // yes

                                      // make new function
}

#[derive(Debug)]
pub struct PLAY_TABLE_DATA {
    pub play_id: String,
    pub song_id: String,
    pub song_title: String,
    pub main_artist: String,
    pub filesize_bytes: i64,
    pub start_dt: String,
    pub end_dt: String,
}

#[derive(Debug)]
pub struct PLAYLIST_TABLE_DATA {
    pub playlist_id: String,
    pub playlist_name: String,
    pub playlist_desc: String,
    pub created_dt: String,
}

#[derive(Debug)]
pub struct PLAYLIST_SONGS_TABLE_DATA {
    pub playlist_id: String,
    pub song_id: String,
    pub added_dt: String,
}

#[derive(Debug)]
pub struct SONG_ARTISTS_TABLE_DATA {
    pub artist_name: String,
    pub song_id: String,
    pub dt_added: String,
}

#[derive(Debug)]
pub struct ALBUM_ARTISTS_TABLE_DATA {
    pub artist_name: String,
    pub song_id: String,
    pub dt_added: String,
}

#[derive(Debug)]
pub struct COMPOSERS_TABLE_DATA {
    pub composer_name: String,
    pub song_id: String,
    pub dt_added: String,
}

#[derive(Debug)]
pub struct GENRES_TABLE_DATA {
    pub genre_name: String,
    pub song_id: String,
    pub dt_added: String,
}
#[derive(Debug)]
pub struct SONGPATHS_TABLE_DATA {
    pub song_id: String,
    pub song_path: String,
}
#[derive(Debug)]
pub struct ARTISTS_TABLE_DATA {
    pub artist_name: String,
    pub artist_bio: String,
    pub artist_photo_location: String,
}
#[derive(Debug)]
pub struct ALBUMS_TABLE_DATA {
    pub album_id: String,
    pub album_name: String,
    pub artist_name: String,
    pub album_description: String,
    pub album_art_location: String,
    pub album_release_date: String,
}

impl default for SONG_TABLE_DATA {
    fn default() -> Self {
        SONG_TABLE_DATA {
            song_id: "".to_string(),
            main_artist: "".to_string(),
            filesize_bytes: -1,
            padding_bytes: -1,
            album_artwork_bit_depth: -1,
            album_artwork_colors: -1,
            album_artwork_height: -1,
            album_artwork_width: -1,
            bit_depth: -1,
            bitrate: -1,
            channels: -1,
            duration: -1.0,
            sample_rate: -1,
            album: "".to_string(),
            barcode: "".to_string(),
            date_created: "".to_string(),
            disc_number: -1,
            disc_total: -1,
            isrc: "".to_string(),
            itunesadvisory: "".to_string(),
            length: -1,
            publisher: "".to_string(),
            rating: -1,
            title: "".to_string(),
            track_number: -1,
            track_total: -1,
            source: "".to_string(),
            filetype: "".to_string(),
        }
    }
}

impl default for PLAY_TABLE_DATA {
    fn default() -> Self {
        PLAY_TABLE_DATA {
            play_id: "".to_string(),
            song_id: "".to_string(),
            song_title: "".to_string(),
            main_artist: "".to_string(),
            filesize_bytes: -1,
            start_dt: "".to_string(),
            end_dt: "".to_string(),
        }
    }
}

impl default for PLAYLIST_TABLE_DATA {
    fn default() -> Self {
        PLAYLIST_TABLE_DATA {
            playlist_id: "".to_string(),
            playlist_name: "".to_string(),
            playlist_desc: "".to_string(),
            created_dt: "".to_string(),
        }
    }
}

impl default for PLAYLIST_SONGS_TABLE_DATA {
    fn default() -> Self {
        PLAYLIST_SONGS_TABLE_DATA {
            playlist_id: "".to_string(),
            song_id: "".to_string(),
            added_dt: "".to_string(),
        }
    }
}

impl default for SONG_ARTISTS_TABLE_DATA {
    fn default() -> Self {
        SONG_ARTISTS_TABLE_DATA {
            artist_name: "".to_string(),
            song_id: "".to_string(),
            dt_added: "".to_string(),
        }
    }
}

impl default for ALBUM_ARTISTS_TABLE_DATA {
    fn default() -> Self {
        ALBUM_ARTISTS_TABLE_DATA {
            artist_name: "".to_string(),
            song_id: "".to_string(),
            dt_added: "".to_string(),
        }
    }
}

impl default for COMPOSERS_TABLE_DATA {
    fn default() -> Self {
        COMPOSERS_TABLE_DATA {
            composer_name: "".to_string(),
            song_id: "".to_string(),
            dt_added: "".to_string(),
        }
    }
}

impl default for GENRES_TABLE_DATA {
    fn default() -> Self {
        GENRES_TABLE_DATA {
            genre_name: "".to_string(),
            song_id: "".to_string(),
            dt_added: "".to_string(),
        }
    }
}

impl default for SONGPATHS_TABLE_DATA {
    fn default() -> Self {
        SONGPATHS_TABLE_DATA {
            song_id: "".to_string(),
            song_path: "".to_string(),
        }
    }
}

impl default for ARTISTS_TABLE_DATA {
    fn default() -> Self {
        ARTISTS_TABLE_DATA {
            artist_name: "".to_string(),
            artist_bio: "".to_string(),
            artist_photo_location: "".to_string(),
        }
    }
}

impl default for ALBUMS_TABLE_DATA {
    fn default() -> Self {
        ALBUMS_TABLE_DATA {
            album_id: "".to_string(),
            album_name: "".to_string(),
            artist_name: "".to_string(),
            album_description: "".to_string(),
            album_art_location: "".to_string(),
            album_release_date: "".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
//                                                                      BEGIN MODELS
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

// SONG TABLE
// "song_id": "", # string
// "main_artist": "", # string
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
// "sample_rate": -1, # in hz
// "album": "", # string
// "barcode": "", # string
// "date_created": "", # in YYYY-MM-DD
// "disc_number": -1, # int
// "disc_total": -1, # int
// "isrc": "", # string
// "itunesadvisory": "", # string
// "length": -1, # int
// "publisher": "", # string
// "rating": -1, # int
// "title": "", # string
// "track_number": -1, # int
// "track_total": -1, # int
// "source": "", # string
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
            is_unique: true,
        },
        Column {
            name: "main_artist",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The main artist of the song",
            is_unique: false,
        },
        Column {
            name: "filesize_bytes",
            data_type: "BIGINT",
            primary_key: false,
            auto_increment: false,
            notes: "The size of the song in bytes",
            is_unique: false,

        },
        Column {
            name: "padding_bytes",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The padding of the song in bytes",
            is_unique: false,

        },
        Column {
            name: "album_artwork_bit_depth",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bit depth of the album artwork in bits",
            is_unique: false,

        },
        Column {
            name: "album_artwork_colors",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The number of colors in the album artwork",
            is_unique: false,

        },
        Column {
            name: "album_artwork_height",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The height of the album artwork in pixels",
            is_unique: false,

        },
        Column {
            name: "album_artwork_width",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The width of the album artwork in pixels",
            is_unique: false,

        },
        Column {
            name: "bit_depth",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bit depth of the song in bits",
            is_unique: false,

        },
        Column {
            name: "bitrate",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The bitrate of the song in bits, divide by 1000 to get Kbps",
            is_unique: false,

        },
        Column {
            name: "channels",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The number of channels in the song",
            is_unique: false,

        },
        Column {
            name: "duration",
            data_type: "FLOAT",
            primary_key: false,
            auto_increment: false,
            notes: "The duration of the song in seconds",
            is_unique: false,

        },
        Column {
            name: "sample_rate",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The sample rate of the song in KHz",
            is_unique: false,

        },
        Column {
            name: "album",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The album of the song",
            is_unique: false,

        },
        Column {
            name: "barcode",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The barcode of the song",
            is_unique: false,

        },
        Column {
            name: "date_created",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date the song was created in YYYY-MM-DD",
            is_unique: false,

        },
        Column {
            name: "disc_number",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The disc number of the song",
            is_unique: false,

        },
        Column {
            name: "disc_total",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The total number of discs in the album",
            is_unique: false,

        },
        Column {
            name: "isrc",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ISRC of the song",
            is_unique: false,

        },
        Column {
            name: "itunesadvisory",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The iTunes advisory of the song",
            is_unique: false,

        },
        Column {
            name: "length",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The length of the song",
            is_unique: false,

        },
        Column {
            name: "publisher",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The publisher of the song",
            is_unique: false,

        },
        Column {
            name: "rating",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The rating of the song",
            is_unique: false,

        },
        Column {
            name: "title",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The title of the song",
            is_unique: false,

        },
        Column {
            name: "track_number",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The track number of the song",
            is_unique: false,

        },
        Column {
            name: "track_total",
            data_type: "INTEGER",
            primary_key: false,
            auto_increment: false,
            notes: "The total number of tracks in the album",
            is_unique: false,

        },
        Column {
            name: "source",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The source of the song",
            is_unique: false,

        },
        Column {
            name: "filetype",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The file type of the song",
            is_unique: false,

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
            data_type: "TEXT",
            primary_key: true,
            auto_increment: false,
            notes: "The unique ID of the play",
            is_unique: true,

        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: false,

        },
        Column {
            name: "song_title",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The title of the song",
            is_unique: false,

        },
        Column {
            name: "main_artist",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The primary artist of the song",
            is_unique: false,

        },
        Column {
            name: "filesize",
            data_type: "BIGINT",
            primary_key: false,
            auto_increment: false,
            notes: "The size of the song in bytes",
            is_unique: false,

        },
        Column {
            name: "start_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The start date and time of the play in YYYY-MM-DD HH:MM:SS",
            is_unique: false,

        },
        Column {
            name: "end_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The end date and time of the play in YYYY-MM-DD HH:MM:SS",
            is_unique: false,

        },
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
            data_type: "TEXT",
            primary_key: true,
            auto_increment: false,
            notes: "The unique ID of the playlist",
            is_unique: true,

        },
        Column {
            name: "playlist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the playlist",
            is_unique: false,

        },
        Column {
            name: "playlist_desc",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The description of the playlist",
            is_unique: false,

        },
        Column {
            name: "created_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the playlist was created in YYYY-MM-DD HH:MM:SS",
            is_unique: false,

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
            is_unique: false,

        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: false,
        },
        Column {
            name: "added_dt",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the song was added to the playlist in YYYY-MM-DD HH:MM:SS",
            is_unique: false,
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
            is_unique: true,

        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: true,

        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the artist was added to the song in YYYY-MM-DD HH:MM:SS",
            is_unique: false,

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
            is_unique: true,
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: true,
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the artist was added to the song in YYYY-MM-DD HH:MM:SS",
            is_unique: false,
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
            is_unique: true,
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: true,
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the composer was added to the song in YYYY-MM-DD HH:MM:SS",
            is_unique: false,

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
            is_unique: true,
        },
        Column {
            name: "song_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the song",
            is_unique: true,
        },
        Column {
            name: "dt_added",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The date and time the genre was added to the song in YYYY-MM-DD HH:MM:SS",
            is_unique: false,
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
            is_unique: true,
        },
        Column {
            name: "song_path",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The path to the song",
            is_unique: true,
        },
    ],
});

// make artist table with artist_name, artist_bio, artist_photo_location

pub static ARTISTS: Lazy<Table> = Lazy::new(|| Table {
    name: "artists",
    columns: vec![
        Column {
            name: "artist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the artist",
            is_unique: true,
        },
        Column {
            name: "artist_bio",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The bio of the artist",
            is_unique: false,
        },
        Column {
            name: "artist_photo_location",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The location of the artist's photo",
            is_unique: false,
        },
    ],
});

// make album table with album id, album_name, artist_name, album_description, album_art_location, album_release_date

pub static ALBUMS: Lazy<Table> = Lazy::new(|| Table {
    name: "albums",
    columns: vec![
        Column {
            name: "album_id",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The ID of the album",
            is_unique: true,
        },
        Column {
            name: "album_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the album",
            is_unique: false,
        },
        Column {
            name: "artist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the artist",
            is_unique: false,

        },
        Column {
            name: "album_description",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The description of the album",
            is_unique: false,
        },
        Column {
            name: "album_art_location",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The location of the album's art",
            is_unique: false,
        },
        Column {
            name: "album_release_date",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The release date of the album",
            is_unique: false,
        },
    ],
});
// make fn generate_insertion_sql that takes a table and returns the SQL for inserting into that table
// for example "INSERT INTO songs (song_id, main_artist, filesize_bytes, padding_bytes, album_artwork_bit_depth, album_artwork_colors, album_artwork_height, album_artwork_width, bit_depth, bitrate, channels, duration, sample_rate, album, barcode, date_created, disc_number, disc_total, isrc, itunesadvisory, length, publisher, rating, title, track_number, track_total, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)";
// static SONGS: Lazy<Table> = Lazy::new(|| Table {
pub fn generate_insertion_sql(table: &Table) -> String {
    let mut sql = String::from("INSERT OR IGNORE INTO ");
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

pub fn generate_select_all_sql(table: &Table) -> String {
    let mut sql = String::from("SELECT * FROM ");
    sql.push_str(&table.name);
    sql
}
