use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use dispatch_desk_solution::{Command, DispatchDesk, Order, Status, parse_command};

const DATA_PATH: &str = "dispatch-data.txt";

fn print_order(order: &Order) {
    let status = match order.status() {
        Status::Ready => "READY".to_owned(),
        Status::OutForDelivery { courier } => format!("OUT WITH {courier}"),
        Status::Delivered => "DELIVERED".to_owned(),
    };
    println!(
        "#{}  {status}  | {} | {} min",
        order.id(),
        if order.priority() {
            "PRIORITY"
        } else {
            "standard"
        },
        order.estimate_minutes()
    );
    println!("    {}", order.customer());
    println!("    {}", order.address());
}

fn print_orders(orders: &[&Order], empty_message: &str) {
    if orders.is_empty() {
        println!("{empty_message}");
        return;
    }

    for (index, order) in orders.iter().enumerate() {
        if index > 0 {
            println!();
        }
        print_order(order);
    }
}

fn print_help() {
    println!("\nEveryday commands");
    println!("  new                     guided order entry");
    println!("  list                    show the delivery board");
    println!("  find QUERY              search customer or address");
    println!("  dispatch ID COURIER     assign a ready order");
    println!("  deliver ID              mark an order delivered");
    println!("  summary                 count orders by status");
    println!();
    println!("Desk commands");
    println!("  save                    save now (changes also auto-save)");
    println!("  help                    show this guide");
    println!("  quit                    save and finish the shift");
    println!();
    println!("Shortcut for experienced users");
    println!("  add CUSTOMER|ADDRESS|DISTANCE|SPEED|PREPARATION|PRIORITY");
}

