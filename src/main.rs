extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn readcsv() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = readcsv() {
        println!("error running readcsv: {}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
