use crate::{domain::Order, error::AppError};

pub fn parse_order(line: &str) -> Result<Order, AppError> {
    let fields: Vec<_> = line.split('|').map(str::trim).collect();
    if fields.len() != 3 {
        return Err(AppError::InvalidFieldCount {
            found: fields.len(),
        });
    }

    let id = fields[0]
        .parse()
        .map_err(|_| AppError::InvalidId(fields[0].to_owned()))?;
    if fields[1].is_empty() {
        return Err(AppError::EmptyCustomer);
    }
    let priority = match fields[2].to_ascii_lowercase().as_str() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => return Err(AppError::InvalidPriority(fields[2].to_owned())),
    };

    Ok(Order {
        id,
        customer: fields[1].to_owned(),
        priority,
    })
}

pub fn dispatch_by_id(queue: &mut Vec<Order>, id: u32) -> Result<Order, AppError> {
    let index = queue
        .iter()
        .position(|order| order.id == id)
        .ok_or(AppError::OrderNotFound(id))?;
    Ok(queue.remove(index))
}
