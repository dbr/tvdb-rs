extern crate tvdb;

use tvdb::{Tvdb, EpisodeId};

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
    assert!(ep.episode_name == "My Mentor");
}

#[test]
fn lookup_by_u32(){
    let api = Tvdb::new(APIKEY);
    let ep = api.episode(76156, 1, 2).ok().unwrap();
    assert!(ep.episode_name == "My Mentor");
}

#[test]
fn epinfo_default(){
    let api = Tvdb::new(APIKEY);
    let sr = api.search("scrubs", "en").ok().unwrap();
    let ep = api.episode(&sr[0], 1, 2).ok().unwrap();
    assert!(ep.episode_name == "My Mentor");
}
