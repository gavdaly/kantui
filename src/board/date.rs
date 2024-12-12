pub struct Date(String);

impl Date {
    pub fn new(date: &str) -> Self {
        Date(date.to_string())
    }
}
