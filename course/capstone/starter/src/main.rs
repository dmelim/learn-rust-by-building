use std::io::{self, Write};

use dispatch_desk_starter::{Command, parse_command};

fn main() {
    println!("Dispatch Desk — capstone scaffold");
    println!("================================");
    println!("Build each stage from course/capstone/README.md.");
    println!("The command loop is ready; implement parsing to bring it to life.");

    loop {
        print!("\ndispatch> ");
        io::stdout().flush().expect("failed to flush prompt");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) if input.trim().is_empty() => {}
            Ok(_) => match parse_command(&input) {
                Ok(Command::Help) => println!("Try: new, list, find, dispatch, summary, quit"),
                Ok(Command::Quit) => {
                    println!("See you next shift!");
                    break;
                }
                Ok(command) => println!("Parsed {command:?}. Now connect it to DispatchDesk."),
                Err(error) => println!("Not quite: {error}"),
            },
            Err(error) => {
                eprintln!("Could not read input: {error}");
                break;
            }
        }
    }
}
