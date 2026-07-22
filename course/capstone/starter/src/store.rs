use std::{error::Error, fmt, fs::File, io, path::Path};

use crate::Order;

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

impl Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn save(path: &Path, orders: &[Order]) -> Result<(), StoreError> {
    // TODO: Write every field and status in a documented text format.
    let _ = orders;
    let _ = File::create(path)?;
    Ok(())
}

pub fn load(path: &Path) -> Result<Vec<Order>, StoreError> {
    // TODO: Parse every line and preserve its one-based line number on error.
    let _ = File::open(path)?;
    Ok(Vec::new())
}
