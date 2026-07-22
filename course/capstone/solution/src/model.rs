use std::{error::Error, fmt};

const MINUTES_PER_HOUR: f64 = 60.0;
const PRIORITY_SAVING_MINUTES: u32 = 10;

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
        validate_text("customer", &customer)?;
        validate_text("address", &address)?;
        validate_positive("distance", distance_km)?;
        validate_positive("speed", speed_kmh)?;
        Ok(Self {
            id,
            customer: normalize_text(&customer),
            address: normalize_text(&address),
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
        let travel = (self.distance_km / self.speed_kmh * MINUTES_PER_HOUR).ceil() as u32;
        let preparation = if self.priority {
            self.preparation_minutes
                .saturating_sub(PRIORITY_SAVING_MINUTES)
        } else {
            self.preparation_minutes
        };
        travel.saturating_add(preparation)
    }

    pub fn dispatch(&mut self, courier: String) -> Result<(), DomainError> {
        validate_text("courier", &courier)?;
        if self.status != Status::Ready {
            return Err(DomainError::InvalidTransition {
                from: self.status.label(),
                action: "dispatch",
            });
        }
        self.status = Status::OutForDelivery {
            courier: normalize_text(&courier),
        };
        Ok(())
    }

    pub fn deliver(&mut self) -> Result<(), DomainError> {
        if !matches!(self.status, Status::OutForDelivery { .. }) {
            return Err(DomainError::InvalidTransition {
                from: self.status.label(),
                action: "deliver",
            });
        }
        self.status = Status::Delivered;
        Ok(())
    }

    pub(crate) fn restore_status(&mut self, status: Status) {
        self.status = status;
    }
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn validate_text(field: &'static str, value: &str) -> Result<(), DomainError> {
    if value.trim().is_empty() {
        return Err(DomainError::EmptyField(field));
    }
    if value.contains('|') || value.contains('\n') || value.contains('\r') {
        return Err(DomainError::UnsupportedCharacter(field));
    }
    Ok(())
}

fn validate_positive(field: &'static str, value: f64) -> Result<(), DomainError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(DomainError::InvalidPositiveNumber(field))
    }
}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    EmptyField(&'static str),
    UnsupportedCharacter(&'static str),
    InvalidPositiveNumber(&'static str),
    InvalidTransition {
        from: &'static str,
        action: &'static str,
    },
    OrderNotFound(u32),
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} cannot be empty"),
            Self::UnsupportedCharacter(field) => {
                write!(
                    formatter,
                    "{field} contains an unsupported delimiter or newline"
                )
            }
            Self::InvalidPositiveNumber(field) => {
                write!(
                    formatter,
                    "{field} must be a finite number greater than zero"
                )
            }
            Self::InvalidTransition { from, action } => {
                write!(formatter, "cannot {action} an order whose status is {from}")
            }
            Self::OrderNotFound(id) => write!(formatter, "order #{id} was not found"),
        }
    }
}

impl Error for DomainError {}
