use transaction::Transaction;

pub struct Rule {
    description: String,
    account: String,
}

pub struct MatchingRules {
    rules: Vec<Rule>
}

impl MatchingRules {
    pub fn read() -> Self {
        MatchingRules {
            rules : vec! [
                Rule { 
                    description : "AMAZON UK RETAIL AMAZON.CO.UK".to_string(), 
                    account : "Expenses:Amazon".to_string() 
                }
            ]
        }
    }

    pub fn match_transaction(&self, tx: &Transaction) -> Option<&Rule> {
        self.rules.iter().find(|&rule| rule.description == tx.description)
    }

    pub fn match_transactions(&self, txs: Vec<Transaction>) -> Vec<&Transaction> {
        let tx_rules =
            txs.into_iter()
                .map (|tx| {
                    let rule = self.rules.iter().find(|&rule| rule.description == tx.description);
                    (tx, rule)
                });

        let matched =
            tx_rules.filter_map(|(tx, rule)| 
                rule.map(|r| tx.categorize(&r.account))).collect();
        matched
    }
}

