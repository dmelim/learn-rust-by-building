use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
#[expect(
    dead_code,
    reason = "variants become reachable as the learner implements parsing"
)]
pub enum AppError {
    InvalidFieldCount { found: usize },
    InvalidId(String),
    EmptyCustomer,
    InvalidPriority(String),
    OrderNotFound(u32),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Write a useful message for every variant.
        write!(formatter, "application error")
    }
}

impl Error for AppError {}
