pub struct SeriesId {
    pub seriesid: u32,
}

impl From<u32> for SeriesId {
    fn from(x: u32) -> Self {
        SeriesId { seriesid: x }
    }
}

#[derive(Debug, Clone)]
pub struct EpisodeId {
    /// Series ID
    pub seriesid: u32,
    /// Language code
    pub language: String,
}

impl EpisodeId {
    /// Constructor
    pub fn new(seriesid: u32, lang: &str) -> EpisodeId {
        EpisodeId {
            seriesid: seriesid,
            language: lang.to_owned(),
        }
    }
}

impl From<u32> for EpisodeId {
    fn from(x: u32) -> Self {
        EpisodeId {
            seriesid: x,
            language: "en".to_owned(),
        }
    }
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
#[serde(rename_all = "camelCase")]
pub struct SeriesSearchData {
    pub aliases: Option<Vec<String>>,
    pub banner: Option<String>,
    pub first_aired: Option<String>,
    pub id: Option<u32>,
    pub network: Option<String>,
    pub overview: Option<String>,
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
pub struct EpisodeRecordResult {
    pub data: Option<Episode>,
    pub errors: Option<JSONErrors>,
}

/// Complete info for an episode
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
    pub dvd_chapter: Option<f32>,
    pub dvd_discid: Option<String>,
    pub dvd_episode_number: Option<f32>,
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
pub struct SeriesEpisodesResult {
    pub data: Option<Vec<BasicEpisode>>,
    pub errors: Option<JSONErrors>,
    pub links: Option<Links>,
}

/// Episode with most common attributes available
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasicEpisode {
    pub absolute_number: Option<u32>,
    pub aired_episode_number: Option<u32>,
    pub aired_season: Option<u32>,
    pub dvd_episode_number: Option<f32>,
    pub dvd_season: Option<u32>,
    pub episode_name: Option<String>,
    pub first_aired: Option<String>,
    pub id: Option<u32>,
    pub last_updated: Option<u32>,
    pub overview: Option<String>,
}

/// Pagination links
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub first: Option<u32>,
    pub last: Option<u32>,
    pub next: Option<u32>,
    pub previous: Option<u32>,
}
