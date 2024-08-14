#[derive(Debug)]
pub enum Data {
    Integer(i32),
    String(String),
}

impl Data {
    pub(crate) fn print(&self) {
        match self {
            Data::Integer(value) => println!("Integer: {}", value),
            Data::String(value) => println!("String: {}", value),
        }
    }
}