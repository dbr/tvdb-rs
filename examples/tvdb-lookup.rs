extern crate tvdb;

fn main(){
    if std::env::args().count() != 4{
        println!("Usage {} <seriesname> <season number> <episode number>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }
    let api = tvdb::Tvdb::new("0629B785CE550C8D".to_owned());
    let sr = api.search(std::env::args().nth(1).unwrap(), "en".to_owned()).ok().unwrap();
    for r in sr.iter(){
        println!("{:?} (id: {})", r.seriesname, r.seriesid);
        println!("{:?}", api.episode(sr[0].clone(), 1, 2));
    }
}
