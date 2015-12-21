extern crate tvdb;

fn main(){
    if std::env::args().count() != 4{
        println!("Usage {} <seriesname> <season number> <episode number>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }

    // Parse arguments
    let series_name = std::env::args().nth(1).unwrap();
    let season_no = std::env::args().nth(2).unwrap().parse::<u32>().unwrap();
    let episode_no = std::env::args().nth(3).unwrap().parse::<u32>().unwrap();

    // Construct API object
    let api = tvdb::Tvdb::new("0629B785CE550C8D");

    // Search for series
    let sr = api.search(series_name, "en".to_owned()).ok().unwrap();

    // Loop over found series
    for r in sr.iter(){
        // Print: "Series Name" (id: 12345)
        println!("{:?} (id: {})", r.seriesname, r.seriesid);

        // Get episode information
        println!("{:?}", api.episode(&sr[0], season_no, episode_no));
    }
}
