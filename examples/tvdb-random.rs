extern crate argparse;
extern crate env_logger;
extern crate rand;
extern crate tvdb;

use argparse::{ArgumentParser, Store};
use rand::{Rng, SeedableRng, StdRng};

fn main() {
    env_logger::init();

    let mut num: u32 = 1;
    let mut season_no = 1;
    let mut episode_no = 1;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut num).add_option(
            &["-n", "--number"],
            Store,
            "Number of random series to parse",
        );
        ap.refer(&mut season_no)
            .add_option(&["-s", "--season"], Store, "Season number");
        ap.refer(&mut episode_no)
            .add_option(&["-e", "--episode"], Store, "Episode number");
        ap.parse_args_or_exit();
    }

    // Construct API object
    let api = tvdb::Tvdb::new("0629B785CE550C8D");

    // Opening a bunch of ~random series to check for panicing
    let seed: &[_] = &[1, 2, 3, 4, 5];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 0..num {
        let rid = rng.gen_range::<u32>(70000, 80000);
        println!("Getting series {}", rid);

        let ep = api.episode(tvdb::EpisodeId::new(rid, "en"));
        match ep {
            Ok(ep) => println!("Okay  ID {}: {}", rid, ep.data.unwrap().episode_name),
            Err(e) => println!("Error ID {}: {:?}", rid, e),
        }
    }
}
