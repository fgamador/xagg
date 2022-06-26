mod file_io;
mod transactions;

use crate::transactions::{csv_record_to_transaction, Transaction, TransactionClassifier};
use chrono::NaiveDate;
use file_io::*;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};
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

    let mut grand_total: f32 = 0.0;
    let mut category_sums: HashMap<String, f32> = HashMap::new();
    for (_source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = classifier
                .classify_transaction(csv_record_to_transaction(&csv_record, &csv_config));
            if !should_exclude_transaction(&transaction) {
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

#[derive(Debug, Serialize)]
pub struct TransactionDataNode {
    name: String,
    value: u32,
}

#[derive(Debug, Serialize)]
pub struct DescriptionDataNode {
    name: String,
    children: Vec<TransactionDataNode>,
}

#[derive(Debug, Serialize)]
pub struct CategoryDataNode {
    name: String,
    children: Vec<DescriptionDataNode>,
}

#[derive(Debug, Serialize)]
pub struct RootDataNode {
    name: String,
    children: Vec<CategoryDataNode>,
}

type TransactionDataNodes = Vec<TransactionDataNode>;
type DescriptionMap = HashMap<String, TransactionDataNodes>;
type CategoryMap = HashMap<String, DescriptionMap>;

fn generate_icicle_chart_data() {
    let category_map = gather_category_map();
    let category_data_nodes = categories_to_data_nodes(category_map);
    let root_data_node = RootDataNode {
        name: "Spending".to_string(),
        children: category_data_nodes,
    };
    println!("{}", serde_json::to_string(&root_data_node).unwrap());
}

fn gather_category_map() -> CategoryMap {
    let rules = read_rules(PathBuf::from("input"));
    let classifier = TransactionClassifier::new(rules);

    let mut categories: CategoryMap = HashMap::new();

    for (source, csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        for csv_record in cvs_records {
            let transaction = classifier
                .classify_transaction(csv_record_to_transaction(&csv_record, &csv_config));
            if !should_exclude_transaction(&transaction) {
                let descriptions = categories
                    .entry(transaction.category.clone())
                    .or_insert(HashMap::new());
                let transactions = descriptions
                    .entry(transaction.description.clone())
                    .or_insert(vec![]);
                if transaction.amount <= 0.0 {
                    transactions.push(transaction_to_data_node(&source, transaction));
                }
            }
        }
    }

    categories
}

fn transaction_to_data_node(source: &str, transaction: Transaction) -> TransactionDataNode {
    TransactionDataNode {
        name: format!("{}, {},", transaction.date, source),
        value: -transaction.amount.round() as u32,
    }
}

fn categories_to_data_nodes(category_map: CategoryMap) -> Vec<CategoryDataNode> {
    let mut category_data_nodes = vec![];
    for (category, description_map) in category_map {
        let mut category_data_node = CategoryDataNode {
            name: category,
            children: vec![],
        };
        for (description, transaction_data_nodes) in description_map {
            category_data_node.children.push(DescriptionDataNode {
                name: description,
                children: transaction_data_nodes,
            });
        }
        category_data_nodes.push(category_data_node);
    }
    category_data_nodes
}

fn should_exclude_transaction(transaction: &Transaction) -> bool {
    let exclude_categories: HashSet<&'static str> =
        ["Dividend", "Salary", "Stock", "Transfer", "Zach"]
            .iter()
            .cloned()
            .collect();
    let exclude_positive_categories: HashSet<&'static str> =
        ["Travel", "Unknown"].iter().cloned().collect();

    if transaction.date < NaiveDate::from_ymd(2020, 3, 14) {
        return true;
    }
    if transaction.date >= NaiveDate::from_ymd(2021, 3, 14) {
        return true;
    }
    if exclude_categories.contains(&*transaction.category) {
        return true;
    }
    if transaction.amount > 0.0 && exclude_positive_categories.contains(&*transaction.category) {
        return true;
    }

    false
}

fn main() {
    generate_icicle_chart_data();
}
