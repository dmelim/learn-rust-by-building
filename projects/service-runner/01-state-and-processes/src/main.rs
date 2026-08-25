use std::{env, error::Error};

use service_runner_state::{ServiceDefinition, Supervisor};

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().any(|argument| argument == "--example-worker") {
        println!("example worker finished its task");
        return Ok(());
    }

    let executable = env::current_exe()?;
    let definition = ServiceDefinition::new("atlas-worker", executable).arg("--example-worker");
    let mut supervisor = Supervisor::new(definition);
    let summary = supervisor.run_to_completion()?;

    println!(
        "service {} ran as process {} and exited with {}",
        summary.service_name,
        summary.pid,
        summary
            .exit_code
            .map_or_else(|| "no exit code".to_owned(), |code| code.to_string())
    );

    Ok(())
}
