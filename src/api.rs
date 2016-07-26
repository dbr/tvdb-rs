use url;
use hyper;
use xmltree;

use std::io::Read;

use data::{Date, EpisodeId, SeriesSearchResult, EpisodeInfo};
use error::{TvdbError, TvdbResult};
use parse::{intify, floatify, dateify};


fn get_xmltree_from_url(url: hyper::Url) -> TvdbResult<xmltree::Element>{
    let urlstr = url.clone().into_string();
    debug!("Fetching URL {}", urlstr);

    // Make request
    let client = hyper::Client::new();
    let res = client.get(url)
        .header(hyper::header::Connection::close())
        .send();

    let mut res = match res {
        Err(e) => return Err(TvdbError::CommunicationError{reason: format!("Error accessing {} - {}", urlstr, e)}),
        Ok(r) => r
    };

    // Ensure status code is good
    if !res.status.is_success() {
        return Err(
            TvdbError::CommunicationError{
                reason: format!("HTTP error accessing {} - {}", urlstr, res.status)});
    }

    // Read the Response body
    let mut body = Vec::new();
    try!(res.read_to_end(&mut body)
        .map_err(|e| TvdbError::CommunicationError{
            reason: format!("Failed to read response: {}", e)}));

    // Parse XML
    let bs = try!(String::from_utf8(body)
        .map_err(|e| TvdbError::DataError{reason:
            format!("Error UTF-8 decoding response from url {} - {}", urlstr, e)}));
    let tree = xmltree::Element::parse(bs.as_bytes());

    return tree.map_err(|e|
        TvdbError::DataError{reason: format!("Error parsing XML from TheTVDB.com: {}", e)});
}

/// Main interface
#[derive(Debug,Clone)]
pub struct Tvdb{
    /// Your API key from TheTVDB.com
    pub key: String,
}

/// Get text from element, or return None
fn get_text_optional(child: &xmltree::Element, x: &str) -> Option<String>{
    child.get_child(x).and_then(|id_child| id_child.text.clone())
}

/// Get text from element, or error
fn get_text_req(root: &xmltree::Element, name: &str) -> TvdbResult<String>{
    get_text_optional(root, name)
        .ok_or_else(|| TvdbError::DataError{reason:format!("Element {} missing", name)})
}

/// Get an integer from element, or return None
fn get_int_optional(root: &xmltree::Element, name: &str) -> TvdbResult<Option<u32>>{
    match get_text_optional(root, name) {
        Some(x) => Ok(intify(&x).ok()),
        None => Ok(None),
    }
}

/// Get an integer from element, or return error
fn get_int_req(root: &xmltree::Element, name: &str) -> TvdbResult<u32>{
    match get_text_optional(root, name) {
        Some(x) =>
            intify(&x).map_err(|e|
                TvdbError::DataError{reason: format!("Error parsing {}: {}", name, e)}),
        None =>
            Err(TvdbError::DataError{reason:format!("Element {} missing", name)})
    }
}

/// Get an Date from element, or return None
fn get_date_optional(root: &xmltree::Element, name: &str) -> TvdbResult<Option<Date>>{
    match get_text_optional(root, name) {
        Some(x) => Ok(dateify(&x).ok()),
        None => Ok(None),
    }
}

/// Get an float from element, or return None
fn get_float_optional(root: &xmltree::Element, name: &str) -> TvdbResult<Option<f32>>{
    match get_text_optional(root, name) {
        Some(x) => Ok(floatify(&x).ok()),
        None => Ok(None),
    }
}


impl Tvdb{
    /// Initalise API with the given API key. A key can be aquired via
    /// the [API Key Registration page](http://thetvdb.com/?tab=apiregister)
    pub fn new<S>(key: S) -> Tvdb where S: Into<String>{
        Tvdb{key: key.into()}
    }

    /// Searches for a given series name.
    ///
    /// # Errors
    ///
    /// Returns TvdbResult under various circumstances - typically
    /// communication errors with TheTVDB servers, malformed XML
    /// responses and so on.
    ///
    /// # Examples
    /// ```
    /// # let MY_API_KEY = "0629B785CE550C8D";
    /// let api = tvdb::Tvdb::new(MY_API_KEY);
    /// let results = api.search("Scrubs", "en");
    /// match results{
    ///    Ok(r) => println!("{}", r[0].seriesname), // Print series name of first result
    ///    Err(_) => panic!(),
    /// }
    /// ```
    pub fn search(&self, seriesname: &str, lang: &str) -> TvdbResult<Vec<SeriesSearchResult>> {
        let params = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("seriesname", seriesname)
            .append_pair("language", lang)
            .finish();

        let formatted_url = format!("http://thetvdb.com/api/GetSeries.php?{}", params);
        let url = try!(
            hyper::Url::parse(&formatted_url)
            .map_err(|x| TvdbError::InternalError{
                reason: format!("Invalid URL {} - {}", formatted_url, x)}));
        debug!("Getting {}", url);

        let tree = try!(get_xmltree_from_url(url));

        // Convert XML into structs
        let mut results : Vec<SeriesSearchResult> = vec![];

        for child in tree.children.iter(){

            let r = SeriesSearchResult{
                seriesid:    try!(get_int_req(child, "seriesid")),
                seriesname:  try!(get_text_req(child, "SeriesName")),
                language:    try!(get_text_req(child, "language")),
                overview:    get_text_optional(child, "Overview"),
                banner:      get_text_optional(child, "banner"),
                imdb_id:     get_text_optional(child, "IMDB_ID"),
                first_aired: try!(get_date_optional(child, "FirstAired")),
                network:     get_text_optional(child, "Network"),
                zap2it_id:   get_text_optional(child, "zap2it_id"),
            };

            results.push(r);
        }

        if results.is_empty(){
            return Err(TvdbError::SeriesNotFound);
        } else {
            return Ok(results);
        }
    }

