

/// The config module is
pub mod config;

/// This class will deal with all the parsing of the physical music files. Like getting the "title" metadata from hello.mp3
/// The basic rundown is the following:
/// 1. Make a trait that all the audio files will inherit from. This will have required functions that all audio files will have, so we can use them in the same way.
/// 2. Make a class for each Audiofile type and make it inherit from the trait. This will have the functions that are specific to that audio file type, but same return type.
/// For example, if I have AudioFileFLAC and AudioFileMP3 and call get_title() on both, they will return the same type of data, but the implementation will be different.
pub mod audio_metadata;
pub mod analyticsdb;
pub mod models; 
