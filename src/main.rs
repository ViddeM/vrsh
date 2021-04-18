use std::process::{exit};

use shell::handle_command::{handle_command, CommandStatus};
use shell::parse_command::{parse_input};

mod shell;

fn main() {
    loop {
        let cmd = parse_input();
        match cmd {
            Ok(command) => match handle_command(command) {
                CommandStatus::Ok => {}
                CommandStatus::Exit => exit(0),
            },
            Err(e) => println!("Failed to parse command: {}", e),
        }
    }
}
