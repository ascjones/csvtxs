use chrono::NaiveDate;
use currency::Currency;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub amount: Currency,
    pub description: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CategorisedTransaction {
    pub transaction: Transaction,
    pub account: String,
}

impl Transaction {
    pub fn new(date: NaiveDate, amount: Currency, description: &str) -> Self {
        Transaction {
            date,
            amount,
            description: description.to_owned(),
        }
    }
}

pub fn read_txs<R>(date_fmt: &str, reader: R) -> color_eyre::Result<Vec<Transaction>>
where
    R: Read,
{
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.records()
        .map(|result| {
            let record = result?;
            let date = NaiveDate::parse_from_str(&record[0], date_fmt);
            let amount = Currency::from_str(record[2].trim()).unwrap();
            let description = record[3].to_string();
            Ok(Transaction::new(date?, amount, &description))
        })
        .collect()
}

pub fn write_txs(account: &str, txs: Vec<CategorisedTransaction>) -> color_eyre::Result<()> {
    for tx in txs {
        println!("{} * {}", tx.transaction.date, tx.transaction.description);
        println!("    {} £{:.2}", tx.account, tx.transaction.amount);
        println!("    {} £-{:.2}", account, tx.transaction.amount);
        println!("");
    }
    Ok(())
}
