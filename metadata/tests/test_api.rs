use decibl_metadata::engine::{api_metadata::{get_artist_profile_url_genius, save_artist_profile_url}, config::create_all_files};
use serial_test::serial;

#[cfg(test)]


pub fn test_sanity() {
    assert_eq!(1, 1);
}

/* ------------------------------------------------------------------------------------------------------------------------------------------------ */
/*                                                              testing image functions                                                              */
/* ------------------------------------------------------------------------------------------------------------------------------------------------ */

#[test]
pub fn test_get_artist_profile_url_genius(){
    create_all_files();
    let artist_name = "Kanye West".to_string();
    let url = get_artist_profile_url_genius(&artist_name);
    assert_eq!(url, "https://images.genius.com/5747a529dca274b0f2765d919c555b2d.1000x1000x1.jpg");
}

#[test]
pub fn test_save_artist_profile_url(){
    create_all_files();
    let artist_name = "Kanye West".to_string();

    let save_path = save_artist_profile_url(&artist_name);

    // save path is a string
    // make sure the file exists

    let path = std::path::Path::new(&save_path);
    assert!(path.exists());
    
}