    fn episode_inner(&self, epid: EpisodeId, season: u32, episode: u32) -> TvdbResult<EpisodeInfo>{
        // <mirrorpath>/api/<apikey>/series/{seriesid}/default/{season}/{episode}/{language}.xml

        let formatted_url = format!("http://thetvdb.com/api/{apikey}/series/{seriesid}/default/{season}/{episode}/{language}.xml",
                                    apikey=self.key,
                                    seriesid=epid.seriesid,
                                    language=epid.language,
                                    season=season,
                                    episode=episode,
                                    );
        let url = try!(hyper::Url::parse(&formatted_url)
            .map_err(|e| TvdbError::InternalError{reason:
                format!("Constructed invalid episode info URL {} - {}", formatted_url, e)}));
        debug!("Getting {}", formatted_url);

        // Perform request
        let tree = try!(get_xmltree_from_url(url));
        let root = try!(tree.children.first()
            .ok_or(TvdbError::DataError{reason:
                format!("XML from {} had no child elements", formatted_url)}));

        // Convert XML into struct
        Ok(EpisodeInfo{
            id:                  try!(get_int_req(root, "id")),
            episode_name:        try!(get_text_req(root, "EpisodeName")),
            first_aired:         try!(get_date_optional(root, "FirstAired")),
            season_number:       try!(get_int_req(root, "SeasonNumber")),
            season_dvd:          try!(get_int_optional(root, "DVD_season")),
            season_combined:     try!(get_float_optional(root, "Combined_season")),
            episode_number:      try!(get_int_req(root, "EpisodeNumber")),
            episode_combined:    try!(get_float_optional(root, "Combined_episodenumber")),
            episode_dvd:         try!(get_float_optional(root, "DVD_episodenumber")),
            imdb_id:             get_text_optional(root, "IMDB_ID"),
            language:            try!(get_text_req(root, "Language")),
            overview:            get_text_optional(root, "Overview"),
            production_code:     get_text_optional(root, "ProductionCode"),
            rating:              try!(get_float_optional(root, "Rating")),
            rating_count:        try!(get_int_optional(root, "RatingCount")),
            guest_stars:         get_text_optional(root, "GuestStars"),
            director:            get_text_optional(root, "Director"),
            writer:              get_text_optional(root, "Writer"),
            episode_absolute:    try!(get_int_optional(root, "absolute_number")),
            airs_after_season:   try!(get_int_optional(root, "airsafter_season")),
            airs_before_episode: try!(get_int_optional(root, "airsbefore_episode")),
            airs_before_season:  try!(get_int_optional(root, "airsbefore_season")),
            season_id:           try!(get_int_req(root, "seasonid")),
            series_id:           try!(get_int_req(root, "seriesid")),
            thumbnail:           get_text_optional(root, "filename"),
            thumbnail_flag:      try!(get_int_optional(root, "EpImgFlag")),
            thumbnail_added:     try!(get_date_optional(root, "thumb_added")),
            thumbnail_width:     try!(get_int_optional(root, "thumb_width")),
            thumbnail_height:    try!(get_int_optional(root, "thumb_width")),
            last_updated:        try!(get_int_optional(root, "lastupdated")),

        })
    }

    /// Get episode information for given season/episode number
    ///
    /// # Examples
    /// ```
    /// # let MY_API_KEY = "0629B785CE550C8D";
    /// let api = tvdb::Tvdb::new(MY_API_KEY);
    ///
    /// // Lookup the 23rd episode for the given series ID:
    /// let ep_by_id = api.episode(76156, 1, 23).unwrap();
    /// println!("{}", ep_by_id.episode_name);
    ///
    /// // More commonly, perform search for series
    /// let sr = api.search("scrubs", "en").unwrap();
    /// let ref first_result = sr[0];
    ///
    /// // ..then lookup the 23rd episode of the first result:
    /// let ep = api.episode(first_result, 1, 23).unwrap();
    /// println!("{}", ep.episode_name);
    /// ```
    pub fn episode<T: Into<EpisodeId>>(&self, epid: T, season: u32, episode: u32) -> TvdbResult<EpisodeInfo>{
        self.episode_inner(epid.into(), season, episode)
    }
}
