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
    let query = query.to_lowercase();
    queue
        .iter()
        .filter(|order| order.customer.to_lowercase().contains(&query))
        .collect()
}

fn status_counts(queue: &[Order]) -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    for order in queue {
        *counts.entry(order.status.label()).or_insert(0) += 1;
    }
    counts
}

fn take_next(queue: &mut Vec<Order>) -> Option<Order> {
    if queue.is_empty() {
        return None;
    }

    let index = queue.iter().position(|order| order.priority).unwrap_or(0);
    Some(queue.remove(index))
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
    println!("Ready: {}", counts.get("ready").copied().unwrap_or(0));
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
    fn searches_without_consuming_orders() {
        let queue = queue();
        let found = search_customers(&queue, "AMINA");
        assert_eq!(
            found.iter().map(|order| order.id).collect::<Vec<_>>(),
            [1, 3]
        );
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn counts_each_status() {
        let mut queue = queue();
        queue.push(sample_order(4, "Mateo", false, Status::Delivered));
        let counts = status_counts(&queue);
        assert_eq!(counts.get("ready"), Some(&2));
        assert_eq!(counts.get("out-for-delivery"), Some(&1));
        assert_eq!(counts.get("delivered"), Some(&1));
    }

    #[test]
    fn takes_earliest_priority_order() {
        let mut queue = queue();
        assert_eq!(take_next(&mut queue).map(|order| order.id), Some(2));
        assert_eq!(
            queue.iter().map(|order| order.id).collect::<Vec<_>>(),
            [1, 3]
        );
    }

    #[test]
    fn falls_back_to_fifo_and_handles_empty_queue() {
        let mut queue = vec![sample_order(7, "Mateo", false, Status::Ready)];
        assert_eq!(take_next(&mut queue).map(|order| order.id), Some(7));
        assert_eq!(take_next(&mut queue), None);
    }
}
