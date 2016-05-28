#[macro_use]
extern crate log;

extern crate xmltree;
extern crate hyper;
extern crate url;
extern crate regex;

use std::io::{Read,Write};

/// Turns "123" into 123
fn intify(instr: &str) -> Result<u32, std::num::ParseIntError>{
    instr.to_owned().parse::<u32>()
}

/// Turns "123.1" into 123.1
fn floatify(instr: &str) -> Result<f32, std::num::ParseFloatError>{
    instr.to_owned().parse::<f32>()
}

/// Used for air-date of an episode etc
#[derive(Debug,Clone)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

/// Parse YYYY-MM-DD formatted string into `Date` struct
fn dateify(instr: &str) -> TvdbResult<Date>{
    let invalid_date = || {TvdbError::DataError{reason: format!("Malformed YYYY-MM-DD date: {}", instr)}};

    let chunks:Vec<&str> = instr.split("-").collect();
    if chunks.len() != 3 {
        return Err(invalid_date());
    }

    let year  = try!(chunks.get(0).ok_or(invalid_date()));
    let month = try!(chunks.get(1).ok_or(invalid_date()));
    let day   = try!(chunks.get(2).ok_or(invalid_date()));

    Ok(Date{
        year: try!(intify(year)),
        month: try!(intify(month)),
        day: try!(intify(day)),
    })
}

#[test]
fn test_date_parser_good() {
    let d = dateify("2001-02-03");
    println!("Parsed date as {:?}", d);

    assert!(d.is_ok());
    let d = d.unwrap();
    assert!(d.year == 2001);
    assert!(d.month == 2);
    assert!(d.day == 3);
}


#[test]
fn test_date_parser_bad() {
    assert!(dateify("blah").is_err());
    assert!(dateify("2001-02").is_err());
    assert!(dateify("2001-02-blah").is_err());
}


/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    InternalError{reason: String},
    SeriesNotFound,
    CommunicationError{reason: String},
    DataError{reason: String},
    Cancelled,
}

pub type TvdbResult<T> = Result<T, TvdbError>;


/// Convert from parse error (e.g for dateify() function)
impl From<std::num::ParseIntError> for TvdbError{
    fn from(err: std::num::ParseIntError) -> TvdbError{
        TvdbError::DataError{reason: format!("{}", err)} // FIXME
    }
}


/// Series ID from TheTVDB.com, along with language
#[derive(Debug,Clone)]
pub struct EpisodeId{
    pub seriesid: u32,
    pub language: String,
}

impl EpisodeId{
    pub fn new(seriesid: u32, lang: &str) -> EpisodeId{
        EpisodeId{
            seriesid: seriesid,
            language: lang.to_owned(),
        }
    }
}

impl From<u32> for EpisodeId{
    fn from(x: u32) -> Self{
        EpisodeId{seriesid: x, language: "en".to_owned()}
    }
}

impl From<SeriesSearchResult> for EpisodeId{
    fn from(x: SeriesSearchResult) -> Self{
        EpisodeId{seriesid: x.seriesid, language: x.language}
    }
}

impl<'a> From<&'a SeriesSearchResult> for EpisodeId{
    fn from(x: &SeriesSearchResult) -> Self{
        EpisodeId{seriesid: x.seriesid.clone(), language: x.language.clone()}
    }
}

/// Series info as returned from TheTVDB's series search method:
/// http://www.thetvdb.com/wiki/index.php?title=API:GetSeries
#[derive(Debug,Clone)]
pub struct SeriesSearchResult{
    /// TheTVDB's series ID ('seriesid' is preferred over 'id' from XML response)
    pub seriesid: u32,

    /// Series name in the language indicated by `language`
    pub seriesname: String,

    /// Language this episode information is in
    pub language: String,

    /// Description of series
    pub overview: Option<String>,

    /// Relative path to the highest rated banner
    pub banner: Option<String>,

    /// [IMDB](http://www.imdb.com/) ID for this series
    pub imdb_id: Option<String>,

    /// First aired date
    pub first_aired: Option<Date>,

    /// Network this series aired on
    pub network: Option<String>,

    /// [zap2it](http://zap2it.com/) ID for this series
    pub zap2it_id: Option<String>,
}


/// Base episode record,
/// http://www.thetvdb.com/wiki/index.php?title=API:Base_Episode_Record
#[derive(Debug,Clone)]
pub struct EpisodeInfo{
    /// An unsigned integer assigned by TheTVDB to the episode. Cannot be null.
    pub id: u32, //id

