extern crate tvdb;
use tvdb::v2::Tvdb;

#[test]
fn basic() {
    let t = Tvdb::new("0629B785CE550C8D");
    t.login().unwrap();
    println!("{:?}", t.search("scrubs"));
    panic!();
}
