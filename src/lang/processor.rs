use super::data::Data;

// Trait for custom data processing
pub trait Processor {
    fn process(&self, data: Data);
}

pub struct IntegerProcessor;

impl Processor for IntegerProcessor {
    fn process(&self, data: Data) {
        if let Data::Integer(value) = data {
            println!("Processed integer: {}", value * 2);
        } else {
            println!("Unsupported data type for integer processing");
        }
    }
}

pub struct StringProcessor;

impl Processor for StringProcessor {
    fn process(&self, data: Data) {
        if let Data::String(value) = data {
            println!("Processed string: {}", value.to_uppercase());
        } else {
            println!("Unsupported data type for string processing");
        }
    }
}