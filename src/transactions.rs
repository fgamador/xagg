use chrono::NaiveDate;
use csv::StringRecord;
use serde::Deserialize;
use std::collections::HashMap;
use std::str;
use trie_rs::Trie;
use trie_rs::TrieBuilder;

#[derive(Debug, Deserialize)]
pub struct CsvConfig {
    date_index: usize,
    date_format: String,
    description_index: usize,
    amount_index: usize,
    #[serde(default)]
    amount_is_debit: bool,
}

#[derive(Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub raw_description: String,
    pub amount: f32,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionClassificationRule {
    pub raw_prefix: String,
    pub description: String,
    pub category: String,
}

pub fn csv_record_to_transaction(csv_record: &StringRecord, csv_config: &CsvConfig) -> Transaction {
    // TODO exit on parse errors? need file path for error message
    Transaction {
        date: NaiveDate::parse_from_str(
            csv_record.get(csv_config.date_index).unwrap_or(""),
            &csv_config.date_format,
        )
            .unwrap_or_else(|_| NaiveDate::from_ymd(1, 1, 1)),
        raw_description: csv_record
            .get(csv_config.description_index)
            .unwrap_or("")
            .trim()
            .to_string(),
        amount: parse_csv_amount(csv_record, csv_config),
        description: "".to_string(),
        category: "".to_string(),
    }
}

fn parse_csv_amount(csv_record: &StringRecord, csv_config: &CsvConfig) -> f32 {
    let amount_str = csv_record
        .get(csv_config.amount_index)
        .unwrap()
        .replace(',', "");
    let amount_str = if amount_str.is_empty() { "0.0".to_string() } else { amount_str };
    let amount: f32 = amount_str
        .parse()
        .unwrap();
    if csv_config.amount_is_debit {
        -amount
    } else {
        amount
    }
}

pub struct TransactionClassifier {
    prefixes: Trie<u8>,
    rules_by_prefix: HashMap<String, TransactionClassificationRule>,
}

impl TransactionClassifier {
    pub fn new(rules: Vec<TransactionClassificationRule>) -> Self {
        let mut trie_builder = TrieBuilder::new();
        let mut rules_by_prefix = HashMap::with_capacity(rules.len());
        for rule in rules {
            trie_builder.push(rule.raw_prefix.clone());
            rules_by_prefix.insert(rule.raw_prefix.clone(), rule);
        }

        TransactionClassifier {
            prefixes: trie_builder.build(),
            rules_by_prefix,
        }
    }

    pub fn classify_transaction(&self, mut transaction: Transaction) -> Transaction {
        if let Some(prefix) =
            get_longest_common_prefix(&transaction.raw_description, &self.prefixes)
        {
            if let Some(rule) = self.rules_by_prefix.get(&prefix) {
                transaction.description = rule.description.clone();
                transaction.category = rule.category.clone();
            }
        } else if transaction.raw_description.is_empty() {
            transaction.description = "Unknown".to_string();
            transaction.category = "Unknown".to_string();
        } else {
            transaction.description = transaction.raw_description.clone();
            transaction.category = "Unknown".to_string();
        }
        transaction
    }
}

fn get_longest_common_prefix(string: &str, prefixes: &Trie<u8>) -> Option<String> {
    prefixes
        .common_prefix_search(string)
        .iter()
        .map(|utf8_prefix| str::from_utf8(utf8_prefix).unwrap())
        .max_by(|prefix1, prefix2| prefix1.len().cmp(&prefix2.len()))
        .map(|prefix| prefix.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction_from_csv_record() {
        let csv_config = CsvConfig {
            date_index: 1,
            date_format: "%m/%d/%Y".to_string(),
            description_index: 2,
            amount_index: 4,
            amount_is_debit: false,
        };
        let csv_record = StringRecord::from(vec![
            "ignore1",
            "2/12/2020",
            "ACME FALAFEL",
            "ignore2",
            "-12.93",
        ]);

        let transaction = csv_record_to_transaction(&csv_record, &csv_config);

        assert_eq!(transaction.date, NaiveDate::from_ymd(2020, 2, 12));
        assert_eq!(transaction.raw_description, "ACME FALAFEL");
        assert_eq!(transaction.amount, -12.93);
    }

    #[test]
    fn gets_longest_common_prefix() {
        let mut builder = TrieBuilder::new();
        builder.push("AB");
        builder.push("ABC");
        builder.push("ABD");
        builder.push("ABCE");
        let trie = builder.build();

        assert_eq!(
            get_longest_common_prefix("ABCD", &trie),
            Some("ABC".to_string())
        );
    }

    #[test]
    fn transaction_classification_sets_description_and_category() {
        let classifier = TransactionClassifier::new(vec![TransactionClassificationRule {
            raw_prefix: "DWB*".to_string(),
            description: "Doctors without Borders".to_string(),
            category: "Donation".to_string(),
        }]);

        let transaction = classifier.classify_transaction(Transaction {
            date: NaiveDate::from_ymd(1, 1, 1),
            raw_description: "DWB*DOCTORS W/O BORDER 212-679-6800 NY".to_string(),
            amount: 0.0,
            description: "".to_string(),
            category: "".to_string(),
        });

        assert_eq!(transaction.description, "Doctors without Borders");
        assert_eq!(transaction.category, "Donation");
    }

    #[test]
    fn transaction_classification_retains_unrecognized_raw_description() {
        let classifier = TransactionClassifier::new(vec![TransactionClassificationRule {
            raw_prefix: "DWB*".to_string(),
            description: "Doctors without Borders".to_string(),
            category: "Donation".to_string(),
        }]);

        let transaction = classifier.classify_transaction(Transaction {
            date: NaiveDate::from_ymd(1, 1, 1),
            raw_description: "ACME FALAFEL".to_string(),
            amount: 0.0,
            description: "".to_string(),
            category: "".to_string(),
        });

        assert_eq!(transaction.description, "ACME FALAFEL");
        assert_eq!(transaction.category, "Unknown");
    }

    #[test]
    fn transaction_classification_handles_empty_raw_description() {
        let classifier = TransactionClassifier::new(vec![TransactionClassificationRule {
            raw_prefix: "DWB*".to_string(),
            description: "Doctors without Borders".to_string(),
            category: "Donation".to_string(),
        }]);

        let transaction = classifier.classify_transaction(Transaction {
            date: NaiveDate::from_ymd(1, 1, 1),
            raw_description: "".to_string(),
            amount: 0.0,
            description: "".to_string(),
            category: "".to_string(),
        });

        assert_eq!(transaction.description, "Unknown");
        assert_eq!(transaction.category, "Unknown");
    }
}
