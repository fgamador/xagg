use csv::StringRecord;
use serde::Deserialize;
use std::fs;
use std::io;
use std::process;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

fn dir_example() -> io::Result<()> {
    let it = fs::read_dir("input")?
        .map(|result| result.map(input_subdir_to_pair));

    for item in it {
        let pair = item?;
        println!("{:?}", pair.0);
        for path in pair.1 {
            println!("{:?}", path);
        }
    }
    Ok(())
}

fn input_subdir_to_pair(subdir: DirEntry) -> (CsvConfig, impl Iterator<Item=csv::Result<StringRecord>>) {
    (input_subdir_to_csv_config(&subdir),
     csv_file_path_iter_to_csv_record_iter(input_subdir_to_csv_file_path_iter(&subdir)))
}

fn input_subdir_to_csv_config(subdir: &DirEntry) -> CsvConfig {
    let mut config_path = subdir.path();
    config_path.push("config.json");
    let contents = fs::read_to_string(config_path).unwrap();
    serde_json::from_str(&contents).unwrap()
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

fn csv_file_path_iter_to_csv_record_iter<I>(paths: I) -> impl Iterator<Item=csv::Result<StringRecord>>
    where I: Iterator<Item=PathBuf>
{
    paths.map(|path| csv::Reader::from_path(path).unwrap().into_records()).flatten()
}

fn main() {
    if let Err(err) = dir_example() {
        println!("error running dir_example: {}", err);
        process::exit(1);
    }
}
