use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidFieldCount { found: usize },
    InvalidId(String),
    EmptyCustomer,
    InvalidPriority(String),
    OrderNotFound(u32),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFieldCount { found } => {
                write!(formatter, "expected 3 fields but found {found}")
            }
            Self::InvalidId(value) => write!(formatter, "invalid order id: {value}"),
            Self::EmptyCustomer => write!(formatter, "customer name cannot be empty"),
            Self::InvalidPriority(value) => write!(formatter, "invalid priority: {value}"),
            Self::OrderNotFound(id) => write!(formatter, "order #{id} was not found"),
        }
    }
}

impl Error for AppError {}