    /// A string containing the episode name in the language requested. Will return the English name if no translation is available in the language requested.
    pub episode_name: String, // EpisodeName


    /// An unsigned integer representing the season number for the episode according to the aired order. Cannot be null.
    pub season_number: u32, // SeasonNumber

    /// An unsigned integer indicating the season the episode was in according to the DVD release. Usually is the same as EpisodeNumber but can be different.
    pub season_dvd: Option<u32>, // DVD_season

    /// An unsigned integer or decimal. Cannot be null. This returns the value of DVD_season if that field is not null. Otherwise it returns the value from SeasonNumber. The field can be used as a simple way of prioritizing DVD order over aired order in your program. In general it's best to avoid using this field as you can accomplish the same task locally and have more control if you use the DVD_season and SeasonNumber fields separately.
    /// (note: missing from episodes so made optional)
    pub season_combined: Option<f32>, // Combined_season


    /// An unsigned integer representing the episode number in its season according to the aired order. Cannot be null.
    pub episode_number: u32, // EpisodeNumber

    /// An unsigned integer or decimal. Cannot be null. This returns the value of DVD_episodenumber if that field is not null. Otherwise it returns the value from EpisodeNumber. The field can be used as a simple way of prioritizing DVD order over aired order in your program. In general it's best to avoid using this field as you can accomplish the same task locally and have more control if you use the DVD_episodenumber and EpisodeNumber fields separately.
    /// (note: missing from episodes so made optional)
    pub episode_combined: Option<f32>, // Combined_episodenumber

    // DVD_chapter - deprecated
    // DVD_discid - deprecated

    /// A decimal with one decimal and can be used to join episodes together. Can be null, usually used to join episodes that aired as two episodes but were released on DVD as a single episode. If you see an episode 1.1 and 1.2 that means both records should be combined to make episode 1. Cartoons are also known to combine up to 9 episodes together, for example Animaniacs season two.
    pub episode_dvd: Option<f32>, // DVD_episodenumber

    /// A string containing the date the series first aired in plain text using the format "YYYY-MM-DD". Can be null.
    pub first_aired: Option<Date>, // FirstAired

    /// An alphanumeric string containing the IMDB ID for the series. Can be null.
    pub imdb_id: Option<String>, // IMDB_ID

    /// A two character string indicating the language in accordance with ISO-639-1. Cannot be null.
    pub language: String, // Language

    /// A string containing the overview in the language requested. Will return the English overview if no translation is available in the language requested. Can be null.
    pub overview: Option<String>, // Overview

    /// An alphanumeric string. Can be null.
    pub production_code: Option<String>, // ProductionCode

    /// The average rating our users have rated the series out of 10, rounded to 1 decimal place. Can be null.
    pub rating: Option<f32>, // Rating

    /// An unsigned integer representing the number of users who have rated the series. Can be null.
    pub rating_count: Option<u32>, // RatingCount

    /// A pipe delimited string of guest stars in plain text. Can be null.
    pub guest_stars: Option<String>, // GuestStars

    /// A pipe delimited string of directors in plain text. Can be null.
    pub director: Option<String>, // Director

    /// A pipe delimited string of writers in plain text. Can be null.
    pub writer: Option<String>, // Writer

    /// An unsigned integer. Can be null. Indicates the absolute episode number and completely ignores seasons. In others words a series with 20 episodes per season will have Season 3 episode 10 listed as 50. The field is mostly used with cartoons and anime series as they may have ambiguous seasons making it easier to use this field.
    pub episode_absolute: Option<u32>, // absolute_number

    /// An unsigned integer indicating the season number this episode comes after. This field is only available for special episodes. Can be null.
    pub airs_after_season: Option<u32>, // airsafter_season

    /// An unsigned integer indicating the episode number this special episode airs before. Must be used in conjunction with airsbefore_season, do not with airsafter_season. This field is only available for special episodes. Can be null.
    pub airs_before_episode: Option<u32>, // airsbefore_episode

    /// An unsigned integer indicating the season number this special episode airs before. Should be used in conjunction with airsbefore_episode for exact placement. This field is only available for special episodes. Can be null.
    pub airs_before_season: Option<u32>, // airsbefore_season

    /// An unsigned integer assigned by our site to the season. Cannot be null.
    pub season_id: u32, // seasonid

    /// An unsigned integer assigned by our site to the series. It does not change and will always represent the same series. Cannot be null.
    pub series_id: u32, // seriesid

