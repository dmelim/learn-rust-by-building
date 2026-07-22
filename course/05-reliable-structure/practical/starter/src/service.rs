use crate::{domain::Order, error::AppError};

pub fn parse_order(line: &str) -> Result<Order, AppError> {
    // TODO: Parse id|customer|priority and return specific failures.
    let _ = line;
    Err(AppError::InvalidFieldCount { found: 0 })
}

pub fn dispatch_by_id(queue: &mut Vec<Order>, id: u32) -> Result<Order, AppError> {
    // TODO: Remove the matching order or return OrderNotFound.
    let _ = queue;
    Err(AppError::OrderNotFound(id))
}
