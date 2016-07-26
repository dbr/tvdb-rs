/// Used for air-date of an episode etc
#[derive(Debug,Clone,Copy)]
pub struct Date {
    /// Year (e.g 2001)
    pub year: u32,
    /// Number of month (e.g 1-12)
    pub month: u32,
    /// Day of month (e.g 1-31)
    pub day: u32,
}


/// Series ID from TheTVDB.com, along with language
#[derive(Debug,Clone)]
pub struct EpisodeId{
    /// Series ID
    pub seriesid: u32,
    /// Language code
    pub language: String,
}

impl EpisodeId{
    /// Constructor
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


/// Series info as returned from [TheTVDB's series search
/// method](http://www.thetvdb.com/wiki/index.php?title=API:GetSeries)
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


/// Base episode record, [details on TvDB Wiki](http://www.thetvdb.com/wiki/index.php?title=API:Base_Episode_Record)
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
