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
        Self {
            id,
            customer,
            delivery_note,
            priority,
            status: DeliveryStatus::Draft,
        }
    }

    fn note(&self) -> Option<&str> {
        self.delivery_note.as_deref()
    }

    fn summary(&self) -> String {
        let priority = if self.priority {
            "priority"
        } else {
            "standard"
        };
        format!("Order #{} for {} ({priority})", self.id, self.customer)
    }

    fn next_action(&self) -> String {
        match &self.status {
            DeliveryStatus::Draft => "complete order details".to_owned(),
            DeliveryStatus::Ready => "assign a courier".to_owned(),
            DeliveryStatus::OutForDelivery { courier } => {
                format!("track delivery with {courier}")
            }
            DeliveryStatus::Delivered => "archive order".to_owned(),
        }
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
    fn new_order_starts_as_draft() {
        assert_eq!(order().status, DeliveryStatus::Draft);
    }

    #[test]
    fn exposes_optional_note_as_a_borrow() {
        let mut order = order();
        assert_eq!(order.note(), None);
        order.delivery_note = Some("Side bell".to_owned());
        assert_eq!(order.note(), Some("Side bell"));
    }

    #[test]
    fn summarizes_priority() {
        assert_eq!(order().summary(), "Order #42 for Amina Nuru (priority)");
    }

    #[test]
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
