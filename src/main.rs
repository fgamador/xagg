mod file_io;
mod transactions;
mod transaction_classification;
mod icicle_chart_data;

fn main() {
    icicle_chart_data::generate_icicle_chart_data();
    // print_draft_rules_for_unrecognized_descriptions();
    // print_categories();
    // sum_categories();
    // list_unrecognized_descriptions();
    // sum_unrecognized_descriptions();
}
