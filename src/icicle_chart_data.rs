use std::collections::{HashMap, HashSet};
use chrono::NaiveDate;
use std::path::PathBuf;
use serde::Serialize;
use crate::file_io::{read_classification_rules, read_input};
use crate::transaction_classification::TransactionClassifier;
use crate::transactions::{csv_record_to_transaction, Transaction};

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

pub fn generate_icicle_chart_data() {
    let category_map = gather_category_map();
    let category_data_nodes = categories_to_data_nodes(category_map);
    let root_data_node = RootDataNode {
        name: "Spending".to_string(),
        children: category_data_nodes,
    };
    println!("{}", serde_json::to_string(&root_data_node).unwrap());
}

fn gather_category_map() -> CategoryMap {
    let rules = read_classification_rules(PathBuf::from("input"));
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

pub fn should_exclude_transaction(transaction: &Transaction) -> bool {
    let exclude_categories: HashSet<&'static str> =
        ["Dividend", "Investment", "Reimbursed", "Salary", "Tax", "Transfer"]
            .iter()
            .cloned()
            .collect();
    let exclude_positive_categories: HashSet<&'static str> =
        ["Travel", "Unknown"].iter().cloned().collect();

    if transaction.date < NaiveDate::from_ymd(2023, 1, 1) {
        return true;
    }
    if transaction.date >= NaiveDate::from_ymd(2024, 1, 1) {
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
