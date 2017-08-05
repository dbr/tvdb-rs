use url;
use reqwest;
use xmltree;

use reqwest::header::{Headers, Authorization, Bearer};

use std::io::Read;
use std::fmt::Debug;

use data::{Date, EpisodeId, SeriesSearchResult, EpisodeInfo};
use error::{TvdbError, TvdbResult};
use parse::{intify, floatify, dateify};


/// Trait for custom implementations of URL fetching
pub trait RequestClient: Debug {
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String>;
}

/// Default implementation of RequestClient
#[derive(Debug)]
pub struct DefaultHttpClient;

impl RequestClient for DefaultHttpClient{
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String>{
        // Make request
        let client = reqwest::Client::new().unwrap();

        let mut headers = Headers::new();
        if let Some(tok) = jwt_token{
            headers.set(
               Authorization(
                   Bearer{token: tok.into()}
               )
            );
        }

        let mut resp = client.get(url)
            .headers(headers)
            .send()
            .map_err(|x| TvdbError::CommunicationError{
                reason: format!("Error creating HTTP request: {}", x)})?;

        // Check response
        if !resp.status().is_success() {
            return Err(TvdbError::CommunicationError{
                reason: format!("Unsuccessful HTTP response from url {}: {}", url, resp.status())})
        }

        let mut result = String::new();
        resp.read_to_string(&mut result)
            .map_err(|x| TvdbError::CommunicationError{
                reason: format!("Error reading response: {}", x)})?;
        return Ok(result);
    }
}


fn get_xmltree_from_str(body: String) -> TvdbResult<xmltree::Element>{
    let tree = xmltree::Element::parse(body.as_bytes());

    return tree.map_err(|e|
        TvdbError::DataError{reason: format!("Error parsing XML from TheTVDB.com: {}", e)});
}

/// Main interface
#[derive(Debug,Clone)]
pub struct Tvdb<'a>{
    /// Your API key from TheTVDB.com
    pub key: String,
    http_client: Option<&'a RequestClient>,
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


impl <'a>Tvdb <'a>{
    /// Initalise API with the given API key. A key can be aquired via
    /// the [API Key Registration page](http://thetvdb.com/?tab=apiregister)
    pub fn new<S>(key: S) -> Tvdb <'a> where S: Into<String>{
        Tvdb{
            key: key.into(),
            http_client: None,
        }
    }

    /// Sets a custom client (implementation of `RequestClient`) used to
    /// perform HTTP requests
    pub fn set_http_client(&mut self, client: &'a RequestClient) {
        self.http_client = Some::<&'a RequestClient>(client);
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

        let url = format!("http://thetvdb.com/api/GetSeries.php?{}", params);
        debug!("Getting {}", url);

        let default_client = DefaultHttpClient{}; // FIXME: Create one per instance
        let c = self.http_client.unwrap_or(&default_client);
        let data = try!(c.get_url(&url, None));

        let tree = try!(get_xmltree_from_str(data));

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

        let url = format!("http://thetvdb.com/api/{apikey}/series/{seriesid}/default/{season}/{episode}/{language}.xml",
                                    apikey=self.key,
                                    seriesid=epid.seriesid,
                                    language=epid.language,
                                    season=season,
                                    episode=episode,
                                    );
        debug!("Getting {}", url);

        // Perform request
        let default_client = DefaultHttpClient{}; // FIXME: Create one per instance
        let c = self.http_client.unwrap_or(&default_client);
        let data = try!(c.get_url(&url, None));

        let tree = try!(get_xmltree_from_str(data));
        let root = try!(tree.children.first()
            .ok_or(TvdbError::DataError{reason:
                format!("XML from {} had no child elements", url)}));

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
