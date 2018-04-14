/// Used for air-date of an episode etc
#[derive(Debug, Clone, Copy)]
pub struct Date {
    /// Year (e.g 2001)
    pub year: u32,
    /// Number of month (e.g 1-12)
    pub month: u32,
    /// Day of month (e.g 1-31)
    pub day: u32,
}

pub struct SeriesId {
    pub seriesid: u32,
}

impl From<u32> for SeriesId {
    fn from(x: u32) -> Self {
        SeriesId { seriesid: x }
    }
}

/// Series ID from TheTVDB.com, along with language
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
