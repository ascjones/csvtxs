extern crate csv;
extern crate chrono;
extern crate currency;
extern crate regex;

mod transaction;
mod rules;

use std::fs::{File};
use std::io;
// use std::process;
use transaction::{read_txs, write_txs};
use rules::{MatchingRules, Rule};

fn main() {
    // options to be passed in
    let date_fmt = "%d/%m/%Y";
    let account = "Liabilities:Amex";
    let rules_csv = File::open("rules.csv").unwrap(); // env::current_dir().unwrap();
    // let default_account2 = "Expenses:Unknown";
    
    let rules = MatchingRules::read_csv(rules_csv).unwrap();

    let txs = read_txs(&date_fmt, std::io::stdin()).unwrap();
    
    let (matched, unmatched) = rules.match_transactions(txs);

    if unmatched.len() > 0 {
        println!("{} transactions could not be categorised with existing rules", 
            unmatched.len());
        let new_rules: Vec<Rule> = Vec::new();
        for tx in unmatched {
            // ask at command line for new account
            println!("");
            println!("No matching rule for: {}, {}, {}", tx.date, tx.description, tx.amount);

            let mut account = String::new();
            println!("Enter account:");

            io::stdin().read_line(&mut account)
                .expect("Failed to read line");

            println!("The accound you entered was: {}", account);

            // default rule to description
            // push new rule to vector
        }
    }

    // todo: iterate over unmatched and any unmatched add rule and rerun
    // todo: tab automcomplete accounts (from rules?)
    // todo: write new rules once finished (give summary and option to cancel)

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
