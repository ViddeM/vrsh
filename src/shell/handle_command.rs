use crate::shell::command::{Cmd};
use std::env::set_current_dir;
use std::path::Path;
use std::process::{Command, Child};

pub enum CommandStatus {
    Ok,
    Exit,
}

pub fn handle_command(command: Cmd) -> CommandStatus {
    let mut all_prevs: Vec<Child> = Vec::new();
    // let mut prev_process: Option<Child> = None;
    for (_, part) in command.parts.into_iter().enumerate().rev() {
        match part.cmd.as_str() {
            "exit" => return CommandStatus::Exit,
            "cd" => handle_dir_change(part.args),
            _ => {
                match Command::new(&part.cmd)
                    .args(part.args)
                    .spawn() {
                    Ok(c) => {
                        all_prevs.push(c);
                        // Some(c)
                    },
                    Err(e) => {
                        println!("Failed to spawn process {}", e);
                        // None
                    }
                }
            },
        }
    }

    for mut child in all_prevs.into_iter() {
        match child.wait() {
            Ok(_) => {}
            Err(e) => println!("Failed to wait for child {}", e)
        }
    }

    CommandStatus::Ok
}

fn handle_dir_change(args: Vec<String>) {
    match args.len() {
        0 => println!("must provide an argument"),
        1 => match args.first() {
            Some(arg) => match set_current_dir(&Path::new(arg)) {

                Err(e) => println!("failed to change working dir {}", e),
                _ => {}
            },
            None => println!("failed to handle args"),
        },
        num => println!("invalid amount of arguments to cd: {}", num),
    }
}
