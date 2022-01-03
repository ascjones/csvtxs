use crate::{
    config::RuleEntry,
    transaction::{CategorisedTransaction, Transaction},
};
use regex::Regex;

#[derive(Debug)]
pub struct Rule {
    description: Regex,
    account: String,
}

impl TryFrom<&RuleEntry> for Rule {
    type Error = regex::Error;

    fn try_from(value: &RuleEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            description: Regex::new(&value.description)?,
            account: value.account.clone(),
        })
    }
}

#[derive(Debug)]
pub struct MatchingRules {
    rules: Vec<Rule>,
}

impl MatchingRules {
    pub fn from_config(rules: &[RuleEntry]) -> color_eyre::Result<MatchingRules> {
        let rules = rules
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { rules })
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
    use crate::config::{CsvColumns, CsvImports};
    use crate::transaction::{read_txs, CategorisedTransaction, Transaction};
    use chrono::NaiveDate;
    use currency::Currency;

    #[test]
    fn rule_matches_description_with_regex() {
        let txs_csv = "\
date,,amount,description
10/12/2017,,10.00,PURCHASE FROM AMAZON";
        let rule_entries = vec![RuleEntry {
            description: "AMAZON".to_string(),
            account: "Expenses::Amazon".to_string(),
        }];
        let rules = MatchingRules::from_config(&rule_entries).unwrap();
        let csv_config = CsvImports {
            account: "Account".to_string(),
            date_format: "%d/%m/%Y".to_string(),
            columns: CsvColumns {
                date: 0,
                amount: 2,
                description: 3,
            },
        };
        let txs = read_txs(&csv_config, txs_csv.as_bytes()).unwrap();
        let (matched, _) = rules.match_transactions(txs);
        assert_eq!(
            matched,
            vec![CategorisedTransaction {
                transaction: Transaction {
                    date: NaiveDate::from_ymd(2017, 12, 10),
                    amount: Currency::from_str("10.00").unwrap(),
                    description: "PURCHASE FROM AMAZON".to_string()
                },
                account: "Expenses::Amazon".to_string()
            }]
        );
    }
}
