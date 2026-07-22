mod model;
mod store;

use std::{collections::HashMap, path::Path};

pub use model::{DomainError, Order, Status};
pub use store::StoreError;

#[derive(Debug, PartialEq)]
pub enum Command {
    New,
    Add {
        customer: String,
        address: String,
        distance_km: f64,
        speed_kmh: f64,
        preparation_minutes: u32,
        priority: bool,
    },
    List,
    Find(String),
    Dispatch {
        id: u32,
        courier: String,
    },
    Deliver(u32),
    Summary,
    Save,
    Help,
    Quit,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("enter a command".to_owned());
    }

    let mut parts = input.splitn(2, char::is_whitespace);
    let verb = parts.next().unwrap_or("").to_ascii_lowercase();
    let arguments = parts.next().unwrap_or("").trim();

    match verb.as_str() {
        "new" if arguments.is_empty() => Ok(Command::New),
        "add" if arguments.is_empty() => Ok(Command::New),
        "add" => parse_add(arguments),
        "list" | "ls" if arguments.is_empty() => Ok(Command::List),
        "find" | "search" if !arguments.is_empty() => Ok(Command::Find(arguments.to_owned())),
        "find" | "search" => Err("find needs a customer or address to search for".to_owned()),
        "dispatch" => parse_dispatch(arguments),
        "deliver" => Ok(Command::Deliver(parse_value(arguments, "order id")?)),
        "summary" | "status" if arguments.is_empty() => Ok(Command::Summary),
        "save" if arguments.is_empty() => Ok(Command::Save),
        "help" | "?" if arguments.is_empty() => Ok(Command::Help),
        "quit" | "exit" | "q" if arguments.is_empty() => Ok(Command::Quit),
        "new" | "list" | "ls" | "summary" | "status" | "save" | "help" | "?" | "quit" | "exit"
        | "q" => Err(format!("{verb} does not take extra information")),
        _ => Err("unknown command; enter help to see available commands".to_owned()),
    }
}

fn parse_dispatch(arguments: &str) -> Result<Command, String> {
    let mut parts = arguments.trim().splitn(2, char::is_whitespace);
    let id = parse_value(parts.next().unwrap_or(""), "order id")?;
    let courier = parts.next().unwrap_or("").trim();
    if courier.is_empty() {
        return Err("dispatch requires a courier".to_owned());
    }
    Ok(Command::Dispatch {
        id,
        courier: courier.to_owned(),
    })
}

fn parse_add(arguments: &str) -> Result<Command, String> {
    let fields: Vec<_> = arguments.split('|').map(str::trim).collect();
    if fields.len() != 6 {
        return Err(format!("add requires 6 fields but found {}", fields.len()));
    }
    let priority = match fields[5].to_ascii_lowercase().as_str() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => return Err("priority must be yes or no".to_owned()),
    };
    Ok(Command::Add {
        customer: fields[0].to_owned(),
        address: fields[1].to_owned(),
        distance_km: parse_value(fields[2], "distance")?,
        speed_kmh: parse_value(fields[3], "speed")?,
        preparation_minutes: parse_value(fields[4], "preparation")?,
        priority,
    })
}

fn parse_value<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("invalid {field}: {value}"))
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

    pub fn from_orders(orders: Vec<Order>) -> Self {
        let next_id = orders
            .iter()
            .map(Order::id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        Self { orders, next_id }
    }

    pub fn load(path: &Path) -> Result<Self, StoreError> {
        Ok(Self::from_orders(store::load(path)?))
    }

    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        store::save(path, &self.orders)
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn order(&self, id: u32) -> Option<&Order> {
        self.orders.iter().find(|order| order.id() == id)
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
        let id = self.next_id;
        let order = Order::new(
            id,
            customer,
            address,
            distance_km,
            speed_kmh,
            preparation_minutes,
            priority,
        )?;
        self.orders.push(order);
        self.next_id = self.next_id.saturating_add(1);
        Ok(id)
    }

    pub fn dispatch(&mut self, id: u32, courier: String) -> Result<(), DomainError> {
        self.find_mut(id)?.dispatch(courier)
    }

    pub fn deliver(&mut self, id: u32) -> Result<(), DomainError> {
        self.find_mut(id)?.deliver()
    }

    pub fn search(&self, query: &str) -> Vec<&Order> {
        let query = query.to_lowercase();
        self.orders
            .iter()
            .filter(|order| {
                order.customer().to_lowercase().contains(&query)
                    || order.address().to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn summary(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        for order in &self.orders {
            *counts.entry(order.status().label()).or_insert(0) += 1;
        }
        counts
    }

    fn find_mut(&mut self, id: u32) -> Result<&mut Order, DomainError> {
        self.orders
            .iter_mut()
            .find(|order| order.id() == id)
            .ok_or(DomainError::OrderNotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_sample(desk: &mut DispatchDesk, priority: bool) -> u32 {
        desk.add(
            "Amina Nuru".to_owned(),
            "101 Baobab Way".to_owned(),
            12.0,
            30.0,
            10,
            priority,
        )
        .unwrap()
    }

    #[test]
    fn estimates_standard_and_priority_orders() {
        let mut desk = DispatchDesk::new();
        let standard = add_sample(&mut desk, false);
        let priority = add_sample(&mut desk, true);
        assert_eq!(
            desk.orders()[(standard - 1) as usize].estimate_minutes(),
            34
        );
        assert_eq!(
            desk.orders()[(priority - 1) as usize].estimate_minutes(),
            24
        );
    }

    #[test]
    fn validates_domain_input() {
        assert_eq!(
            Order::new(1, "".to_owned(), "Address".to_owned(), 1.0, 1.0, 0, false),
            Err(DomainError::EmptyField("customer"))
        );
        assert!(
            Order::new(
                1,
                "Amina".to_owned(),
                "A".to_owned(),
                f64::NAN,
                1.0,
                0,
                false
            )
            .is_err()
        );
    }

    #[test]
    fn enforces_status_transitions() {
        let mut desk = DispatchDesk::new();
        let id = add_sample(&mut desk, false);
        assert!(desk.deliver(id).is_err());
        desk.dispatch(id, "Noor".to_owned()).unwrap();
        desk.deliver(id).unwrap();
        assert_eq!(desk.orders()[0].status(), &Status::Delivered);
        assert!(desk.dispatch(id, "Mateo".to_owned()).is_err());
    }

    #[test]
    fn searches_and_summarizes() {
        let mut desk = DispatchDesk::new();
        let id = add_sample(&mut desk, false);
        assert_eq!(desk.search("BAOBAB").len(), 1);
        assert_eq!(desk.summary().get("ready"), Some(&1));
        desk.dispatch(id, "Noor".to_owned()).unwrap();
        assert_eq!(desk.summary().get("out-for-delivery"), Some(&1));
    }

    #[test]
    fn parses_supported_commands() {
        assert_eq!(parse_command("LIST"), Ok(Command::List));
        assert_eq!(parse_command("new"), Ok(Command::New));
        assert_eq!(parse_command("add"), Ok(Command::New));
        assert_eq!(parse_command("q"), Ok(Command::Quit));
        assert_eq!(
            parse_command("dispatch 42 Noor Rahman"),
            Ok(Command::Dispatch {
                id: 42,
                courier: "Noor Rahman".to_owned()
            })
        );
        assert!(matches!(
            parse_command("add Amina|404 Test Route|12|30|10|yes"),
            Ok(Command::Add { priority: true, .. })
        ));
        assert!(parse_command("dispatch x Noor").is_err());
        assert!(parse_command("find").is_err());
        assert!(parse_command("list extra").is_err());
    }
}
