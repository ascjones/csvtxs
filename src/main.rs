extern crate csv;
extern crate chrono;

use std::error::Error;
use std::io;
use std::process;
use chrono::{NaiveDate};

struct Transaction {
    date: String
}

fn readcsv(date_fmt: &str) -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        let date = NaiveDate::parse_from_str(&record[0], date_fmt);
        println!("date {:?}", date);
    }
    Ok(())
}

fn main() {
    let date_fmt = "%d/%m/%Y";
    if let Err(err) = readcsv(&date_fmt) {
        println!("error running readcsv: {}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
