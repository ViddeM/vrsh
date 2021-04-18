use std::process::{exit};

use shell::handle_command::{handle_command, CommandStatus};
use shell::parse_command::{parse_input};

mod shell;

use rustyline::{Editor};
use std::borrow::{BorrowMut};

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let cmd = parse_input(rl.borrow_mut());
        match cmd {
            Ok(command) => match handle_command(command) {
                CommandStatus::Ok => {}
                CommandStatus::Exit => exit(0),
            },
            Err(e) => println!("Failed to parse command: {}", e),
        }
    }
}