    /// A string which should be appended to <mirrorpath>/banners/ to determine the actual location of the artwork. Returns the location of the episode image. Can be null.
    pub thumbnail: Option<String>, // filename

    /// An unsigned integer from 1-6.
    ///
    /// 1. Indicates an image is a proper 4:3 (1.31 to 1.35) aspect ratio.
    /// 2. Indicates an image is a proper 16:9 (1.739 to 1.818) aspect ratio.
    /// 3. Invalid Aspect Ratio - Indicates anything not in a 4:3 or 16:9 ratio. We don't bother listing any other non standard ratios.
    /// 4. Image too Small - Just means the image is smaller then 300x170.
    /// 5. Black Bars - Indicates there are black bars along one or all four sides of the image.
    /// 6. Improper Action Shot - Could mean a number of things, usually used when someone uploads a promotional picture that isn't actually from that episode but does refrence the episode, it could also mean it's a credit shot or that there is writting all over it. It's rarely used since most times an image would just be outright deleted if it falls in this category.
    ///
    /// It can also be null. If it's 1 or 2 the site assumes it's a proper image, anything above 2 is considered incorrect and can be replaced by anyone with an account.
    pub thumbnail_flag: Option<u32>, // EpImgFlag

    /// A string containing the time the episode image was added to our site in the format "YYYY-MM-DD HH:MM:SS" based on a 24 hour clock. Can be null.
    pub thumbnail_added: Option<Date>, // thumb_added

    /// An unsigned integer that represents the height of the episode image in pixels. Can be null
    pub thumbnail_height: Option<u32>, // thumb_height

    /// An unsigned integer that represents the width of the episode image in pixels. Can be null
    pub thumbnail_width: Option<u32>, // thumb_width

    /// Unix time stamp indicating the last time any changes were made to the episode. Can be null.
    pub last_updated: Option<u32>, // lastupdated
}


fn get_xmltree_from_url(url: hyper::Url) -> TvdbResult<xmltree::Element>{
    let enable_cache = false;

    // Check if URL is in cache
    let urlstr = url.clone().into_string();
    let re = regex::Regex::new("[^a-zA-Z0-9_-]+").unwrap();
    let cachefile = format!("cache/cache__{}", re.replace_all(&urlstr, "_"));

    let mut body = Vec::new();

    if enable_cache && std::path::Path::new(&cachefile).exists() {
        debug!("Reading from cached path");
        let f = std::fs::File::open(&cachefile).ok().expect("failed to open cache file");
        let mut reader = std::io::BufReader::new(f);
        reader.read_to_end(&mut body).unwrap();
    } else {
        let client = hyper::Client::new();
        let res = client.get(url)
            .header(hyper::header::Connection::close())
            .send();

        let mut res = match res {
            Err(e) => return Err(TvdbError::CommunicationError{reason: format!("Error contacting TVDB: {}", e)}), // FIXME: http://stackoverflow.com/questions/28911833/error-handling-best-practices
            Ok(r) => r
        };

        // Read the Response.
        res.read_to_end(&mut body).expect("Failed to read response");
    }

    if enable_cache {
        debug!("Saving XML to {}", cachefile);
        std::fs::create_dir_all("cache").expect("Failed to create cache dir");
        let mut f = std::fs::File::create(cachefile).ok().expect("Failed to create file");
        f.write_all(&mut body).ok().unwrap();
    }

    // Parse XML
    let bs = String::from_utf8(body).unwrap();
    let tree = xmltree::Element::parse(bs.as_bytes());

    return tree.map_err(
        |e| TvdbError::DataError{reason: format!("XML error: {}", e)});
}

/// Main interface
#[derive(Debug,Clone)]
pub struct Tvdb{
    pub key: String,
}

fn get_text(child: &xmltree::Element, x: &str) -> Option<String>{
    child.get_child(x).and_then(|id_child| id_child.text.clone())
}

impl Tvdb{
    /// Initalise API with the given API key. A key can be aquired via
    /// the [API Key Registration page](http://thetvdb.com/?tab=apiregister)
    pub fn new<S>(key: S) -> Tvdb where S: Into<String>{
        Tvdb{key: key.into()}
    }

