/// Provides mostly direct binding to the HTTP API

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::rc::Rc;

use reqwest;
use serde_json;
use url;

use super::data::*;
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
        let client = reqwest::blocking::Client::new();
        let mut req = client.get(url);

        // Add auth header
        if let Some(tok) = jwt_token {
            req = req.bearer_auth(tok);
        }

        // Send request
        let mut resp = req.send().map_err(|x| {
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

        let c = reqwest::blocking::Client::new();
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

        let deserialized: serde_json::Value = serde_json::from_str(&result)?;
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
        let result: SeriesSearchResult = serde_json::from_str(&data)?;

        Ok(result)
    }

    fn episode_inner(&self, id: EpisodeId) -> TvdbResult<EpisodeRecordResult> {
        let dc = self.default_client.as_ref();
        let c = self.http_client.unwrap_or(dc);

        // TODO Use `id.language`

        let url = format!("https://api.thetvdb.com/episodes/{id}", id = id.seriesid);
        let data = c.get_url(&url, self.get_token())?;
        // Parse result
        let result: EpisodeRecordResult = serde_json::from_str(&data)?;
        return Ok(result);
    }

    /// Full information about given episode
    /// <https://api.thetvdb.com/swagger#!/Episodes/get_episodes_id>
    pub fn episode<E>(&self, id: E) -> TvdbResult<EpisodeRecordResult>
    where
        E: Into<EpisodeId>,
    {
        self.episode_inner(id.into())
    }

    fn series_episodes_inner(&self, id: SeriesId, page: u32) -> TvdbResult<SeriesEpisodesResult> {
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
        let result: Result<SeriesEpisodesResult, serde_json::Error> = serde_json::from_str(&data);
        return Ok(result?);
    }

    /// All episodes for given series
    pub fn series_episodes<S>(&self, id: S, page: u32) -> TvdbResult<SeriesEpisodesResult>
    where
        S: Into<SeriesId>,
    {
        self.series_episodes_inner(id.into(), page)
    }
}
