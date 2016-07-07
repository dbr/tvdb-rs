extern crate tvdb;
extern crate argparse;
extern crate rand;
extern crate env_logger;

use argparse::{ArgumentParser, StoreTrue, Store};
use rand::{Rng, SeedableRng, StdRng};


fn main(){
    env_logger::init().unwrap();

    //let mut series_name = "".to_owned();
    //let mut season_no = 1;
    //let mut episode_no = 1;

    {
        let mut ap = ArgumentParser::new();
        //ap.refer(&mut series_name).add_argument("series", Store, "Series name");
        //ap.refer(&mut season_no).add_argument("season", Store, "Season number");
        ap.parse_args_or_exit();
    }

    // Construct API object
    let api = tvdb::Tvdb::new("0629B785CE550C8D");

    // Opening a bunch of ~random series to check for panicing
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 1..100{
        let rid = rng.gen_range::<u32>(70000, 80000);
        println!("Getting series {}", rid);

        let ep = api.episode(tvdb::EpisodeId::new(rid, "en"), 1, 2);
        match ep{
            Ok(ep) => println!("Okay  ID {}: {}", rid, ep.episode_name),
            Err(e) => println!("Error ID {}: {:?}", rid, e),
        }
    }

}
