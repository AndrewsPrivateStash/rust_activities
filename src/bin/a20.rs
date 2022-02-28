// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io::{self, Write};

enum States {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl States {
    fn enum_map(s: &str) -> Result<States, String> {
        match s.to_lowercase().as_str() {
            "off" => Ok(States::Off),
            "sleep" => Ok(States::Sleep),
            "reboot" => Ok(States::Reboot),
            "shutdown" => Ok(States::Shutdown),
            "hibernate" => Ok(States::Hibernate),
            _ => Err("not a valid Power State".to_string()),
        }
    }

    fn print_message(s: States) {
        match s {
            States::Off => println!("Turning Off"),
            States::Sleep => println!("Going to Sleep"),
            States::Reboot => println!("Rebooting"),
            States::Shutdown => println!("Shuting Down"),
            States::Hibernate => println!("hibernating"),
            //_ => println!("Say WAH!?"),
        }
    }
}

fn get_input() -> io::Result<String> {
    print!("please enter a command (q to quit): ");
    io::stdout().flush()?; // print! sits in buffer so must flush

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    // get input loop
    loop {
        let input = match get_input() {
            Ok(s) => s,
            Err(e) => {
                println!("whoops, bad entry {}", e);
                std::process::exit(1);
            }
        };
        match input.to_lowercase().as_str() {
            "q" | "quit" | "exit" => {
                println!("goodbye!..");
                break;
            }
            _ => (),
        }
        let mapped_value = match States::enum_map(&input) {
            Ok(ps) => ps,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        States::print_message(mapped_value);
    }
}
