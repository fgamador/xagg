mod file_io;
mod transactions;

use crate::transactions::csv_record_to_transaction;
use chrono::NaiveDate;
use file_io::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn _print_all_transactions() {
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("*** {} ***", source);
        for csv_record in cvs_records {
            println!("{:?}", csv_record_to_transaction(&csv_record, &csv_config));
        }
        println!();
    }
}

fn _summarize_transactions() {
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("{}", source);
        let mut min_date = NaiveDate::from_ymd(3000, 1, 1);
        let mut max_date = NaiveDate::from_ymd(1000, 1, 1);
        for csv_record in cvs_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            min_date = min_date.min(transaction.date);
            max_date = max_date.max(transaction.date);
        }
        println!("   Date range: {} to {}", min_date, max_date);
    }
}

fn _list_descriptions() {
    let mut descriptions = BTreeMap::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            if transaction.date >= NaiveDate::from_ymd(2020, 3, 14) {
                descriptions.insert(transaction.raw_description, "");
            }
        }
    }
    for (raw_description, description) in &descriptions {
        println!("\"{}\" => \"{}\"", raw_description, description);
    }
}

fn main() {
    _list_descriptions();
}
