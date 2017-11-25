extern crate csv;
extern crate chrono;

use std::error::Error;
use std::io;
use chrono::{NaiveDate};

#[derive(Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub amount: f32,
    pub description: String
}

pub struct CategorisedTransaction {
    pub transaction: Transaction,
    pub account: String
}

impl Transaction {
    pub fn new(date: NaiveDate, amount: f32, description: &str) -> Self {
        Transaction { 
            date : date,
            amount : amount,
            description : description.to_owned()
        }
    }
}

pub fn read_txs(date_fmt: &str) -> Result<Vec<Transaction>, Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    rdr.records()
        .map(|result| {
            let record = result?;
            let date = NaiveDate::parse_from_str(&record[0], date_fmt);
            let amount: f32 = record[2].trim().parse()?;
            let description = record[3].to_string();
            Ok(Transaction::new(date?, amount, &description))
        }).collect()
}

pub fn write_txs(account: &str  , txs: Vec<CategorisedTransaction>) -> Result<(), Box<Error>> {
    for tx in txs {
        println!("{} * {}", tx.transaction.date, tx.transaction.description);
        println!("    {} £{:.2}", tx.account, tx.transaction.amount);
        println!("    {} £-{:.2}", account, tx.transaction.amount);
        println!("");
    }
    Ok(())
}