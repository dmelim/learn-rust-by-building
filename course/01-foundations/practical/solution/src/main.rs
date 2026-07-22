use std::io::{self, Write};

const MINUTES_PER_HOUR: f64 = 60.0;
const PRIORITY_SAVING_MINUTES: u32 = 10;

fn calculate_travel_minutes(distance_km: f64, speed_kmh: f64) -> u32 {
    (distance_km / speed_kmh * MINUTES_PER_HOUR).ceil() as u32
}

fn adjusted_preparation_minutes(preparation_minutes: u32, is_priority: bool) -> u32 {
    if is_priority {
        preparation_minutes.saturating_sub(PRIORITY_SAVING_MINUTES)
    } else {
        preparation_minutes
    }
}

fn estimate_total_minutes(
    distance_km: f64,
    speed_kmh: f64,
    preparation_minutes: u32,
    is_priority: bool,
) -> u32 {
    let travel_minutes = calculate_travel_minutes(distance_km, speed_kmh);
    let preparation_minutes = adjusted_preparation_minutes(preparation_minutes, is_priority);

    travel_minutes.saturating_add(preparation_minutes)
}

fn delivery_band(total_minutes: u32) -> &'static str {
    match total_minutes {
        0..=30 => "fast",
        31..=60 => "standard",
        _ => "extended",
    }
}

fn delivery_band_message(total_minutes: u32) -> &'static str {
    match delivery_band(total_minutes) {
        "fast" => "Fast window — this one should arrive within half an hour.",
        "standard" => "Standard window — a comfortable same-hour estimate.",
        _ => "Extended window — let the customer know it may take over an hour.",
    }
}

fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush output");

    let mut input = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    (bytes_read != 0).then_some(input)
}

fn parse_positive_f64(input: &str) -> Option<f64> {
    input
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite() && *value > 0.0)
}

fn parse_u32(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

fn parse_yes_no(input: &str) -> Option<bool> {
    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        _ => None,
    }
}

fn read_positive_f64(prompt: &str) -> Option<f64> {
    loop {
        let input = read_line(prompt)?;

        match parse_positive_f64(&input) {
            Some(value) => break Some(value),
            None => println!("  That needs to be a number above zero, for example 12.5."),
        }
    }
}

fn read_u32(prompt: &str) -> Option<u32> {
    loop {
        let input = read_line(prompt)?;

        match parse_u32(&input) {
            Some(value) => break Some(value),
            None => println!("  Use a whole number of minutes, such as 0 or 15."),
        }
    }
}

fn read_yes_no(prompt: &str) -> Option<bool> {
    loop {
        let input = read_line(prompt)?;

        match parse_yes_no(&input) {
            Some(value) => break Some(value),
            None => println!("  Please answer y/yes or n/no."),
        }
    }
}

fn run() -> Option<()> {
    println!("Delivery Desk — quick quote");
    println!("Answer four questions and I’ll build a customer-ready estimate.");

    loop {
        println!("\nNew delivery");
        println!("------------");
        let distance_km = read_positive_f64("Distance (km, e.g. 12.5): ")?;
        let speed_kmh = read_positive_f64("Expected average speed (km/h): ")?;
        let preparation_minutes = read_u32("Preparation time (whole minutes): ")?;
        let is_priority = read_yes_no("Priority service? (y/n): ")?;

        let travel_minutes = calculate_travel_minutes(distance_km, speed_kmh);
        let adjusted_preparation = adjusted_preparation_minutes(preparation_minutes, is_priority);
        let priority_saving = preparation_minutes.saturating_sub(adjusted_preparation);
        let priority_saving = if priority_saving == 0 {
            "0".to_owned()
        } else {
            format!("-{priority_saving}")
        };
        let total_minutes =
            estimate_total_minutes(distance_km, speed_kmh, preparation_minutes, is_priority);

        println!("\nYour quote");
        println!("----------");
        println!("Travel              {travel_minutes:>4} min");
        println!("Preparation         {preparation_minutes:>4} min");
        println!("Priority saving     {priority_saving:>4} min");
        println!("                      ----");
        println!("Estimated total     {total_minutes:>4} min");
        println!("{}", delivery_band_message(total_minutes));

        if !read_yes_no("\nQuote another delivery? (y/n): ")? {
            println!("Thanks — the desk is ready when you need another quote.");
            break;
        }
    }
    Some(())
}

fn main() {
    if run().is_none() {
        eprintln!("\nInput ended before the estimate was complete.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_travel_time_and_rounds_up() {
        assert_eq!(calculate_travel_minutes(12.0, 30.0), 24);
        assert_eq!(calculate_travel_minutes(1.0, 8.0), 8);
    }

    #[test]
    fn priority_adjustment_stops_at_zero() {
        assert_eq!(adjusted_preparation_minutes(25, true), 15);
        assert_eq!(adjusted_preparation_minutes(5, true), 0);
        assert_eq!(adjusted_preparation_minutes(5, false), 5);
    }

    #[test]
    fn classifies_boundaries() {
        assert_eq!(delivery_band(30), "fast");
        assert_eq!(delivery_band(31), "standard");
        assert_eq!(delivery_band(60), "standard");
        assert_eq!(delivery_band(61), "extended");
    }

    #[test]
    fn explains_each_delivery_band() {
        assert!(delivery_band_message(30).starts_with("Fast"));
        assert!(delivery_band_message(31).starts_with("Standard"));
        assert!(delivery_band_message(61).starts_with("Extended"));
    }

    #[test]
    fn calculates_complete_estimate() {
        assert_eq!(estimate_total_minutes(12.0, 30.0, 10, false), 34);
        assert_eq!(estimate_total_minutes(12.0, 30.0, 10, true), 24);
    }

    #[test]
    fn total_saturates_instead_of_overflowing() {
        assert_eq!(estimate_total_minutes(f64::MAX, 1.0, 1, false), u32::MAX);
    }

    #[test]
    fn parses_valid_input_variants() {
        assert_eq!(parse_positive_f64(" 12.5 "), Some(12.5));
        assert_eq!(parse_u32(" 10\n"), Some(10));
        assert_eq!(parse_yes_no("YES"), Some(true));
        assert_eq!(parse_yes_no("n"), Some(false));
    }

    #[test]
    fn rejects_invalid_input() {
        for input in ["0", "-1", "NaN", "inf", "distance"] {
            assert_eq!(parse_positive_f64(input), None);
        }
        assert_eq!(parse_u32("-1"), None);
        assert_eq!(parse_u32("1.5"), None);
        assert_eq!(parse_yes_no("maybe"), None);
    }
}
