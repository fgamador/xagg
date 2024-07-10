#![allow(dead_code)]

use std::path::PathBuf;

use chrono::NaiveDate;

use crate::file_io::read_input;
use crate::transactions::csv_record_to_transaction;

pub fn print_all_transactions() {
    for (source, csv_config, csv_records) in read_input(PathBuf::from("input")) {
        println!("*** {} ***", source);
        for csv_record in csv_records {
            println!("{:?}", csv_record_to_transaction(&csv_record, &csv_config));
        }
        println!();
    }
}

pub fn print_all_transactions_as_csv() {
    println!("Date,Description,Expense,Income,Category,Source");
    for (source, csv_config, csv_records) in read_input(PathBuf::from("input")) {
        for csv_record in csv_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            let expense = if transaction.amount <= 0.0 { -transaction.amount } else { 0.0 };
            let income = if transaction.amount > 0.0 { transaction.amount } else { 0.0 };
            println!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                     transaction.date.format("%d/%m/%Y"),
                     transaction.raw_description, expense, income, transaction.raw_category, source);
        }
    }
}

pub fn summarize_transactions() {
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
