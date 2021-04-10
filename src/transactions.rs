use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvConfig {
    date_index: usize,
    date_format: String,
    description_index: usize,
    amount_index: usize,
}

pub struct Transaction {
    raw_description: String,
    amount: f32,
}

pub fn csv_record_to_transaction(csv_record: &StringRecord, csv_config: &CsvConfig) -> Transaction {
    Transaction {
        raw_description: csv_record
            .get(csv_config.description_index)
            .unwrap_or("")
            .to_string(),
        amount: csv_record
            .get(csv_config.amount_index)
            .unwrap_or("0.0")
            .parse()
            .unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_transaction_from_csv_record() {
        let csv_config = CsvConfig {
            date_index: 1,
            date_format: "%m/%d/%Y".to_string(),
            description_index: 2,
            amount_index: 4,
        };
        let csv_record = StringRecord::from(vec![
            "ignore1",
            "2/12/2020",
            "ACME FALAFEL",
            "ignore2",
            "-12.93",
        ]);

        let transaction = csv_record_to_transaction(&csv_record, &csv_config);

        assert_eq!(transaction.raw_description, "ACME FALAFEL");
        assert_eq!(transaction.amount, -12.93);
    }
}
