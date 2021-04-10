use csv::StringRecord;
use serde::Deserialize;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

pub fn input_dir_to_pair_iterator(
    dir: PathBuf,
) -> impl Iterator<Item = (CsvConfig, impl Iterator<Item = StringRecord>)> {
    unwrap_or_exit(fs::read_dir(&dir), &dir)
        .map(move |result| input_subdir_to_pair(unwrap_or_exit(result, &dir)))
}

fn input_subdir_to_pair(subdir: DirEntry) -> (CsvConfig, impl Iterator<Item = StringRecord>) {
    (
        input_subdir_to_csv_config(&subdir),
        csv_file_path_iter_to_csv_record_iter(input_subdir_to_csv_file_path_iter(&subdir)),
    )
}

fn input_subdir_to_csv_config(subdir: &DirEntry) -> CsvConfig {
    let mut config_path = subdir.path();
    config_path.push("config.json");
    let contents = unwrap_or_exit(fs::read_to_string(&config_path), &config_path);
    unwrap_or_exit(serde_json::from_str(&contents), &config_path)
}

fn input_subdir_to_csv_file_path_iter(subdir: &DirEntry) -> impl Iterator<Item = PathBuf> {
    let subdir_path = subdir.path();
    unwrap_or_exit(fs::read_dir(&subdir_path), &subdir_path).filter_map(move |result| {
        let entry = unwrap_or_exit(result, &subdir_path);
        let file_name = unwrap_or_exit_debug(entry.file_name().into_string(), &entry.path());
        if file_name.to_lowercase().ends_with(".csv") {
            Some(entry.path())
        } else {
            None
        }
    })
}

fn csv_file_path_iter_to_csv_record_iter<I>(paths: I) -> impl Iterator<Item = StringRecord>
where
    I: Iterator<Item = PathBuf>,
{
    paths
        .map(|path| {
            unwrap_or_exit(csv::Reader::from_path(&path), &path)
                .into_records()
                .map(move |result| unwrap_or_exit(result, &path))
        })
        .flatten()
}

fn unwrap_or_exit<T, E: Display>(result: Result<T, E>, path: &Path) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            println!("{}: {}", path.display(), error);
            exit(1)
        }
    }
}

fn unwrap_or_exit_debug<T, E: Debug>(result: Result<T, E>, path: &Path) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            println!("{}: {:?}", path.display(), error);
            exit(1)
        }
    }
}
