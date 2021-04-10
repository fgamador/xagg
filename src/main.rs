mod file_io;
mod transactions;

use crate::transactions::csv_record_to_transaction;
use chrono::NaiveDate;
use file_io::*;
use std::path::PathBuf;

fn _print_all_transactions() {
    for (csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("*** {} ***", csv_config.source);
        for csv_record in cvs_records {
            println!("{:?}", csv_record_to_transaction(&csv_record, &csv_config));
        }
        println!();
    }
}

fn summarize_transactions() {
    for (csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("{}", csv_config.source);
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

fn main() {
    summarize_transactions();
}
