pub struct StaticUtils {}

impl StaticUtils {
    pub fn concatenate(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }
}