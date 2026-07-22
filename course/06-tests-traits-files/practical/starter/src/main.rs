use std::path::Path;

use persistent_dispatch_starter::{Order, load_records, save_records};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("dispatch-data.txt");
    let orders = vec![Order {
        id: 42,
        customer: "Amina Nuru".to_owned(),
        priority: true,
    }];
    save_records(path, &orders)?;
    println!(
        "Saved and loaded {} order(s)",
        load_records::<Order>(path)?.len()
    );
    Ok(())
}
