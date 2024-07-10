use std::str;

use chrono::NaiveDate;
use csv::StringRecord;
use serde::Deserialize;

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

pub fn csv_record_to_transaction(csv_record: &StringRecord, csv_config: &CsvConfig) -> Transaction {
    Transaction {
        date: NaiveDate::parse_from_str(
            csv_record.get(csv_config.date_index).unwrap(),
            &csv_config.date_format,
        )
            .unwrap(),
        raw_description: csv_record
            .get(csv_config.description_index)
            .unwrap()
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
}
