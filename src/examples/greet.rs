use crate::utils::aliases::*;

// Prints Number
// '!' at end of function name println! means it a macro in rust
pub fn greet_number(x: Long) {
    println!("Hello to number {}", x);
}

// String
pub fn greet_string(x: String) {
    println!("Hello to string {}", x);
}

// String Slice
pub fn greet_str(x: &str) {
    println!("Hello to string {}", x);
}

pub fn greet_format(x: &str) -> String {
    return format!("Hello to string {}", x);
}