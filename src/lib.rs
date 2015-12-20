extern crate xmltree;
extern crate hyper;
extern crate url;
extern crate regex;

use std::io::{Read,Write};

/// Turns "123" into 123
fn intify(instr: &str) -> Result<u32, std::num::ParseIntError>{
    // TODO: Better error handling
    instr.to_owned().parse::<u32>()
}

/// Used for air-date of an episode etc
#[derive(Debug,Clone)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

fn dateify(instr: &str) -> TvdbResult<Date>{
    let chunks:Vec<&str> = instr.split("-").collect();

    let invalid_date = TvdbError::DataError{reason: format!("Malformed YYYY-MM-DD date: {}", instr)};

    let year  = chunks.get(0).ok_or(invalid_date.clone());
    let month = chunks.get(1).ok_or(invalid_date.clone());
    let day   = chunks.get(2).ok_or(invalid_date.clone());

    let year = try!(year);
    let month = try!(month);
    let day = try!(day);

    let year = try!(intify(year));
    let month = try!(intify(month));
    let day = try!(intify(day));

    Ok(Date{
        year: year,
        month: month,
        day: day,
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
#[derive(Debug,Clone)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
    DataError{reason: String},
    Cancelled,
}

pub type TvdbResult<T> = Result<T, TvdbError>;

/// Series ID from TheTVDB.com, along with language
#[derive(Debug,Clone)]
pub struct EpisodeId{
    seriesid: u32,
    language: String,
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
    pub firstaired: Option<Date>,

    /// Network this series aired on
    pub network: Option<String>,

    /// [zap2it](http://zap2it.com/) ID for this series
    pub zap2it_id: Option<String>,
}


/// Base episode record,
/// http://www.thetvdb.com/wiki/index.php?title=API:Base_Episode_Record
#[derive(Debug,Clone)]
pub struct EpisodeInfo{
    pub id: u32, //id
    // Combined_episodenumber
    // Combined_season
    // DVD_chapter
    // DVD_discid
    // DVD_episodenumber
    // DVD_season
    // Director
    // EpImgFlag
    pub episodename: String, // EpisodeName
    // EpisodeNumber
    // FirstAired
    // GuestStars
    // IMDB_ID
    // Language
    // Overview
    // ProductionCode
    // Rating
    // RatingCount
    // SeasonNumber
    // Writer
    // absolute_number
    // airsafter_season
    // airsbefore_episode
    // airsbefore_season
    // filename
    // lastupdated
    // seasonid
    // seriesid
    // thumb_added
    // thumb_height
    // thumb_width
}


fn get_xmltree_from_url(url: hyper::Url) -> TvdbResult<xmltree::Element>{
    // Check if URL is in cache
    let urlstr = url.serialize();
    let re = regex::Regex::new("[^a-zA-Z0-9_-]+").unwrap();
    let cachefile = format!("cache/cache__{}", re.replace_all(&urlstr, "_"));

    let mut body = Vec::new();

    if std::path::Path::new(&cachefile).exists() {
        println!("Reading from cached path");
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

    {
        println!("Saving XML to {}", cachefile);
        std::fs::create_dir_all("cache").expect("Failed to create cache dir");
        let mut f = std::fs::File::create(cachefile).ok().expect("Failed to create file");
        f.write_all(&mut body).ok().unwrap();
    }

    // Parse XML
    let bs = String::from_utf8(body).unwrap();
    let tree = xmltree::Element::parse(bs.as_bytes());

    return Ok(tree);
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
    pub fn search<S>(&self, seriesname: S, lang: S) -> TvdbResult<Vec<SeriesSearchResult>> where S: Into<String>{
        let params = url::form_urlencoded::serialize(
            [("seriesname", &seriesname.into()), ("language", &lang.into())].iter());
        let formatted_url = format!("http://thetvdb.com/api/GetSeries.php?{}", params);
        let url = hyper::Url::parse(&formatted_url).ok().expect("invalid URL");
        println!("Getting {}", url);

        let tree = try!(get_xmltree_from_url(url));

        // Convert XML into structs
        let mut results : Vec<SeriesSearchResult> = vec![];

        for child in tree.children.iter(){

            let r = SeriesSearchResult{
                seriesid:   intify(&get_text(child, "seriesid").expect("Search result XML missing 'seriesid' element")).ok().unwrap(),
                seriesname: get_text(child, "SeriesName").expect("Search result XML Missing 'SeriesName' element"),
                language:   get_text(child, "language").expect("Search result XML missing 'language' element"),
                overview:   get_text(child, "Overview"),
                banner:     get_text(child, "banner"),
                imdb_id:    get_text(child, "IMDB_ID"),
                firstaired: get_text(child, "FirstAired").and_then(|x| dateify(&x).ok()),
                network:    get_text(child, "Network"),
                zap2it_id:  get_text(child, "zap2it_id"),
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
        println!("Getting {}", url);

        // Perform request
        let tree = try!(get_xmltree_from_url(url));
        let root = tree.children.first().unwrap();

        // Convert XML into struct
        Ok(EpisodeInfo{
            id: intify(&get_text(root, "id").unwrap()).ok().unwrap(),
            episodename: get_text(root, "EpisodeName").unwrap(),
        })
    }

    /// Get episode information for given season/episode number
    pub fn episode<T: Into<EpisodeId>>(&self, epid: T, season: u32, episode: u32) -> TvdbResult<EpisodeInfo>{
        self.episode_inner(epid.into(), season, episode)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    const APIKEY: &'static str = "0629B785CE550C8D";

    #[test]
    fn search() {
        let api = Tvdb::new(APIKEY.to_owned());
        let sr = api.search("scrubs", "en");
        assert!(sr.ok().unwrap()[0].seriesname == "Scrubs");
    }

    #[test]
    fn nonexist() {
        let api = Tvdb::new(APIKEY);
        let sr = api.search("ladlkgdklfgsdfglk", "en");
        assert!(sr.is_err());
    }

    #[test]
    fn lookup_by_epid(){
        let api = Tvdb::new(APIKEY);
        let ep = api.episode(EpisodeId::new(76156, "en"), 1, 2).ok().unwrap();
        assert!(ep.episodename == "My Mentor");
    }

    #[test]
    fn lookup_by_u32(){
        let api = Tvdb::new(APIKEY);
        let ep = api.episode(76156, 1, 2).ok().unwrap();
        assert!(ep.episodename == "My Mentor");
    }

    #[test]
    fn epinfo_default(){
        let api = Tvdb::new(APIKEY);
        let sr = api.search("scrubs", "en").ok().unwrap();
        let ep = api.episode(sr[0].clone(), 1, 2).ok().unwrap();
        assert!(ep.episodename == "My Mentor");
    }

}
