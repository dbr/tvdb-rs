extern crate argparse;
extern crate tvdb;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut series_name = "".to_owned();
    let mut season_no = 1;
    let mut episode_no = 1;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut series_name)
            .add_argument("series", Store, "Series name");
        ap.refer(&mut season_no)
            .add_argument("season", Store, "Season number");
        ap.refer(&mut episode_no)
            .add_argument("episode", Store, "Episode number");
        ap.parse_args_or_exit();
    }

    // Construct API object
    let api = tvdb::Tvdb::new("0629B785CE550C8D");
    // FIXME
}
