// Ok here's what we want to do
// Reference popular GUIs, we want to take an image for the album and the artist.
// We want to download the album art and place it in config::get_album_photo_path(artist_name, album_name) which gives the path to put the image
// album image should be named "album.jpg" where album is the album name
// artist folder eshould have two things, "{ARTIST_NAME}_banner.jpg" and "{ARTIST_NAME}_profile.jpg"

// we also want to grab lyrics if we can, in the album folder with {SONG_NAME}_lyrics.txt

// These are some useful sites:
// musicbrainz.org
// https://genius.com/{ARTIST}
// Discogs
// also fuck their API key bs, we web scraping >:)

// --------------------------------------------------------------------------------------------------------------------------------------------
//                                                      Getting Album and Artist Art
// --------------------------------------------------------------------------------------------------------------------------------------------

/// Attempts to get the Image of the artist from the Genius website
pub fn get_artist_profile_genius(artist_name: &str) -> String {
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

    println!("{}", style);
    style


}
// let response = reqwest::blocking::get(
//     "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
// )
// .unwrap()
// .text()
// .unwrap();

// let document = scraper::Html::parse_document(&response);

// let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
// let rating_selector = scraper::Selector::parse("div.inline-block.ratings-imdb-rating").unwrap();

// let mut titles = Vec::new();

// for title in document.select(&title_selector) {
//     titles.push(title.inner_html());
// }


// println!("{:?}", titles);