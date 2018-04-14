use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::rc::Rc;

use reqwest;
use reqwest::header::{Authorization, Bearer, Headers};
use serde_json;
use url;

use super::error::{TvdbError, TvdbResult};
use data::{EpisodeId, SeriesId};

/// Trait for custom implementations of URL fetching
pub trait RequestClient: Debug {
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String>;
}

/// Default implementation of RequestClient
#[derive(Debug)]
pub struct DefaultHttpClient;

impl RequestClient for DefaultHttpClient {
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String> {
        // Make request
        let client = reqwest::Client::new();

        let mut headers = Headers::new();
        if let Some(tok) = jwt_token {
            headers.set(Authorization(Bearer { token: tok.into() }));
        }

        let mut resp = client.get(url).headers(headers).send().map_err(|x| {
            TvdbError::CommunicationError {
                reason: format!("Error creating HTTP request: {}", x),
            }
        })?;

        // Check response
        if !resp.status().is_success() {
            return Err(TvdbError::CommunicationError {
                reason: format!(
                    "Unsuccessful HTTP response from url {}: {}",
                    url,
                    resp.status()
                ),
            });
        }

        let mut result = String::new();
        resp.read_to_string(&mut result)
            .map_err(|x| TvdbError::CommunicationError {
                reason: format!("Error reading response: {}", x),
            })?;
        return Ok(result);
    }
}

/// Main interface
#[derive(Debug, Clone)]
pub struct Tvdb<'a> {
    /// Your API key from TheTVDB.com
    pub key: String,
    http_client: Option<&'a RequestClient>,
    jwt_token: RefCell<Option<String>>,
    default_client: Rc<RequestClient>,
}

/// https://api.thetvdb.com/swagger#/Authentication
#[derive(Deserialize, Debug)]
struct LoginResponse {
    token: String,
}

/// List of `SeriesSearchData`, returned from a search
#[derive(Deserialize, Debug)]
pub struct SeriesSearchResult {
    pub data: Option<Vec<SeriesSearchData>>,
    pub error: Option<String>,
}

/// Info for a single series, as returned from search query
#[derive(Deserialize, Debug, Clone)]
pub struct SeriesSearchData {
    pub aliases: Option<Vec<String>>,
    pub banner: Option<String>,
    #[serde(rename = "firstAired")]
    pub first_aired: Option<String>,
    pub id: Option<u32>,
    pub network: Option<String>,
    pub overview: Option<String>,
    #[serde(rename = "seriesName")]
    pub series_name: String,
    pub status: Option<String>,
}

