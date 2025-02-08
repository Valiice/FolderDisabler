use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod operations;

fn main() -> io::Result<()> {
    // Get the current working directory.
    let current_dir = env::current_dir()?;
    println!("Operating in directory: {:?}", current_dir);

    // Ask the user whether to add or remove the prefix.
    println!("Do you want to add the 'DISABLED_' prefix to folders? (y/n)");
    print!("> ");
    io::stdout().flush()?; // Ensure the prompt is printed immediately.

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim().to_lowercase();

    if choice == "y" {
        operations::add_prefix(&current_dir)?;
    } else if choice == "n" {
        operations::remove_prefix(&current_dir)?;
    } else {
        println!("Invalid input. Please run the program again and enter 'y' or 'n'.");
    }

    println!("Operation complete.");
    thread::sleep(Duration::from_secs(3));
    Ok(())
}
