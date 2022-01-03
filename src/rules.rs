use crate::transaction::{CategorisedTransaction, Transaction};
use regex::Regex;
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct Rule {
    description: Regex,
    account: String,
}

#[derive(Debug)]
pub struct MatchingRules {
    rules: Vec<Rule>,
}

impl MatchingRules {
    pub fn read_csv<R>(reader: R) -> Result<MatchingRules, Box<dyn Error>>
    where
        R: Read,
    {
        let mut rdr = csv::Reader::from_reader(reader);
        let rules: Result<Vec<Rule>, Box<dyn Error>> = rdr
            .records()
            .map(|result| {
                result
                    .map(|record| Rule {
                        description: Regex::new(&record[0]).unwrap(),
                        account: record[1].to_string(),
                    })
                    .map_err(From::from)
            })
            .collect();
        rules.map(|rs| MatchingRules { rules: rs })
    }

    pub fn match_transactions(
        &self,
        txs: Vec<Transaction>,
    ) -> (Vec<CategorisedTransaction>, Vec<Transaction>) {
        let mut categorised: Vec<CategorisedTransaction> = Vec::new();
        let mut uncategorised: Vec<Transaction> = Vec::new();
        for tx in txs {
            let rule = self
                .rules
                .iter()
                .find(|rule| rule.description.is_match(&tx.description));
            match rule {
                Some(r) => categorised.push(CategorisedTransaction {
                    transaction: tx,
                    account: r.account.to_string(),
                }),
                None => uncategorised.push(tx),
            }
        }
        (categorised, uncategorised)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{read_txs, CategorisedTransaction, Transaction};
    use chrono::NaiveDate;
    use currency::Currency;

    #[test]
    fn read_rule_from_csv() {
        let rules_csv = "\
description,account
AMAZON,Expenses:Amazon";
        let rules = MatchingRules::read_csv(rules_csv.as_bytes()).unwrap();
        assert_eq!(
            format!("{}", rules.rules[0].description),
            "AMAZON".to_string()
        );
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
        let (matched, _) = rules.match_transactions(txs);
        assert_eq!(
            matched,
            vec![CategorisedTransaction {
                transaction: Transaction {
                    date: NaiveDate::from_ymd(2017, 12, 10),
                    amount: Currency::from_str("10.00").unwrap(),
                    description: "PURCHASE FROM AMAZON".to_string()
                },
                account: "Expenses:Amazon".to_string()
            }]
        );
    }
}
