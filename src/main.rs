extern crate csv;
extern crate chrono;
extern crate currency;
extern crate regex;

mod transaction;
mod rules;

use std::fs::{File};
// use std::process;
use transaction::{read_txs, write_txs};
use rules::{MatchingRules};

fn main() {
    // options to be passed in
    let date_fmt = "%d/%m/%Y";
    let account = "Liabilities:Amex";
    let rules_csv = File::open("rules.csv").unwrap(); // env::current_dir().unwrap();
    // let default_account2 = "Expenses:Unknown";
    
    let rules = MatchingRules::read_csv(rules_csv).unwrap();

    let txs = read_txs(&date_fmt, std::io::stdin()).unwrap();
    
    let matched = rules.match_transactions(txs);

    // todo: unmatched

    write_txs(&account, matched).unwrap()
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
