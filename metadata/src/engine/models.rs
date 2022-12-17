// This file will hold all the different tables as models
// We'll have structs for each table and a compile method which turns the struct into an sql string

use std::collections::HashMap;
use std::collections::Vec;

struct Column {
    name: &'static str,
    data_type: &'static str,
    primary_key: bool,
    auto_increment: bool,
    notes: &'static str,
}

struct Table {
    name: &'static str,
    columns: Vec<Column>,
}

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
// "sample_rate": -1, # in KHz
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
static SONGS: Table = Table {
    name: "songs",
    columns: [
        Column {
            name: "song_id",
            data_type: "INTEGER",
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
            data_type: "INTEGER",
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
            data_type: "INTEGER",
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
        Column {
            name: "main_artist_name",
            data_type: "TEXT",
            primary_key: false,
            auto_increment: false,
            notes: "The name of the main artist of the song",
        },
    ],
};

// PLAYS TABLE
// play_id INTEGER PRIMARY KEY AUTOINCREMENT,
// song_title TEXT NOT NULL,
// song_primary_artist TEXT NOT NULL,
// filesize BIGINT,
// start_dt TEXT NOT NULL,
// end_dt TEXT NOT NULL,
// song_id TEXT NOT NULL

static PLAYS: Table = Table {
    name: "plays",
    columns: [
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
            name: "song_primary_artist",
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
};

// PLAYLISTS TABLE
// playlist_id INTEGER PRIMARY KEY AUTOINCREMENT,
// playlist_name TEXT NOT NULL,
// playlist_desc TEXT,
// created_dt TEXT NOT NULL

static PLAYLISTS: Table = Table {
    name: "playlists",
    columns: [
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
};

// PLAYLIST_SONGS TABLE
// playlist_id INTEGER NOT NULL,
// song_id TEXT NOT NULL,
// added_dt TEXT NOT NULL

static PLAYLIST_SONGS: Table = Table {
    name: "playlist_songs",
    columns: [
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
};

// SONG ARTISTS TABLE
// artist_name TEXT NOT NULL,
// song_id TEXT NOT NULL,
// dt_added TEXT NOT NULL

static SONG_ARTISTS: Table = Table {
    name: "song_artists",
    columns: [
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
};

// ALBUM ARTISTS TABLE
// artist_name TEXT NOT NULL,
// song_id TEXT NOT NULL,
// dt_added TEXT NOT NULL

static ALBUM_ARTISTS: Table = Table {
    name: "album_artists",
    columns: [
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
};

// COMPOSERS TABLE
// composer_name TEXT NOT NULL,
// song_id TEXT NOT NULL,
// dt_added TEXT NOT NULL

static COMPOSERS: Table = Table {
    name: "composers",
    columns: [
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
};

// GENRES TABLE
// genre_name TEXT NOT NULL,
// song_id TEXT NOT NULL,
// dt_added TEXT NOT NULL

static GENRES: Table = Table {
    name: "genres",
    columns: [
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
};

// SONGPATHS TABLE
// song_id TEXT NOT NULL,
// song_path TEXT NOT NULL,

static SONGPATHS: Table = Table {
    name: "songpaths",
    columns: [
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
};