    /// Searches for a given series name.
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
                reason: format!("Invalid URL {}: {}", formatted_url, x)}));
        debug!("Getting {}", url);

        let tree = try!(get_xmltree_from_url(url));

        // Convert XML into structs
        let mut results : Vec<SeriesSearchResult> = vec![];

        for child in tree.children.iter(){

            let r = SeriesSearchResult{
                seriesid:    intify(&get_text(child, "seriesid").expect("Search result XML missing 'seriesid' element")).ok().unwrap(),
                seriesname:  get_text(child, "SeriesName").expect("Search result XML Missing 'SeriesName' element"),
                language:    get_text(child, "language").expect("Search result XML missing 'language' element"),
                overview:    get_text(child, "Overview"),
                banner:      get_text(child, "banner"),
                imdb_id:     get_text(child, "IMDB_ID"),
                first_aired: get_text(child, "FirstAired").and_then(|x| dateify(&x).ok()),
                network:     get_text(child, "Network"),
                zap2it_id:   get_text(child, "zap2it_id"),
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
        let url = hyper::Url::parse(&formatted_url).ok().expect("invalid URL");
        debug!("Getting {}", url);

        // Perform request
        let tree = try!(get_xmltree_from_url(url));
        let root = tree.children.first().unwrap();

        // Convert XML into struct
        Ok(EpisodeInfo{
            id:                  try!(get_text(root, "id").and_then(|x| intify(&x).ok()).ok_or_else(|| TvdbError::DataError{reason:"id missing".to_owned()})),
            episode_name:        try!(get_text(root, "EpisodeName").ok_or_else(|| TvdbError::DataError{reason:"EpisodeName missing".to_owned()})),
            first_aired:         get_text(root, "FirstAired").and_then(|x| dateify(&x).ok()),
            season_number:       try!(get_text(root, "SeasonNumber").and_then(|x| intify(&x).ok()).ok_or_else(|| TvdbError::DataError{reason:"SeasonNumber missing".to_owned()})),
            season_dvd:          get_text(root, "DVD_season").and_then(|x| intify(&x).ok()),
            season_combined:     get_text(root, "Combined_season").and_then(|x| floatify(&x).ok()),
            episode_number:      try!(get_text(root, "EpisodeNumber").and_then(|x| intify(&x).ok()).ok_or_else(|| TvdbError::DataError{reason:"EpisodeNumber missing".to_owned()})),
            episode_combined:    get_text(root, "Combined_episodenumber").and_then(|x| floatify(&x).ok()),
            episode_dvd:         get_text(root, "DVD_episodenumber").and_then(|x| floatify(&x).ok()),
            imdb_id:             get_text(root, "IMDB_ID"),
            language:            try!(get_text(root, "Language").ok_or_else(|| TvdbError::DataError{reason:"language missing".to_owned()})),
            overview:            get_text(root, "Overview"),
            production_code:     get_text(root, "ProductionCode"),
            rating:              get_text(root, "Rating").and_then(|x| floatify(&x).ok()),
            rating_count:        get_text(root, "RatingCount").and_then(|x| intify(&x).ok()),
            guest_stars:         get_text(root, "GuestStars"),
            director:            get_text(root, "Director"),
            writer:              get_text(root, "Writer"),
            episode_absolute:    get_text(root, "absolute_number").and_then(|x| intify(&x).ok()),
            airs_after_season:   get_text(root, "airsafter_season").and_then(|x| intify(&x).ok()),
            airs_before_episode: get_text(root, "airsbefore_episode").and_then(|x| intify(&x).ok()),
            airs_before_season:  get_text(root, "airsbefore_season").and_then(|x| intify(&x).ok()),
            season_id:           try!(get_text(root, "seasonid").and_then(|x| intify(&x).ok()).ok_or_else(|| TvdbError::DataError{reason:"seasonid missing".to_owned()})),
            series_id:           try!(get_text(root, "seriesid").and_then(|x| intify(&x).ok()).ok_or_else(|| TvdbError::DataError{reason:"seriesid missing".to_owned()})),
            thumbnail:           get_text(root, "filename"),
            thumbnail_flag:      get_text(root, "EpImgFlag").and_then(|x| intify(&x).ok()),
            thumbnail_added:     get_text(root, "thumb_added").and_then(|x| dateify(&x).ok()),
            thumbnail_width:     get_text(root, "thumb_width").and_then(|x| intify(&x).ok()),
            thumbnail_height:    get_text(root, "thumb_width").and_then(|x| intify(&x).ok()),
            last_updated:        get_text(root, "lastupdated").and_then(|x| intify(&x).ok()),

        })
    }

    /// Get episode information for given season/episode number
    pub fn episode<T: Into<EpisodeId>>(&self, epid: T, season: u32, episode: u32) -> TvdbResult<EpisodeInfo>{
        self.episode_inner(epid.into(), season, episode)
    }
}
