use oxidation::{advanced_examples, basic_examples, concurrency, error_handling};

mod types;
mod utils;
mod examples;
mod models;
mod lang;

fn main() {
    basic_examples::run();
    advanced_examples::run();
    error_handling::run();
    concurrency::run();
}