use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvConfig {
    date_index: u32,
    date_format: String,
    description_index: u32,
    amount_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_transaction_from_csv_record() {
        let _csv_config = CsvConfig {
            date_index: 1,
            date_format: "%m/%d/%Y".to_string(),
            description_index: 2,
            amount_index: 4,
        };
        let csv_record =
            StringRecord::from(vec!["foo", "2/12/2020", "A transaction", "bar", "-12.93"]);

        assert_eq!(csv_record.get(3), Some("bar"));
    }
}