impl From<SeriesSearchData> for EpisodeId {
    fn from(x: SeriesSearchData) -> Self {
        EpisodeId {
            seriesid: x.id.unwrap() as u32,
            language: "en".into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JSONErrors {
    pub invalid_filters: Option<Vec<String>>,
    pub invalid_language: Option<Vec<String>>,
    pub invalid_query_params: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeRecordData {
    data: Option<Episode>,
    errors: Option<JSONErrors>,
}

/// Info for an episode
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub absolute_number: Option<u32>,
    pub aired_episode_number: Option<u32>,
    pub aired_season: Option<u32>,
    pub airs_after_season: Option<u32>,
    pub airs_before_episode: Option<u32>,
    pub airs_before_season: Option<u32>,
    pub director: Option<String>,
    pub directors: Option<Vec<String>>,
    pub dvd_chapter: Option<u32>,
    pub dvd_discid: Option<String>,
    pub dvd_episode_number: Option<u32>,
    pub dvd_season: Option<u32>,
    pub episode_name: String, // FIXME: Should be optional
    pub filename: Option<String>,
    pub first_aired: Option<String>,
    pub guest_stars: Option<Vec<String>>,
    pub id: Option<u32>,
    pub imdb_id: Option<String>,
    pub last_updated: Option<u32>,
    pub last_updated_by: Option<u32>, // FIXME: Should be String
    pub overview: Option<String>,
    pub production_code: Option<String>,
    pub series_id: Option<u32>,
    pub show_url: Option<String>,
    pub site_rating: Option<f32>,
    pub site_rating_count: Option<u32>,
    pub thumb_added: Option<String>,
    pub thumb_author: Option<u32>, // FIXME: Should be String
    pub thumb_height: Option<String>,
    pub thumb_width: Option<String>,
    pub writers: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SeriesEpisodes {
    pub data: Option<Vec<BasicEpisode>>,
    pub errors: Option<JSONErrors>,
    pub links: Option<Links>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasicEpisode {
    pub absolute_number: Option<u32>,
    pub aired_episode_number: Option<u32>,
    pub aired_season: Option<u32>,
    pub dvd_episode_number: Option<u32>,
    pub dvd_season: Option<u32>,
    pub episode_name: Option<String>,
    pub first_aired: Option<String>,
    pub id: Option<u32>,
    pub last_updated: Option<u32>,
    pub overview: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub first: Option<u32>,
    pub last: Option<u32>,
    pub next: Option<u32>,
    pub previous: Option<u32>,
}

impl<'a> Tvdb<'a> {
    /// Initalise API with the given API key. A key can be acquired via
    /// the [API Key Registration page](http://thetvdb.com/?tab=apiregister)
    pub fn new<S>(key: S) -> Tvdb<'a>
    where
        S: Into<String>,
    {
        Tvdb {
            key: key.into(),
            http_client: None,
            jwt_token: RefCell::new(None),
            default_client: Rc::new(DefaultHttpClient {}),
        }
    }

    /// Set the JWT session token
    fn set_token(&self, token: String) {
        let mut j = self.jwt_token.borrow_mut();
        *j = Some(token);
    }

    /// Get JWT session token (typically set via `login` method)
    fn get_token(&self) -> Option<String> {
        let j = self.jwt_token.borrow();
        match *j {
            None => None,
            Some(ref t) => Some(format!("{}", *t)),
        }
    }

    /// Authenticate with TheTVDB, storing the JWT token internally for use by
    /// other methods.
    pub fn login(&self) -> TvdbResult<bool> {
        let mut map = HashMap::new();
        map.insert("apikey", &self.key);

        let c = reqwest::Client::new();
        let mut resp = c.post("https://api.thetvdb.com/login")
            .json(&map)
            .send()
            .map_err(|x| TvdbError::CommunicationError {
                reason: format!("{}", x),
            })?;
        let mut result = String::new();
        resp.read_to_string(&mut result)
            .map_err(|x| TvdbError::CommunicationError {
                reason: format!("Error reading response: {}", x),
            })?;

        let deserialized: serde_json::Value = serde_json::from_str(&result).unwrap();
        let tok: String = deserialized["token"].as_str().unwrap().into();
        self.set_token(tok);

        Ok(true)
    }

    /// Sets a custom client (implementation of `RequestClient`) used to
    /// perform HTTP requests
    pub fn set_http_client(&mut self, client: &'a RequestClient) {
        self.http_client = Some::<&'a RequestClient>(client);
    }

    /// Search for series by name or IMDB ID
    /// <https://api.thetvdb.com/swagger#!/Search/get_search_series>
    pub fn search(
        &self,
        name: Option<&str>,
        imdb_id: Option<&str>,
    ) -> TvdbResult<SeriesSearchResult> {
        let dc = self.default_client.as_ref();
        let c = self.http_client.unwrap_or(dc);

        let mut params: HashMap<&str, &str> = HashMap::new();
        if let Some(n) = name {
            params.insert("name", n);
        }
        if let Some(i) = imdb_id {
            params.insert("imdbId", i);
        }

        let search_url = "https://api.thetvdb.com/search/series";
        let url: String = url::Url::parse_with_params(search_url, params)
            .unwrap()
            .as_str()
            .into();
        // Query URL
        let data = c.get_url(&url, self.get_token())?;

        // Parse result
        let result: SeriesSearchResult = serde_json::from_str(&data).unwrap();

        Ok(result)
    }

    fn episode_inner(&self, id: EpisodeId) -> TvdbResult<Episode> {
        let dc = self.default_client.as_ref();
        let c = self.http_client.unwrap_or(dc);

        // TODO Use `id.language`

        let url = format!("https://api.thetvdb.com/episodes/{id}", id = id.seriesid);
        let data = c.get_url(&url, self.get_token())?;
        // Parse result
        println!("{}", data);
        let result: Result<EpisodeRecordData, serde_json::Error> = serde_json::from_str(&data);
        match result {
            Ok(r) => Ok(r.data.unwrap()),
            Err(e) => Err(TvdbError::DataError {
                reason: e.to_string(),
            }),
        }
    }

    /// Full information about given episode
    /// <https://api.thetvdb.com/swagger#!/Episodes/get_episodes_id>
    pub fn episode<T: Into<EpisodeId>>(&self, id: T) -> TvdbResult<Episode> {
        self.episode_inner(id.into())
    }

    fn series_episodes_inner(&self, id: SeriesId, page: u32) -> TvdbResult<SeriesEpisodes> {
        let dc = self.default_client.as_ref();
        let c = self.http_client.unwrap_or(dc);

        // TODO Use `id.language`

        let url = format!(
            "https://api.thetvdb.com/series/{id}/episodes?page={page}",
            id = id.seriesid,
            page = page
        );
        let data = c.get_url(&url, self.get_token())?;
        // Parse result
        let result: Result<SeriesEpisodes, serde_json::Error> = serde_json::from_str(&data);
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(TvdbError::DataError {
                reason: e.to_string(),
            }),
        }
    }

    /// All episodes for given series
    pub fn series_episodes<T: Into<SeriesId>>(
        &self,
        id: T,
        page: u32,
    ) -> TvdbResult<SeriesEpisodes> {
        self.series_episodes_inner(id.into(), page)
    }
}
