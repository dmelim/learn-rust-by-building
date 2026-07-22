#[derive(Debug, PartialEq)]
enum DeliveryStatus {
    Draft,
    Ready,
    OutForDelivery { courier: String },
    Delivered,
}

#[derive(Debug, PartialEq)]
struct Order {
    id: u32,
    customer: String,
    delivery_note: Option<String>,
    priority: bool,
    status: DeliveryStatus,
}

impl Order {
    fn new(id: u32, customer: String, delivery_note: Option<String>, priority: bool) -> Self {
        // TODO: Build a draft order from every argument.
        let _ = (customer, delivery_note, priority);
        Self {
            id,
            customer: String::new(),
            delivery_note: None,
            priority: false,
            status: DeliveryStatus::Draft,
        }
    }

    fn note(&self) -> Option<&str> {
        // TODO: Borrow the optional note.
        None
    }

    fn summary(&self) -> String {
        // TODO: Include id, customer, and service level.
        String::new()
    }

    fn next_action(&self) -> String {
        // TODO: Match every status, including courier data.
        String::new()
    }
}

fn main() {
    let mut order = Order::new(
        42,
        "Amina Nuru".to_owned(),
        Some("Ring the side bell".to_owned()),
        true,
    );
    order.status = DeliveryStatus::Ready;
    println!("{}", order.summary());
    println!("Note: {}", order.note().unwrap_or("none"));
    println!("Next: {}", order.next_action());
    order.status = DeliveryStatus::OutForDelivery {
        courier: "Noor".to_owned(),
    };
    println!("Next: {}", order.next_action());
    order.status = DeliveryStatus::Delivered;
    println!("Next: {}", order.next_action());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order() -> Order {
        Order::new(42, "Amina Nuru".to_owned(), None, true)
    }

    #[test]
    #[ignore = "complete the constructor"]
    fn new_order_starts_as_draft() {
        assert_eq!(order().status, DeliveryStatus::Draft);
        assert_eq!(order().customer, "Amina Nuru");
    }

    #[test]
    #[ignore = "borrow the optional note"]
    fn exposes_optional_note_as_a_borrow() {
        let mut order = order();
        assert_eq!(order.note(), None);
        order.delivery_note = Some("Side bell".to_owned());
        assert_eq!(order.note(), Some("Side bell"));
    }

    #[test]
    #[ignore = "implement the summary"]
    fn summarizes_priority() {
        assert_eq!(order().summary(), "Order #42 for Amina Nuru (priority)");
    }

    #[test]
    #[ignore = "match every status"]
    fn describes_every_next_action() {
        let mut order = order();
        assert_eq!(order.next_action(), "complete order details");
        order.status = DeliveryStatus::Ready;
        assert_eq!(order.next_action(), "assign a courier");
        order.status = DeliveryStatus::OutForDelivery {
            courier: "Noor".to_owned(),
        };
        assert_eq!(order.next_action(), "track delivery with Noor");
        order.status = DeliveryStatus::Delivered;
        assert_eq!(order.next_action(), "archive order");
    }
}
