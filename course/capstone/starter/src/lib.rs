mod model;
mod store;

use std::{collections::HashMap, path::Path};

pub use model::{DomainError, Order, Status};
pub use store::StoreError;

#[derive(Debug, PartialEq)]
pub enum Command {
    New,
    Add,
    List,
    Find(String),
    Dispatch { id: u32, courier: String },
    Deliver(u32),
    Summary,
    Save,
    Help,
    Quit,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    // TODO: Parse all commands and their typed arguments.
    match input.trim() {
        "new" | "add" => Ok(Command::New),
        "help" => Ok(Command::Help),
        "quit" => Ok(Command::Quit),
        _ => Err("command parsing is not implemented".to_owned()),
    }
}

#[derive(Debug, Default)]
pub struct DispatchDesk {
    orders: Vec<Order>,
    next_id: u32,
}

impl DispatchDesk {
    pub fn new() -> Self {
        Self {
            orders: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load(path: &Path) -> Result<Self, StoreError> {
        let orders = store::load(path)?;
        let next_id = orders
            .iter()
            .map(Order::id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        Ok(Self { orders, next_id })
    }

    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        store::save(path, &self.orders)
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn add(
        &mut self,
        customer: String,
        address: String,
        distance_km: f64,
        speed_kmh: f64,
        preparation_minutes: u32,
        priority: bool,
    ) -> Result<u32, DomainError> {
        // TODO: Validate, append, and advance the stable id.
        let _ = (
            self.next_id,
            customer,
            address,
            distance_km,
            speed_kmh,
            preparation_minutes,
            priority,
        );
        Err(DomainError::InvalidInput(
            "add is not implemented".to_owned(),
        ))
    }

    pub fn dispatch(&mut self, id: u32, courier: String) -> Result<(), DomainError> {
        // TODO: Find the order and apply its transition.
        let _ = courier;
        Err(DomainError::OrderNotFound(id))
    }

    pub fn deliver(&mut self, id: u32) -> Result<(), DomainError> {
        // TODO: Find the order and apply its transition.
        Err(DomainError::OrderNotFound(id))
    }

    pub fn search(&self, query: &str) -> Vec<&Order> {
        // TODO: Search customer and address without consuming orders.
        let _ = query;
        self.orders.iter().take(0).collect()
    }

    pub fn summary(&self) -> HashMap<&'static str, usize> {
        // TODO: Count all status labels.
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order() -> Order {
        Order::new(
            1,
            "Amina".to_owned(),
            "404 Test Route".to_owned(),
            12.0,
            30.0,
            10,
            false,
        )
        .unwrap()
    }

    #[test]
    #[ignore = "implement estimates"]
    fn calculates_estimate() {
        assert_eq!(order().estimate_minutes(), 34);
    }

    #[test]
    #[ignore = "implement transitions"]
    fn follows_valid_status_transitions() {
        let mut order = order();
        order.dispatch("Noor".to_owned()).unwrap();
        order.deliver().unwrap();
        assert_eq!(order.status(), &Status::Delivered);
    }

    #[test]
    #[ignore = "implement command parsing"]
    fn parses_commands() {
        assert_eq!(parse_command("list"), Ok(Command::List));
        assert_eq!(
            parse_command("dispatch 1 Noor"),
            Ok(Command::Dispatch {
                id: 1,
                courier: "Noor".to_owned()
            })
        );
    }
}
