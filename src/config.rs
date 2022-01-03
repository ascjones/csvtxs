use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub csv: Option<HashMap<String, CsvImports>>,
    pub rules: Option<Vec<RuleEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct CsvImports {
    pub account: String,
    pub date_format: String,
    pub columns: CsvColumns,
}

#[derive(Debug, Deserialize)]
pub struct CsvColumns {
    pub date: usize,
    pub amount: usize,
    pub description: usize,
}

#[derive(Debug, Deserialize)]
pub struct RuleEntry {
    pub description: String,
    pub account: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_config() {
        let toml_str = r#"
            [csv.current_account]
            account = "Assets::Current"
            date_format = "%d/%m/%Y"
            columns = { date = 0, amount = 1, description = 2 }

            [csv.amex]
            account = "Liabilities::Amex"
            date_format = "%d/%m/%Y"
            columns = { date = 0, amount = 2, description = 3 }

            [[rules]]
            description = "AMAZON"
            account = "Expenses::Amazon"

            [[rules]]
            description = "Sainsbury's"
            account = "Expenses::Groceries"
        "#;

        let decoded: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(decoded.csv.unwrap().len(), 2);
        assert_eq!(decoded.rules.unwrap().len(), 2);
    }
}
