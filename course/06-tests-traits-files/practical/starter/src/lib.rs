use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, BufWriter, Write},
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
        // TODO: Encode id|customer|priority.
        String::new()
    }

    fn decode(line: &str) -> Result<Self, String> {
        // TODO: Decode and validate every field.
        let _ = line;
        Err("record decoding is not implemented".to_owned())
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

impl Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn save_records<T: Record>(path: &Path, records: &[T]) -> Result<(), StoreError> {
    // TODO: Write one encoded record per line.
    let mut writer = BufWriter::new(File::create(path)?);
    let _ = records;
    writer.flush()?;
    Ok(())
}

pub fn load_records<T: Record>(path: &Path) -> Result<Vec<T>, StoreError> {
    // TODO: Read, decode, and add one-based line context.
    let _ = File::open(path)?;
    Ok(Vec::new())
}

pub fn priority_customers(orders: &[Order]) -> Vec<&str> {
    // TODO: Return borrowed priority customer names.
    orders
        .iter()
        .take(0)
        .map(|order| order.customer.as_str())
        .collect()
}
