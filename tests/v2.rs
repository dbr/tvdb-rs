extern crate tvdb;
use tvdb::raw::Tvdb;

static APIKEY: &'static str = "0629B785CE550C8D";

#[test]
fn basic() {
    let t = Tvdb::new(APIKEY);
    t.login().unwrap();
    let sr = t.search(Some("scrubs"), None).unwrap();
    for s in sr.data.unwrap().iter() {
        println!("{:?}: ID {:?}", s.series_name, s.id);
    }
}

#[test]
fn search_by_imdb() {
    let t = Tvdb::new(APIKEY);
    t.login().unwrap();
    let sr = t.search(None, Some("tt0285403")).unwrap();
    for s in sr.data.unwrap().iter() {
        println!("{:?}: ID {:?}", s.series_name, s.id);
    }
}
