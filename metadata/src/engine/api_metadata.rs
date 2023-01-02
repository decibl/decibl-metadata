// Ok here's what we want to do
// Reference popular GUIs, we want to take an image for the album and the artist.
// We want to download the album art and place it in config::get_album_photo_path(artist_name, album_name) which gives the path to put the image
// album image should be named "album.jpg" where album is the album name
// artist folder eshould have two things, "{ARTIST_NAME}_banner.jpg or png" and "{ARTIST_NAME}_profile.jpg or png"

// we also want to grab lyrics if we can, in the album folder with {SONG_NAME}_lyrics.txt

// These are some useful sites:
// musicbrainz.org
// https://genius.com/{ARTIST}
// Discogs
// also fuck their API key bs, we web scraping >:)

// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                      Getting Album and Artist Art
// --------------------------------------------------------------------------------------------------------------------------------------------

use super::config::get_artist_photo_path;

/// Attempts to get the Image of the artist from the Genius website
pub fn get_artist_profile_url_genius(artist_name: &str) -> String {
    // we are going to use the genius "API" to get the artist profile image

    // first lets format the URL
    // the general format is https://genius.com/api/artists/{ARTIST_NAME}

    // we need to replace spaces with dashes
    let artist_name_fix = artist_name.replace(" ", "-");

    let mut url = String::from("https://genius.com/artists/");
    url.push_str(&artist_name_fix);

    // now we need to get the response
    let response = reqwest::blocking::get(&url).unwrap().text().unwrap();

    // now we need to parse the response
    let document = scraper::Html::parse_document(&response);

    // this is what the HTML element we want to get the image link from looks like
    // <div class="user_avatar profile_header-avatar clipped_background_image--background_fill clipped_background_image" ng-class="{'profile_header-avatar--verified': $ctrl.user.is_verified}" clipped-background-image="$ctrl.avatar_url()" ng-show="!$ctrl.show_artist_pane" style="background-image: url(&quot;https://images.genius.com/52607f8751b7104a9a22e30a62b701d9.1000x1000x1.png&quot;);"></div>

    // we want to get the style attribute

    let style_selector = scraper::Selector::parse("div.user_avatar").unwrap();

    let mut style = String::new();

    for style_element in document.select(&style_selector) {
        style = style_element.value().attr("style").unwrap().to_string();
    }


    // now we need to get the url from the style attribute
    // the url is in the format "background-image: url('https://images.genius.com/5747a529dca274b0f2765d919c555b2d.1000x1000x1.jpg');"
    // we need to get the url between the quotes

    let url_start = style.find("url('").unwrap() + 5;
    let url_end = style.find("');").unwrap();

    let url = &style[url_start..url_end];


    url.to_string()
}

pub fn save_artist_profile_url(artist_name: &str) {
    let url = get_artist_profile_url_genius(artist_name);

    // now we need to download the image
    let mut response = reqwest::blocking::get(url).unwrap();

    // now we need to save the image
    let extension = response
        .url()
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .split(".")
        .last()
        .unwrap();

    let filename = format!("{}_profile.{}", artist_name, extension);

    let mut save_path = format!("{}/{}", get_artist_photo_path(), filename);

    // turn save_path into a Path
    let save_path = std::path::Path::new(&save_path);

    println!("{}", save_path.display());

    // now we need to save the image
    let mut file = std::fs::File::create(save_path).unwrap();

    std::io::copy(&mut response, &mut file).unwrap();

    println!("Saved image to {}", save_path.display());
}
