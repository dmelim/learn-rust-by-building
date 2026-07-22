use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Ready,
    OutForDelivery,
    Delivered,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::OutForDelivery => "out-for-delivery",
            Self::Delivered => "delivered",
        }
    }
}

#[derive(Debug, PartialEq)]
struct Order {
    id: u32,
    customer: String,
    priority: bool,
    status: Status,
}

fn search_customers<'a>(queue: &'a [Order], query: &str) -> Vec<&'a Order> {
    // TODO: Return borrowed matches without consuming the queue.
    let _ = query;
    queue.iter().take(0).collect()
}

fn status_counts(queue: &[Order]) -> HashMap<&'static str, usize> {
    // TODO: Count each status using the entry API.
    let _ = queue;
    HashMap::new()
}

fn take_next(queue: &mut Vec<Order>) -> Option<Order> {
    // TODO: Remove earliest priority, or the first order, or return None.
    let _ = queue;
    None
}

fn sample_order(id: u32, customer: &str, priority: bool, status: Status) -> Order {
    Order {
        id,
        customer: customer.to_owned(),
        priority,
        status,
    }
}

fn main() {
    let mut queue = vec![
        sample_order(1, "Amina Nuru", false, Status::Ready),
        sample_order(2, "Haru Tanaka", true, Status::Ready),
        sample_order(3, "Lucía Vega", false, Status::OutForDelivery),
        sample_order(4, "Mateo Rivera", false, Status::Delivered),
    ];
    let counts = status_counts(&queue);
    println!("Queued: {}", queue.len());
    println!(
        "Ready: {}",
        counts.get(Status::Ready.label()).copied().unwrap_or(0)
    );
    println!(
        "Customer matches: {}",
        search_customers(&queue, "amina").len()
    );
    if let Some(order) = take_next(&mut queue) {
        println!("Dispatch next: #{} for {}", order.id, order.customer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn queue() -> Vec<Order> {
        vec![
            sample_order(1, "Amina Nuru", false, Status::Ready),
            sample_order(2, "Haru Tanaka", true, Status::Ready),
            sample_order(3, "Amina Okoro", true, Status::OutForDelivery),
        ]
    }

    #[test]
    #[ignore = "implement customer search"]
    fn searches_without_consuming_orders() {
        let queue = queue();
        assert_eq!(search_customers(&queue, "AMINA").len(), 2);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    #[ignore = "implement status counts"]
    fn counts_each_status() {
        let counts = status_counts(&queue());
        assert_eq!(counts.get("ready"), Some(&2));
        assert_eq!(counts.get("out-for-delivery"), Some(&1));
    }

    #[test]
    #[ignore = "implement priority dispatch"]
    fn takes_earliest_priority_order() {
        let mut queue = queue();
        assert_eq!(take_next(&mut queue).map(|order| order.id), Some(2));
    }

    #[test]
    #[ignore = "handle fallback and empty cases"]
    fn falls_back_to_fifo_and_handles_empty_queue() {
        let mut queue = vec![sample_order(7, "Mateo", false, Status::Ready)];
        assert_eq!(take_next(&mut queue).map(|order| order.id), Some(7));
        assert_eq!(take_next(&mut queue), None);
    }
}
