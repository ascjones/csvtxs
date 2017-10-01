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

fn readcsv(date_fmt: &str) -> Result<Vec<Transaction>, Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    rdr.records()
        .map(|result| {
            let record = result?;
            let date = NaiveDate::parse_from_str(&record[0], date_fmt);
            let amount: f32 = record[2].trim().parse()?;
            let description = record[3].to_string();
            Ok(Transaction { date : date?, amount : amount, description : description })
        }).collect()
}

fn write_txs(account: &str, txs: Vec<Transaction>) -> Result<(), Box<Error>> {
    for tx in txs {
        println!("{} * {}", tx.date, tx.description);
        println!("    {} £{:.2}", account, tx.amount);
        println!("");
    }
    Ok(())
}

fn main() {
    // options to be passed in
    let date_fmt = "%d/%m/%Y";
    let account = "Liabilities:Amex";

    let txs = readcsv(&date_fmt).unwrap();
    write_txs(&account, txs);
    // if let Err(err) = readcsv(&date_fmt) {
    //     println!("error running readcsv: {}", err);
    //     process::exit(1);
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
