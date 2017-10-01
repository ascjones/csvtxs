extern crate csv;
extern crate chrono;

use std::error::Error;
use std::io;
use std::process;
use chrono::{NaiveDate};

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,
    amount: f32,
    description: String,
}

fn readcsv(date_fmt: &str) -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        let date = NaiveDate::parse_from_str(&record[0], date_fmt);
        let amount: f32 = record[2].trim().parse()?;
        let description = record[3].to_string();
        let tx = Transaction { date : date?, amount : amount, description : description };
        println!("{:?}", tx);
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
