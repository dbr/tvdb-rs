extern crate tvdb;

/// Custom error
#[derive(Debug)]
enum MyError {
    ErrorFromTvdb { reason: String },
    NotFound,
}

impl From<tvdb::TvdbError> for MyError {
    fn from(err: tvdb::TvdbError) -> MyError {
        MyError::ErrorFromTvdb { reason: format!("{}", err) }
    }
}


fn lookup_tvdb(series: &str, season: u32, episode: u32) -> Result<String, MyError> {
    // Create API with your API key
    let api = tvdb::Tvdb::new("0629B785CE550C8D");

    // Perform search (returns a vector of SeriesSearchResult's)
    let lang = "en"; // Search for English show
    let sr = try!(api.search(series));

    if sr.data.len() > 0 {
        // Look up episode based on reference to first result (the API automatically creates an
        // tvdb::EpisodeId from the `SeriesSearchResult` which the `search` method returns)
        let ep = try!(api.episode(&sr[0], season, episode));

        // Return episode name
        return Ok(ep.episode_name);
    } else {
        // No search results, return error
        return Err(MyError::NotFound);
    }
}

fn main() {
    match lookup_tvdb("Scrubs", 1, 22) {
        Ok(name) => println!("Success: {}", name),
        Err(e) => println!("Error: {:?}", e),
    }
}
