use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Debug, PartialEq)]
pub struct Order {
    pub id: u32,
    pub customer: String,
    pub priority: bool,
}

pub trait Record: Sized {
    fn encode(&self) -> String;
    fn decode(line: &str) -> Result<Self, String>;
}

impl Record for Order {
    fn encode(&self) -> String {
        format!("{}|{}|{}", self.id, self.customer, self.priority)
    }

    fn decode(line: &str) -> Result<Self, String> {
        let fields: Vec<_> = line.split('|').collect();
        if fields.len() != 3 {
            return Err("expected id|customer|priority".to_owned());
        }
        let id = fields[0]
            .parse()
            .map_err(|_| format!("invalid id: {}", fields[0]))?;
        if fields[1].trim().is_empty() {
            return Err("customer cannot be empty".to_owned());
        }
        let priority = fields[2]
            .parse()
            .map_err(|_| format!("invalid priority: {}", fields[2]))?;
        Ok(Self {
            id,
            customer: fields[1].to_owned(),
            priority,
        })
    }
}

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    InvalidRecord { line: usize, message: String },
}

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "storage I/O failed: {error}"),
            Self::InvalidRecord { line, message } => {
                write!(formatter, "invalid record on line {line}: {message}")
            }
        }
    }
}

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::InvalidRecord { .. } => None,
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn save_records<T: Record>(path: &Path, records: &[T]) -> Result<(), StoreError> {
    let mut writer = BufWriter::new(File::create(path)?);
    for record in records {
        writeln!(writer, "{}", record.encode())?;
    }
    writer.flush()?;
    Ok(())
}

pub fn load_records<T: Record>(path: &Path) -> Result<Vec<T>, StoreError> {
    let reader = BufReader::new(File::open(path)?);
    reader
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line?;
            T::decode(&line).map_err(|message| StoreError::InvalidRecord {
                line: index + 1,
                message,
            })
        })
        .collect()
}

pub fn priority_customers(orders: &[Order]) -> Vec<&str> {
    orders
        .iter()
        .filter(|order| order.priority)
        .map(|order| order.customer.as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_record_round_trips() {
        let order = Order {
            id: 42,
            customer: "Amina Nuru".to_owned(),
            priority: true,
        };
        assert_eq!(Order::decode(&order.encode()).unwrap(), order);
    }

    #[test]
    fn rejects_ambiguous_or_invalid_records() {
        assert!(Order::decode("1|Amina|Nuru|true").is_err());
        assert!(Order::decode("x|Amina|true").is_err());
        assert!(Order::decode("1||true").is_err());
    }

    #[test]
    fn borrows_priority_customer_names() {
        let orders = vec![
            Order {
                id: 1,
                customer: "Amina".to_owned(),
                priority: true,
            },
            Order {
                id: 2,
                customer: "Mateo".to_owned(),
                priority: false,
            },
        ];
        assert_eq!(priority_customers(&orders), ["Amina"]);
        assert_eq!(orders.len(), 2);
    }
}
