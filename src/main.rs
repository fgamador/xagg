use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process;
use std::fs::DirEntry;

#[derive(Debug, Deserialize)]
struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

fn _csv_example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("input/checking/transactions.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn _json_example() -> serde_json::Result<()> {
    let contents = fs::read_to_string("input/checking/config.json")
        .expect("Something went wrong reading the file");
    let config: CsvConfig = serde_json::from_str(&contents)?;
    println!("{:?}", config);
    Ok(())
}

fn dir_example() -> Result<(), Box<dyn Error>> {
    let it = fs::read_dir("input")?
        .map(|result| result.map(dir_entry_to_pair));

    for item in it {
        println!("{:?}", item?);
    }
    Ok(())
}

fn dir_entry_to_pair(entry: DirEntry) -> CsvConfig {
    dir_entry_to_config(&entry)
}

fn dir_entry_to_config(entry: &DirEntry) -> CsvConfig {
    let mut config_path = entry.path();
    config_path.push("config.json");
    let contents = fs::read_to_string(config_path)
        .expect("fs::read_to_string error");
    let config: CsvConfig = serde_json::from_str(&contents)
        .expect("serde_json::from_str error");
    config
}

fn main() {
    if let Err(err) = dir_example() {
        println!("error running dir_example: {}", err);
        process::exit(1);
    }
    // if let Err(err) = csv_example() {
    //     println!("error running csv_example: {}", err);
    //     process::exit(1);
    // }
    // if let Err(err) = json_example() {
    //     println!("error running json_example: {}", err);
    //     process::exit(1);
    // }
}
