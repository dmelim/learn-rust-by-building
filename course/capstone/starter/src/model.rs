use std::{error::Error, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Ready,
    OutForDelivery { courier: String },
    Delivered,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::OutForDelivery { .. } => "out-for-delivery",
            Self::Delivered => "delivered",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub(crate) id: u32,
    pub(crate) customer: String,
    pub(crate) address: String,
    pub(crate) distance_km: f64,
    pub(crate) speed_kmh: f64,
    pub(crate) preparation_minutes: u32,
    pub(crate) priority: bool,
    pub(crate) status: Status,
}

impl Order {
    pub fn new(
        id: u32,
        customer: String,
        address: String,
        distance_km: f64,
        speed_kmh: f64,
        preparation_minutes: u32,
        priority: bool,
    ) -> Result<Self, DomainError> {
        // TODO: Normalize and validate text and positive numeric fields.
        Ok(Self {
            id,
            customer,
            address,
            distance_km,
            speed_kmh,
            preparation_minutes,
            priority,
            status: Status::Ready,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn priority(&self) -> bool {
        self.priority
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn estimate_minutes(&self) -> u32 {
        // TODO: Reuse the Module 1 estimate rules and saturating arithmetic.
        0
    }

    pub fn dispatch(&mut self, courier: String) -> Result<(), DomainError> {
        // TODO: Allow only Ready -> OutForDelivery.
        let _ = courier;
        Err(DomainError::InvalidTransition)
    }

    pub fn deliver(&mut self) -> Result<(), DomainError> {
        // TODO: Allow only OutForDelivery -> Delivered.
        Err(DomainError::InvalidTransition)
    }
}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    InvalidInput(String),
    InvalidTransition,
    OrderNotFound(u32),
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
            Self::InvalidTransition => write!(formatter, "invalid order status transition"),
            Self::OrderNotFound(id) => write!(formatter, "order #{id} was not found"),
        }
    }
}

impl Error for DomainError {}
