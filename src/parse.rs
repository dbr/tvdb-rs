use error::TvdbError;
use error::TvdbResult;
use data::Date;

use std::num::ParseIntError;
use std::num::ParseFloatError;

/// Turns "123" into 123
pub fn intify(instr: &str) -> Result<u32, ParseIntError>{
    instr.to_owned().parse::<u32>()
}

/// Turns "123.1" into 123.1
pub fn floatify(instr: &str) -> Result<f32, ParseFloatError>{
    instr.to_owned().parse::<f32>()
}

/// Parse YYYY-MM-DD formatted string into `Date` struct
pub fn dateify(instr: &str) -> TvdbResult<Date>{
    let invalid_date = || {TvdbError::DataError{reason: format!("Malformed YYYY-MM-DD date: {}", instr)}};

    let chunks:Vec<&str> = instr.split("-").collect();
    if chunks.len() != 3 {
        return Err(invalid_date());
    }

    let year  = try!(chunks.get(0).ok_or(invalid_date()));
    let month = try!(chunks.get(1).ok_or(invalid_date()));
    let day   = try!(chunks.get(2).ok_or(invalid_date()));

    Ok(Date{
        year: try!(intify(year)),
        month: try!(intify(month)),
        day: try!(intify(day)),
    })
}

#[test]
fn test_date_parser_good() {
    let d = dateify("2001-02-03");
    println!("Parsed date as {:?}", d);

    assert!(d.is_ok());
    let d = d.unwrap();
    assert!(d.year == 2001);
    assert!(d.month == 2);
    assert!(d.day == 3);
}


#[test]
fn test_date_parser_bad() {
    assert!(dateify("blah").is_err());
    assert!(dateify("2001-02").is_err());
    assert!(dateify("2001-02-blah").is_err());
}
