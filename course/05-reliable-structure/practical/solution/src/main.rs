mod domain;
mod error;
mod service;

use error::AppError;
use service::{dispatch_by_id, parse_order};

fn run() -> Result<(), AppError> {
    let mut queue = vec![parse_order("42 | Amina Nuru | yes")?];
    let dispatched = dispatch_by_id(&mut queue, 42)?;
    println!("Dispatching #{} for {}", dispatched.id, dispatched.customer);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_order() {
        let order = parse_order("42 | Amina Nuru | YES").unwrap();
        assert_eq!(order.id, 42);
        assert_eq!(order.customer, "Amina Nuru");
        assert!(order.priority);
    }

    #[test]
    fn reports_each_invalid_field() {
        assert_eq!(
            parse_order("42|Amina").unwrap_err(),
            AppError::InvalidFieldCount { found: 2 }
        );
        assert_eq!(
            parse_order("x|Amina|yes").unwrap_err(),
            AppError::InvalidId("x".to_owned())
        );
        assert_eq!(parse_order("1| |no").unwrap_err(), AppError::EmptyCustomer);
        assert_eq!(
            parse_order("1|Amina|maybe").unwrap_err(),
            AppError::InvalidPriority("maybe".to_owned())
        );
    }

    #[test]
    fn dispatches_or_reports_missing_order() {
        let mut queue = vec![parse_order("42|Amina|no").unwrap()];
        assert_eq!(dispatch_by_id(&mut queue, 42).unwrap().id, 42);
        assert_eq!(
            dispatch_by_id(&mut queue, 9),
            Err(AppError::OrderNotFound(9))
        );
    }

    #[test]
    fn errors_have_friendly_messages() {
        assert_eq!(
            AppError::OrderNotFound(8).to_string(),
            "order #8 was not found"
        );
    }
}
