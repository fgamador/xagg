mod read_files;
mod transactions;

use crate::transactions::csv_record_to_transaction;
use read_files::*;
use std::path::PathBuf;

fn print_all_transactions() {
    for (csv_config, cvs_records) in read_input(PathBuf::from("input")) {
        println!("*** {} ***", csv_config.source);
        for csv_record in cvs_records {
            println!("{:?}", csv_record_to_transaction(&csv_record, &csv_config));
        }
        println!();
    }
}

fn main() {
    print_all_transactions();
}
