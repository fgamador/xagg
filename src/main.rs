use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug, Deserialize)]
struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

fn csv_example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("input/checking/transactions.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn json_example() -> serde_json::Result<()> {
    let contents = fs::read_to_string("input/checking/config.json")
        .expect("Something went wrong reading the file");
    let config: CsvConfig = serde_json::from_str(&contents)?;
    println!("{:?}", config);
    Ok(())
}

fn main() {
    if let Err(err) = csv_example() {
        println!("error running csv_example: {}", err);
        process::exit(1);
    }
    if let Err(err) = json_example() {
        println!("error running json_example: {}", err);
        process::exit(1);
    }
}