fn prompt(label: &str) -> io::Result<Option<String>> {
    print!("{label}");
    io::stdout().flush()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        return Ok(None);
    }
    let value = input.trim();
    if value.eq_ignore_ascii_case("cancel") {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

fn prompt_text(label: &str) -> io::Result<Option<String>> {
    loop {
        match prompt(label)? {
            Some(value) if value.is_empty() => println!("  This field cannot be empty."),
            Some(value) if value.contains('|') => {
                println!("  The | character is reserved for the storage format.")
            }
            result => return Ok(result),
        }
    }
}

fn prompt_positive(label: &str) -> io::Result<Option<f64>> {
    loop {
        let Some(value) = prompt(label)? else {
            return Ok(None);
        };
        match value.parse::<f64>() {
            Ok(number) if number.is_finite() && number > 0.0 => return Ok(Some(number)),
            _ => println!("  Enter a number above zero, for example 12.5."),
        }
    }
}

fn prompt_minutes(label: &str) -> io::Result<Option<u32>> {
    loop {
        let Some(value) = prompt(label)? else {
            return Ok(None);
        };
        match value.parse() {
            Ok(minutes) => return Ok(Some(minutes)),
            Err(_) => println!("  Enter a whole number of minutes, such as 0 or 15."),
        }
    }
}

fn prompt_yes_no(label: &str) -> io::Result<Option<bool>> {
    loop {
        let Some(value) = prompt(label)? else {
            return Ok(None);
        };
        match value.to_ascii_lowercase().as_str() {
            "y" | "yes" => return Ok(Some(true)),
            "n" | "no" => return Ok(Some(false)),
            _ => println!("  Please answer y/yes or n/no."),
        }
    }
}

fn guided_order() -> io::Result<Option<Command>> {
    println!("\nLet’s put a delivery on the board.");
    println!("Type cancel at any prompt to leave without saving it.\n");

    let Some(customer) = prompt_text("Customer name: ")? else {
        return Ok(None);
    };
    let Some(address) = prompt_text("Delivery address: ")? else {
        return Ok(None);
    };
    let Some(distance_km) = prompt_positive("Distance (km): ")? else {
        return Ok(None);
    };
    let Some(speed_kmh) = prompt_positive("Expected average speed (km/h): ")? else {
        return Ok(None);
    };
    let Some(preparation_minutes) = prompt_minutes("Preparation time (minutes): ")? else {
        return Ok(None);
    };
    let Some(priority) = prompt_yes_no("Priority service? (y/n): ")? else {
        return Ok(None);
    };

    Ok(Some(Command::Add {
        customer,
        address,
        distance_km,
        speed_kmh,
        preparation_minutes,
        priority,
    }))
}

fn save_change(desk: &DispatchDesk, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    desk.save(path)?;
    Ok(())
}

fn execute(
    desk: &mut DispatchDesk,
    command: Command,
    path: &Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    match command {
        Command::New => match guided_order()? {
            Some(order) => return execute(desk, order, path),
            None => println!("No changes made — the board is just as you left it."),
        },
        Command::Add {
            customer,
            address,
            distance_km,
            speed_kmh,
            preparation_minutes,
            priority,
        } => {
            let id = desk.add(
                customer,
                address,
                distance_km,
                speed_kmh,
                preparation_minutes,
                priority,
            )?;
            save_change(desk, path)?;
            let order = desk.order(id).expect("newly added order must exist");
            println!(
                "Order #{id} is on the board — about {} minutes. Saved.",
                order.estimate_minutes()
            );
        }
        Command::List => {
            println!("\nDelivery board — {} order(s)\n", desk.orders().len());
            let orders: Vec<_> = desk.orders().iter().collect();
            print_orders(
                &orders,
                "The board is clear. Enter new to queue the first delivery.",
            );
        }
        Command::Find(query) => {
            let matches = desk.search(&query);
            println!("\nFound {} match(es) for “{query}”\n", matches.len());
            print_orders(&matches, "Nothing matched. Try a shorter name or street.");
        }
        Command::Dispatch { id, courier } => {
            let courier_for_message = courier.clone();
            desk.dispatch(id, courier)?;
            save_change(desk, path)?;
            println!("Order #{id} is out with {courier_for_message}. Saved.");
        }
        Command::Deliver(id) => {
            desk.deliver(id)?;
            save_change(desk, path)?;
            println!("Order #{id} delivered — nicely done. Saved.");
        }
        Command::Summary => {
            let counts = desk.summary();
            println!("\nShift snapshot");
            println!("  Ready                 {:>3}", count(&counts, "ready"));
            println!(
                "  Out for delivery      {:>3}",
                count(&counts, "out-for-delivery")
            );
            println!("  Delivered             {:>3}", count(&counts, "delivered"));
            println!("                         ---");
            println!("  Total                 {:>3}", desk.orders().len());
        }
        Command::Save => {
            desk.save(path)?;
            println!(
                "Saved {} order(s) to {}.",
                desk.orders().len(),
                path.display()
            );
        }
        Command::Help => print_help(),
        Command::Quit => {
            desk.save(path)?;
            println!("Saved. See you next shift!");
            return Ok(false);
        }
    }
    Ok(true)
}

fn count(counts: &std::collections::HashMap<&'static str, usize>, status: &str) -> usize {
    counts.get(status).copied().unwrap_or(0)
}

fn main() {
    let path = Path::new(DATA_PATH);
    let (mut desk, loaded_existing) = if fs::exists(path).unwrap_or(false) {
        match DispatchDesk::load(path) {
            Ok(desk) => (desk, true),
            Err(error) => {
                eprintln!("Could not open the delivery board: {error}");
                eprintln!(
                    "No data was changed. Fix or move {} and try again.",
                    path.display()
                );
                return;
            }
        }
    } else {
        (DispatchDesk::new(), false)
    };

    println!("Dispatch Desk");
    println!("=============");
    if loaded_existing {
        println!("Welcome back — loaded {} order(s).", desk.orders().len());
    } else {
        println!("Fresh board — no saved orders yet.");
    }
    println!("Enter new for guided entry, or help for every command.");

    loop {
        print!("\ndispatch> ");
        io::stdout().flush().expect("failed to flush prompt");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                match desk.save(path) {
                    Ok(()) => println!("\nSaved. See you next shift!"),
                    Err(error) => eprintln!("\nCould not save data: {error}"),
                }
                break;
            }
            Ok(_) if input.trim().is_empty() => {}
            Ok(_) => match parse_command(&input) {
                Ok(command) => match execute(&mut desk, command, path) {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => eprintln!("Couldn’t do that: {error}"),
                },
                Err(error) => eprintln!("Not quite: {error}"),
            },
            Err(error) => {
                eprintln!("Could not read input: {error}");
                break;
            }
        }
    }
}
