extern crate rand;
extern crate tvdb;

use rand::{Rng, SeedableRng};

use tvdb::{EpisodeId, Tvdb, TvdbError, TvdbResult};

const APIKEY: &'static str = "0629B785CE550C8D";

#[test]
fn search() {
    let api = Tvdb::new(APIKEY.to_owned());
    api.login().unwrap();
    let sr = api.search(Some("scrubs"), None);
    println!("{:?}", sr);
    assert!(sr.ok().unwrap().data.unwrap()[0].series_name == "Scrubs");
}

#[test]
fn nonexist() {
    let api = Tvdb::new(APIKEY);
    let sr = api.search(Some("ladlkgdklfgsdfglk"), None);
    assert!(sr.is_err());
}

#[test]
fn lookup_by_epid() {
    let api = Tvdb::new(APIKEY);
    api.login().unwrap();
    let ep = api.episode(EpisodeId::new(184603, "en"));
    println!("Episode: {:?}", ep);
    assert!(ep.unwrap().data.unwrap().episode_name == "My Mentor");
}

#[test]
fn lookup_by_u32() {
    let api = Tvdb::new(APIKEY);
    api.login().unwrap();
    let ep = api.episode(184603);
    println!("Episode; {:?}", ep);
    assert!(ep.unwrap().data.unwrap().episode_name == "My Mentor");
}

#[test]
fn random_series() {
    // Opening a bunch of ~random series to check for panicing
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    for _ in 1..10 {
        let rid: u32 = rng.gen_range(1, 20000);
        println!("Getting series {}", rid);

        let api = Tvdb::new(APIKEY);
        let ep = api.episode(EpisodeId::new(rid, "en"));
        println!("{:?}", ep);
        match ep {
            Ok(ep) => println!("{}", ep.data.unwrap().episode_name),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Debug, Clone)]
struct DummyRequestClient;

impl DummyRequestClient {
    pub fn new() -> DummyRequestClient {
        return DummyRequestClient {};
    }
}
use tvdb::RequestClient;
impl RequestClient for DummyRequestClient {
    fn get_url(&self, url: &str, jwt_token: Option<String>) -> TvdbResult<String> {
        return Err(TvdbError::CommunicationError {
            reason: format!(
                "Fake error while doing fake request for: {:?} with JWT {:?}",
                url, jwt_token,
            ),
        });
    }
}

#[test]
fn custom_http_client() {
    let c = DummyRequestClient::new();

    let mut api = Tvdb::new(APIKEY);
    api.set_http_client(&c);

    let result = api.search(Some("scrubs"), None);
    println!("{:?}", result);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            match e {
                TvdbError::CommunicationError { reason: _ } => (),
                _ => panic!("Unexpected"),
            }
        }
    }
}

#[test]
fn all_episodes() {
    let api = Tvdb::new(APIKEY.to_owned());
    api.login().unwrap();
    let sr = api.search(Some("scrubs"), None).unwrap();
    let first_id = sr.data.unwrap()[0].id.unwrap();
    let eps = api.series_episodes(first_id, 1).unwrap();
    let data = eps.data.unwrap();
    assert!(data.len() > 10);
    let ep = data[0].clone();
    assert!(ep.episode_name.unwrap() == "My First Day");
}
