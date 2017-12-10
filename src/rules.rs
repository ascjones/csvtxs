extern crate csv;

use std::error::Error;
use transaction::{Transaction, CategorisedTransaction};
use std::io::{Read};

#[derive(Debug, Eq, PartialEq)]
pub struct Rule {
    description: String,
    account: String,
}

#[derive(Debug, Eq, PartialEq)]
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
                            description: record[0].to_string(),
                            account: record[1].to_string()
                        }
                    }).map_err(From::from)
            }).collect();
        rules.map(|rs| { MatchingRules { rules: rs } })
    }

    pub fn match_transactions(&self, txs: Vec<Transaction>) -> Vec<CategorisedTransaction> {
        txs.into_iter()
            .filter_map (|tx| {
                self.rules.iter()
                    .find(|rule| rule.description == tx.description)
                    .map(|r| CategorisedTransaction { transaction: tx, account: r.account.to_string() })
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use rules::{MatchingRules, Rule};

    #[test]
    fn read_rule_from_csv() {
        let rules_csv = "\
description,account
AMAZON,Expenses:Amazon";
        let rules = MatchingRules::read_csv(rules_csv.as_bytes()).unwrap();
        let expected_rule = 
            Rule {
                description: "AMAZON".to_string(),
                account: "Expenses:Amazon".to_string()
            };
        assert_eq!(rules.rules[0], expected_rule);
    }
}

