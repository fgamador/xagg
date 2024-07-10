#![allow(dead_code)]

use std::path::PathBuf;
use chrono::NaiveDate;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::file_io::{read_classification_rules, read_input};
use crate::icicle_chart_data;
use crate::transaction_classification::TransactionClassifier;
use crate::transactions::csv_record_to_transaction;

pub fn print_draft_rules_for_unrecognized_descriptions() {
    let (_grand_total, description_sums) = get_unrecognized_description_sums();

    println!("[");
    for ((source, raw_description), sum) in &description_sums {
        if (*sum).abs() < 90.0 { continue; }

        println!(
            r#"  {{
    "sum": "{}",
    "source": "{}",
    "raw_prefix": "{}",
    "description": "{}",
    "category": "Unknown"
  }},"#,
            sum, source, raw_description, titlecase::titlecase(strip_unwanted_prefix(raw_description))
        );
    }
    println!("]");
}

fn strip_unwanted_prefix(raw_description: &String) -> &str {
    let unwanted_prefixes = vec!["SP ", "SQ *", "TST* "];
    for prefix in unwanted_prefixes
    {
        if let Some(stripped) = raw_description.strip_prefix(prefix)
        {
            return stripped;
        }
    }
    raw_description
}

pub fn sum_unrecognized_descriptions() {
    let (grand_total, description_sums) = get_unrecognized_description_sums();

    println!("Grand total: {:.2}", grand_total);
    for ((source, description), sum) in &description_sums {
        println!("  {} / {}: {:.2}", source, description, sum);
    }
}

fn get_unrecognized_description_sums() -> (f32, Vec<((String, String), f32)>) {
    let rules = read_classification_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut grand_total: f32 = 0.0;
    let mut description_sums: HashMap<(String, String), f32> = HashMap::new();
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            let transaction = classifier.classify_transaction(transaction);
            if !icicle_chart_data::should_exclude_transaction(&transaction) && transaction.category == "Unknown" {
                grand_total += transaction.amount;
                *description_sums.entry((source.clone(), transaction.raw_description)).or_insert(0.0) += transaction.amount;
            }
        }
    }

    let mut description_sums: Vec<((String, String), f32)> = description_sums
        .iter()
        .map(|(desc, amt)| (desc.clone(), *amt))
        .collect();
    description_sums.sort_by(|(_desc1, amt1), (_desc2, amt2)| amt1.partial_cmp(amt2).unwrap());
    (grand_total, description_sums)
}

pub fn list_unrecognized_descriptions() {
    let raw_descriptions = get_unrecognized_descriptions();

    for (source, raw_description) in &raw_descriptions {
        println!("{}: {}", source, raw_description);
    }
}

fn get_unrecognized_descriptions() -> BTreeSet<(String, String)> {
    let rules = read_classification_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut raw_descriptions = BTreeSet::new();
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = csv_record_to_transaction(&csv_record, &csv_config);
            let transaction = classifier.classify_transaction(transaction);
            if !icicle_chart_data::should_exclude_transaction(&transaction) && transaction.category == "Unknown" {
                raw_descriptions.insert((source.clone(), transaction.raw_description));
            }
        }
    }
    raw_descriptions
}

pub fn print_all_transactions() {
    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("*** {} ***", source);
        for csv_record in cvs_records {
            println!("{:?}", csv_record_to_transaction(&csv_record, &csv_config));
        }
        println!();
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

pub fn list_descriptions() {
    let rules = read_classification_rules(PathBuf::from("input"));
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

pub fn print_draft_rules() {
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

pub fn align_checking_and_paypal() {
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

pub fn print_categories() {
    let rules = read_classification_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut categories = BTreeSet::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = classifier
                .classify_transaction(csv_record_to_transaction(&csv_record, &csv_config));
            categories.insert(transaction.category);
        }
    }

    for category in categories {
        println!("{}", category);
    }
}

pub fn sum_categories() {
    let rules = read_classification_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut grand_total: f32 = 0.0;
    let mut category_sums: HashMap<String, f32> = HashMap::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = classifier
                .classify_transaction(csv_record_to_transaction(&csv_record, &csv_config));
            if !icicle_chart_data::should_exclude_transaction(&transaction) {
                grand_total += transaction.amount;
                *category_sums.entry(transaction.category).or_insert(0.0) += transaction.amount;
            }
        }
    }

    let mut category_sums: Vec<(String, f32)> = category_sums
        .iter()
        .map(|(cat, amt)| (cat.clone(), *amt))
        .collect();
    category_sums.sort_by(|(_cat1, amt1), (_cat2, amt2)| amt1.partial_cmp(amt2).unwrap());

    println!("Grand total: {:.2}", grand_total);
    for (category, sum) in &category_sums {
        println!("  {}: {:.2}", category, sum);
    }
}
