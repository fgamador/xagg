use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Deserialize)]
struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

fn input_iter_example() {
    for pair in input_dir_to_pair_iterator(&Path::new("input")) {
        println!("{:?}", pair.0);
        for path in pair.1 {
            println!("{:?}", path);
        }
    }
}

fn input_dir_to_pair_iterator(
    dir: &Path,
) -> impl Iterator<Item = (CsvConfig, impl Iterator<Item = csv::Result<StringRecord>>)> {
    fs::read_dir(dir)
        .unwrap()
        .map(|result| input_subdir_to_pair(result.unwrap()))
}

fn input_subdir_to_pair(
    subdir: DirEntry,
) -> (CsvConfig, impl Iterator<Item = csv::Result<StringRecord>>) {
    (
        input_subdir_to_csv_config(&subdir),
        csv_file_path_iter_to_csv_record_iter(input_subdir_to_csv_file_path_iter(&subdir)),
    )
}

fn input_subdir_to_csv_config(subdir: &DirEntry) -> CsvConfig {
    let mut config_path = subdir.path();
    config_path.push("config.json");
    let contents = fs::read_to_string(config_path).unwrap();
    serde_json::from_str(&contents).unwrap()
}

fn input_subdir_to_csv_file_path_iter(subdir: &DirEntry) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(subdir.path()).unwrap().filter_map(|result| {
        let entry = result.unwrap();
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .to_lowercase()
            .ends_with(".csv")
        {
            Some(entry.path())
        } else {
            None
        }
    })
}

fn csv_file_path_iter_to_csv_record_iter<I>(
    paths: I,
) -> impl Iterator<Item = csv::Result<StringRecord>>
where
    I: Iterator<Item = PathBuf>,
{
    paths
        .map(|path| unwrap_or_exit(csv::Reader::from_path(&path), &path).into_records())
        .flatten()
}

fn unwrap_or_exit<T, E: Error>(result: Result<T, E>, path: &Path) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            println!("{}: {}", path.display(), error);
            exit(1)
        }
    }
}

fn main() {
    input_iter_example();
}
