extern crate tvdb;
extern crate rand;

use tvdb::{Tvdb, EpisodeId, TvdbResult, TvdbError};

const APIKEY: &'static str = "0629B785CE550C8D";

#[test]
fn search() {
    let api = Tvdb::new(APIKEY.to_owned());
    let sr = api.search("scrubs");
    assert!(sr.ok().unwrap().data[0].series_name == "Scrubs");
}

#[test]
fn nonexist() {
    let api = Tvdb::new(APIKEY);
    let sr = api.search("ladlkgdklfgsdfglk");
    assert!(sr.is_err());
}

#[test]
fn lookup_by_epid(){
    let api = Tvdb::new(APIKEY);
    api.login().unwrap();
    let ep = api.episode(EpisodeId::new(184603, "en"));
    println!("Episode: {:?}", ep);
    assert!(ep.unwrap().episode_name == "My Mentor");
}

#[test]
fn lookup_by_u32(){
    let api = Tvdb::new(APIKEY);
    api.login().unwrap();
    let ep = api.episode(184603);
    println!("Episode; {:?}", ep);
    assert!(ep.unwrap().episode_name == "My Mentor");
}

#[test]
fn random_series(){
    // Opening a bunch of ~random series to check for panicing
    use rand::{Rng, SeedableRng, StdRng};

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 1..10{
        let rid = rng.gen_range::<u32>(1, 20000);
        println!("Getting series {}", rid);

        let api = Tvdb::new(APIKEY);
        let ep = api.episode(EpisodeId::new(rid, "en"));
        println!("{:?}", ep);
        match ep{
            Ok(ep) => println!("{}", ep.episode_name),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Debug,Clone)]
struct DummyRequestClient;

impl DummyRequestClient{
    pub fn new() -> DummyRequestClient{
        return DummyRequestClient{};
    }
}
use tvdb::RequestClient;
impl RequestClient for DummyRequestClient{
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String>{
        return Err(TvdbError::CommunicationError{reason: "Fake error!".into()});
    }
}

#[test]
fn custom_http_client() {
    let c = DummyRequestClient::new();

    let mut api = Tvdb::new(APIKEY);
    api.set_http_client(&c);

    let result = api.search("scrubs");
    println!("{:?}", result);

    match result{
        Ok(_) => panic!("Expected error"),
        Err(e) => (
            match e {
                TvdbError::CommunicationError{reason: _} => (),
                _ => panic!("Unexpected"),
            }
        ),
    }
}

#[test]
fn all_episodes(){
    let api = Tvdb::new(APIKEY.to_owned());
    api.login().unwrap();
    let sr = api.search("scrubs").unwrap();
    let first_id = sr.data[0].id.unwrap();
    let eps = api.series_episodes(first_id).unwrap();
    let data = eps.data.unwrap();
    assert!(data.len() > 10);
    let ep = data[0].clone();
    assert!(ep.episode_name.unwrap() == "My First Day");
}
