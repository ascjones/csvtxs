mod config;
mod rules;
mod transaction;

use color_eyre::eyre::{self, WrapErr};
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

use config::Config;
use rules::{MatchingRules, Rule};
use transaction::{read_txs, write_txs};

#[derive(Debug, StructOpt)]
struct Args {
    /// CSV file to import
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    /// Config file
    #[structopt(long, parse(from_os_str), default_value = "csvtxs.toml")]
    config: PathBuf,
    /// CSV import config
    #[structopt(long)]
    csv: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Args = Args::from_args();

    let config_file = fs::read(args.config).context("Error opening config file")?;
    let config: Config = toml::from_slice(&config_file)?;

    let csv_configs = config.csv.unwrap_or(HashMap::new());
    let csv_config = csv_configs
        .get(&args.csv)
        .ok_or_else(|| eyre::eyre!("No csv config '{}' found", args.csv))?;

    let rule_entries = config.rules.unwrap_or(Vec::new());
    let rules = MatchingRules::from_config(&rule_entries)?;

    let txs_csv = fs::File::open(args.file).context("Error opening CSV file")?;
    let txs = read_txs(&csv_config, txs_csv)?;

    let (matched, unmatched) = rules.match_transactions(txs);

    if unmatched.len() > 0 {
        println!(
            "{} transactions could not be categorised with existing rules",
            unmatched.len()
        );
        let _new_rules: Vec<Rule> = Vec::new();
        for tx in unmatched {
            // ask at command line for new account
            println!("");
            println!(
                "No matching rule for: {}, {}, {}",
                tx.date, tx.description, tx.amount
            );

            let mut account = String::new();
            println!("Enter account:");

            std::io::stdin()
                .read_line(&mut account)
                .expect("Failed to read line");

            println!("The accound you entered was: {}", account);

            // default rule to description
            // push new rule to vector
        }
    }

    // todo: iterate over unmatched and any unmatched add rule and rerun
    // todo: tab automcomplete accounts (from rules?)
    // todo: write new rules once finished (give summary and option to cancel)

    write_txs(&csv_config.account, matched)
    // if let Err(err) = readcsv(&date_fmt) {
    //     println!("error running readcsv: {}", err);
    //     process::exit(1);
    // }
}
