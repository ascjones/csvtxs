extern crate csv;
extern crate chrono;

// #[macro_use]
// extern crate serde_derive;
// extern crate toml;

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

struct Rule {
    description: String,
    account: String,
}

struct MatchingRules {
    rules: Vec<Rule>
}

struct LedgerTransaction {
    date: NaiveDate,
    amount: f32,
    description: String,
    account: String,
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

impl MatchingRules {
    fn match_transaction(&self, tx: &Transaction) -> Option<&Rule> {
        self.rules.iter().find(|&rule| rule.description == tx.description)
    }
}

fn write_txs(account: &str, txs: Vec<LedgerTransaction>) -> Result<(), Box<Error>> {
    for tx in txs {
        println!("{} * {}", tx.date, tx.description);
        println!("    {} £{:.2}", tx.account, tx.amount);
        println!("    {} £-{:.2}", account, tx.amount);
        println!("");
    }
    Ok(())
}

fn main() {
    // options to be passed in
    let date_fmt = "%d/%m/%Y";
    let account = "Liabilities:Amex";
    let rules = 
        MatchingRules {
            rules : vec! [
                Rule { 
                    description : "AMAZON UK RETAIL AMAZON.CO.UK".to_string(), 
                    account : "Expenses:Amazon".to_string() 
                }
            ]
        };


    let txs = readcsv(&date_fmt).unwrap();
    let tx_rules =
        txs.into_iter()
            .map (|tx| {
                let rule = rules.match_transaction(&tx);
                (tx, rule)
            });

    let matched =
        tx_rules
            .filter_map(|(tx, rule)| {
                rule.map(|r| {
                    LedgerTransaction { 
                        date : tx.date,
                        amount : tx.amount,
                        description : tx.description.to_owned(), 
                        account : r.account.to_owned() 
                    }
                })
            }).collect();

    // todo: unmatched

    write_txs(&account, matched);
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
