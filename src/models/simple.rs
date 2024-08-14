use std::fmt;
use crate::utils::aliases::*;

pub struct Simple {
    pub id: Int,
    pub name: String,
}

// Simple struct implements Display trait this is used when passing struct to println macro to format output string
impl fmt::Display for Simple {
    fn fmt(&self, mutable_formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(mutable_formatter, "Simple {{ id: {}, name: {}}}", self.id, self.name)
    }
}