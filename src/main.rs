mod read_files;

use read_files::*;
use std::path::PathBuf;

fn input_iter_example() {
    for pair in input_dir_to_pair_iterator(PathBuf::from("input")) {
        println!("{:?}", pair.0);
        for path in pair.1 {
            println!("{:?}", path);
        }
    }
}

fn main() {
    input_iter_example();
}
