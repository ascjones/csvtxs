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

    pub fn match_transactions(&self, txs: Vec<Transaction>) -> Vec<Transaction> {
        txs.into_iter()
            .filter_map (|tx| {
                self.rules.iter()
                    .find(|rule| rule.description == tx.description)
                    // todo: how to use record update syntax here
                    .map(|r| Transaction { account: Some(r.account.to_string()), date: tx.date, amount: tx.amount, description: tx.description.to_string() })
            }).collect()
    }
}

