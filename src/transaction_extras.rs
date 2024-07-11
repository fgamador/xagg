#![allow(dead_code)]

use std::collections::HashSet;
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
    println!("Date,Description,Category,Expense,Income,Memo,Source");
    for (source, csv_config, csv_records) in read_input(PathBuf::from("input")) {
        for csv_record in csv_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            // if is_transfer(&*transaction.raw_description) { continue; }

            let expense = if transaction.amount <= 0.0 { -transaction.amount } else { 0.0 };
            let income = if transaction.amount > 0.0 { transaction.amount } else { 0.0 };
            println!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"\",\"{}\"",
                     transaction.date.format("%m/%d/%Y"),
                     transaction.raw_description, transaction.raw_category, expense, income, source);
        }
    }
}

fn _is_transfer(description: &str) -> bool
{
    let transfer_descriptions = HashSet::from(
        ["BA ELECTRONIC PAYMENT", "BANK OF AMERICA", "CAPITAL ONE", "CITI AUTOPAY",
            "CITI CREDIT CARD", "CREDIT BALANCE REFUND DEBIT", "CREDIT CARD PAYMENT",
            "DEPOSIT FROM SHARE 01", "DIGITAL DEPOSIT FROM SHARE 01",
            "DIGITAL DEPOSIT FROM SHARE 09", "DIGITAL WITHDRAWAL TO SHARE 01",
            "DIGITAL WITHDRAWAL TO SHARE 09", "PADDLE", "PAYMENT THANK YOU", "PAYPAL",
            "PAYPAL TRANSFER", "WITHDRAWAL TO SHARE 09"]);
    transfer_descriptions.contains(description)
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
