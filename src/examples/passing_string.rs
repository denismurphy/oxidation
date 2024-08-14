pub struct PassingString {
    pub value: String,
}

// adding functions to struct PassingString
impl PassingString {

    // '&'
    // &self reference self in function or return reference
    // return pointer in memory to String value (aka by reference)
    pub fn value(&self) -> &str {
        &self.value
    }

    // return value making clone/copy
    pub fn clone(&self) -> String {
        self.value.clone()
    }
}