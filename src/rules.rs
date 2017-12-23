extern crate csv;

use std::error::Error;
use transaction::{Transaction, CategorisedTransaction};
use std::io::{Read};
use regex::Regex;

#[derive(Debug)]
pub struct Rule {
    description: Regex,
    account: String,
}

#[derive(Debug)]
pub struct MatchingRules {
    rules: Vec<Rule>
}

impl MatchingRules {
    pub fn read_csv<R>(reader: R) -> Result<MatchingRules, Box<Error>> 
        where R: Read 
    {
        let mut rdr = csv::Reader::from_reader(reader);
        let rules : Result<Vec<Rule>, Box<Error>> = 
            rdr.records()
                .map(|result| {
                    result.map(|record| {
                        Rule {
                            description: Regex::new(&record[0]).unwrap(),
                            account: record[1].to_string()
                        }
                    }).map_err(From::from)
            }).collect();
        rules.map(|rs| { MatchingRules { rules: rs } })
    }

    pub fn match_transactions(&self, txs: Vec<Transaction>) -> Vec<CategorisedTransaction> {
        // let rules_with_regex =
        //     self.rules.into_iter()
        //         .map (|rule| (rule, Regex::new(&rule.description.to_string()).unwrap()));
        txs.into_iter()
            .filter_map (|tx| {
                self.rules.iter()
                    .find(|rule| rule.description.is_match(&tx.description))
                    .map(|rule| CategorisedTransaction { transaction: tx, account: rule.account.to_string() })
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate};
    use rules::{MatchingRules};
    use transaction::{read_txs, Transaction, CategorisedTransaction};
    use currency::Currency;

    #[test]
    fn read_rule_from_csv() {
        let rules_csv = "\
description,account
AMAZON,Expenses:Amazon";
        let rules = MatchingRules::read_csv(rules_csv.as_bytes()).unwrap();
        assert_eq!(format!("{}", rules.rules[0].description), "AMAZON".to_string());
        assert_eq!(rules.rules[0].account, "Expenses:Amazon".to_string());
    }

    #[test]
    fn rule_matches_description_with_regex() {
        let rules_csv = "\
date, account
AMAZ.*,Expenses:Amazon"; 
        let txs_csv = "\
date,,amount,description
10/12/2017,,10.00,PURCHASE FROM AMAZON";
        let rules = MatchingRules::read_csv(rules_csv.as_bytes()).unwrap();
        let txs = read_txs("%d/%m/%Y", txs_csv.as_bytes()).unwrap();
        let matched = rules.match_transactions(txs);
        assert_eq!(matched, vec! [CategorisedTransaction {
            transaction: Transaction {
                date: NaiveDate::from_ymd(2017, 12, 10),
                amount: Currency::from_str("10.00").unwrap(),
                description: "PURCHASE FROM AMAZON".to_string()
            },
            account: "Expenses:Amazon".to_string()
        }]);
    }
}

