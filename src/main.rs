use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io;
use std::process;
use std::fs::DirEntry;
use csv::{StringRecord, StringRecordIter};
use std::path::PathBuf;

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

fn dir_example() -> io::Result<()> {
    let it = fs::read_dir("input")?
        .map(|result| result.map(input_subdir_to_pair));

    for item in it {
        println!("{:?}", item?.0);
    }
    Ok(())
}

fn input_subdir_to_pair(subdir: DirEntry) -> (CsvConfig, impl Iterator<Item=PathBuf>) {
    (input_subdir_to_csv_config(&subdir), input_subdir_to_csv_file_path_iter(&subdir))
}

fn input_subdir_to_csv_config(subdir: &DirEntry) -> CsvConfig {
    let mut config_path = subdir.path();
    config_path.push("config.json");
    let contents = fs::read_to_string(config_path)
        .expect("fs::read_to_string error");
    let config: CsvConfig = serde_json::from_str(&contents)
        .expect("serde_json::from_str error");
    config
}

fn input_subdir_to_csv_file_path_iter(subdir: &DirEntry) -> impl Iterator<Item=PathBuf> {
    fs::read_dir(subdir.path()).unwrap()
        .filter_map(|result| {
            let entry = result.unwrap();
            if entry.file_name().into_string().unwrap().to_lowercase().ends_with(".csv") {
                Some(entry.path())
            } else {
                None
            }
        })
}

// fn input_dir_entry_to_csv_line_iter(entry: &DirEntry) -> std::slice::Iter<'_, StringRecord> {
//     let mut rdr = csv::Reader::from_path("input/checking/transactions.csv")?;
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     }
// }

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
