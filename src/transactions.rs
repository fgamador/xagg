use std::str;

use chrono::NaiveDate;
use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvConfig {
    #[serde(default)]
    pub source_alias: String,
    date_index: usize,
    date_format: String,
    description_index: usize,
    #[serde(default = "usize::max_value")]
    category_index: usize,
    #[serde(default = "usize::max_value")]
    amount_index: usize,
    #[serde(default = "usize::max_value")]
    debit_index: usize,
    #[serde(default = "usize::max_value")]
    credit_index: usize,
}

#[derive(Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub raw_description: String,
    pub raw_category: String,
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
        raw_description: get_string_field_value(csv_record, csv_config.description_index),
        raw_category: get_string_field_value(csv_record, csv_config.category_index),
        amount: get_amount(csv_record, csv_config),
        description: "".to_string(),
        category: "".to_string(),
    }
}

fn get_string_field_value(csv_record: &StringRecord, field_index: usize) -> String {
    if field_index == usize::MAX { return "".to_string(); }

    csv_record
        .get(field_index)
        .unwrap()
        .trim()
        .to_string()
}

fn get_amount(csv_record: &StringRecord, csv_config: &CsvConfig) -> f32 {
    if csv_config.amount_index != usize::MAX {
        return get_f32_field_value(csv_record, csv_config.amount_index);
    }

    assert_ne!(csv_config.debit_index, usize::MAX);
    assert_ne!(csv_config.credit_index, usize::MAX);

    let debit = get_f32_field_value(csv_record, csv_config.debit_index);
    if debit > 0.0 {
        return -debit;
    }

    get_f32_field_value(csv_record, csv_config.credit_index)
}

fn get_f32_field_value(csv_record: &StringRecord, field_index: usize) -> f32 {
    let value_str = csv_record
        .get(field_index)
        .unwrap()
        .replace(',', "");
    if value_str.is_empty() { return 0.0; }

    value_str
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction_from_csv_record() {
        let csv_config = CsvConfig {
            source_alias: "".to_string(),
            date_index: 1,
            date_format: "%m/%d/%Y".to_string(),
            description_index: 2,
            category_index: 4,
            amount_index: 5,
            debit_index: usize::MAX,
            credit_index: usize::MAX,
        };
        let csv_record = StringRecord::from(vec![
            "ignore1",
            "2/12/2020",
            "ACME FALAFEL",
            "ignore2",
            "ignore3",
            "-12.93",
        ]);

        let transaction = csv_record_to_transaction(&csv_record, &csv_config);

        assert_eq!(transaction.date, NaiveDate::from_ymd(2020, 2, 12));
        assert_eq!(transaction.raw_description, "ACME FALAFEL");
        assert_eq!(transaction.amount, -12.93);
    }
}
