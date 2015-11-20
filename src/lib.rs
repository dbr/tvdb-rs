extern crate xmltree;
extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

use hyper::Url;


/// Turns "123" into 123
pub fn intify(instr: &str) -> i32{
    // TODO: Better error handling
    instr.to_owned().parse::<i32>().unwrap()
}


/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
    DataError{reason: String},
    Cancelled,
}


#[derive(Debug,Clone)]
pub struct SeriesSearchResult{
    pub id: i32, // TODO: Is this any different to seriesid?
    pub seriesname: String,
    pub language: String,
    pub overview: Option<String>,
    pub banner: Option<String>,
    pub imdb_id: Option<String>,
    //pub firstaired: Date,
    pub network: Option<String>,
    pub zap2it_id: Option<String>,
}

pub struct Tvdb{
    key: String,
}

impl Tvdb{
    fn new(key: String) -> Tvdb{
        Tvdb{key: key}
    }

    fn search(&self, name: String, lang: String) -> Result<Vec<SeriesSearchResult>, TvdbError>{
        let client = Client::new();

        let formatted_url = format!("http://thetvdb.com/api/GetSeries.php?seriesname={}", name);
        let url = Url::parse(&formatted_url).ok().expect("invalid URL");
        println!("Getting {}", url);

        let res = client.get(url)
            .header(Connection::close())
            .send();

        let mut res = match res {
            Err(e) => return Err(TvdbError::CommunicationError{reason: "Error contacting TVDB".to_owned()}), // FIXME: http://stackoverflow.com/questions/28911833/error-handling-best-practices
            Ok(r) => r
        };

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        // Parse XML
        let tree = xmltree::Element::parse(body.as_bytes());

        // Convert XML into structs
        let mut results : Vec<SeriesSearchResult> = vec![];

        for child in tree.children.iter(){

            fn get_text(child: &xmltree::Element, x: &str) -> Option<String>{
                child.get_child(x).and_then(|id_child| id_child.text.clone())
            }

            let r = SeriesSearchResult{
                id:         intify(&get_text(child, "id").expect("Search result XML missing 'id' element")),
                seriesname: get_text(child, "SeriesName").expect("Search result XML Missing 'SeriesName' element"),
                language:   get_text(child, "language").expect("Search result XML missing 'language' element"),
                overview:   get_text(child, "Overview"),
                banner:     get_text(child, "banner"),
                imdb_id:    get_text(child, "IMDB_ID"),
                //firstaired: Date,
                network:    get_text(child, "Network"),
                zap2it_id:  get_text(child, "zap2it_id"),
            };

            results.push(r);
        }

        if results.is_empty(){
            return Err(TvdbError::SeriesNotFound);
        }

        return Ok(results);
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn search() {
        let api = Tvdb::new("APIKEY".to_owned());
        let sr = api.search("scrubs".to_owned(), "en".to_owned());
        assert!(sr.ok().unwrap()[0].seriesname == "Scrubs");
    }

    #[test]
    fn nonexist() {
        let api = Tvdb::new("APIKEY".to_owned());
        let sr = api.search("ladlkgdklfgsdfglk".to_owned(), "en".to_owned());
        assert!(sr.is_err());
    }

}
