mod file_io;
mod transactions;

use crate::transactions::{csv_record_to_transaction, TransactionClassifier};
use chrono::NaiveDate;
use file_io::*;
use std::collections::{BTreeMap, HashMap};
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
    let rules = read_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut descriptions = BTreeMap::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            let transaction = classifier.classify_transaction(transaction);
            if transaction.date >= NaiveDate::from_ymd(2020, 3, 14) {
                descriptions.insert(transaction.raw_description, transaction.description);
            }
        }
    }

    for (raw_description, description) in &descriptions {
        println!("\"{}\" => \"{}\"", raw_description, description);
    }
}

fn _print_draft_rules() {
    let mut descriptions = BTreeMap::new();
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        if source == "PayPal" {
            for csv_record in cvs_records {
                let transaction = csv_record_to_transaction(&csv_record, &csv_config);
                if transaction.date >= NaiveDate::from_ymd(2020, 3, 14) {
                    descriptions.insert(transaction.raw_description, transaction.description);
                }
            }
        }
    }

    println!("[");
    for (raw_description, _description) in &descriptions {
        println!(
            r#"  {{
    "raw_prefix": "{}",
    "description": "{}",
    "category": "TODO"
  }},"#,
            raw_description, raw_description
        );
    }
    println!("]");
}

fn _align_checking_and_paypal() {
    let mut tuples = vec![];
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        if source == "PayPal" || source == "WSECU Checking" {
            for csv_record in cvs_records {
                let transaction = csv_record_to_transaction(&csv_record, &csv_config);
                if transaction.date >= NaiveDate::from_ymd(2020, 3, 14)
                    && transaction.date < NaiveDate::from_ymd(2021, 3, 14)
                {
                    tuples.push((
                        transaction.date,
                        transaction.raw_description,
                        transaction.amount,
                    ));
                }
            }
        }
    }

    tuples.sort_by(|(_date1, desc1, _amt1), (_date2, desc2, _amt2)| {
        desc1.to_lowercase().cmp(&desc2.to_lowercase())
    });

    for (date, description, amount) in tuples {
        println!("{}, \"{}\", {}", date, description, amount);
    }
}

fn _sum_categories() {
    let rules = read_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut category_sums: HashMap<String, f32> = HashMap::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = classifier
                .classify_transaction(csv_record_to_transaction(&csv_record, &csv_config));
            if transaction.date >= NaiveDate::from_ymd(2020, 3, 14)
                && transaction.date < NaiveDate::from_ymd(2021, 3, 14)
            {
                *category_sums.entry(transaction.category).or_insert(0.0) += transaction.amount;
            }
        }
    }

    let mut category_sums: Vec<(String, f32)> = category_sums
        .iter()
        .map(|(cat, amt)| (cat.clone(), *amt))
        .collect();
    category_sums.sort_by(|(_cat1, amt1), (_cat2, amt2)| amt1.partial_cmp(amt2).unwrap());

    for (category, sum) in &category_sums {
        println!("{}: {:.2}", category, sum);
    }
}

fn main() {
    _sum_categories();
}
