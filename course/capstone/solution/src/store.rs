use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use crate::model::{DomainError, Order, Status};

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
                write!(formatter, "invalid stored order on line {line}: {message}")
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

pub fn save(path: &Path, orders: &[Order]) -> Result<(), StoreError> {
    let mut writer = BufWriter::new(File::create(path)?);
    for order in orders {
        let status = match order.status() {
            Status::Ready => "ready".to_owned(),
            Status::OutForDelivery { courier } => format!("out:{courier}"),
            Status::Delivered => "delivered".to_owned(),
        };
        writeln!(
            writer,
            "{}|{}|{}|{}|{}|{}|{}|{}",
            order.id,
            order.customer,
            order.address,
            order.distance_km,
            order.speed_kmh,
            order.preparation_minutes,
            order.priority,
            status
        )?;
    }
    writer.flush()?;
    Ok(())
}

pub fn load(path: &Path) -> Result<Vec<Order>, StoreError> {
    BufReader::new(File::open(path)?)
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line?;
            parse_record(&line).map_err(|message| StoreError::InvalidRecord {
                line: index + 1,
                message,
            })
        })
        .collect()
}

fn parse_record(line: &str) -> Result<Order, String> {
    let fields: Vec<_> = line.split('|').collect();
    if fields.len() != 8 {
        return Err(format!("expected 8 fields but found {}", fields.len()));
    }

    let id = parse(fields[0], "id")?;
    let distance = parse(fields[3], "distance")?;
    let speed = parse(fields[4], "speed")?;
    let preparation = parse(fields[5], "preparation")?;
    let priority = parse(fields[6], "priority")?;
    let status = match fields[7] {
        "ready" => Status::Ready,
        "delivered" => Status::Delivered,
        value if value.starts_with("out:") => {
            let courier = value[4..].to_owned();
            if courier.trim().is_empty() {
                return Err("out-for-delivery status needs a courier".to_owned());
            }
            Status::OutForDelivery { courier }
        }
        value => return Err(format!("invalid status: {value}")),
    };

    let mut order = Order::new(
        id,
        fields[1].to_owned(),
        fields[2].to_owned(),
        distance,
        speed,
        preparation,
        priority,
    )
    .map_err(|error: DomainError| error.to_string())?;
    order.restore_status(status);
    Ok(order)
}

fn parse<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("invalid {field}: {value}"))
}
