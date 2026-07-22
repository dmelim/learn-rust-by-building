#[derive(Debug, PartialEq)]
pub struct Order {
    pub id: u32,
    pub customer: String,
    pub priority: bool,
}
