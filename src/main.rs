use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("input/checking/transactions.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
