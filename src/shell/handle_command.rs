use crate::shell::command::{Cmd, CmdPart};
use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;

pub enum CommandStatus {
    Ok,
    Exit,
}

pub fn handle_command(command: Cmd) -> CommandStatus {
    for part in command.parts.into_iter() {
        match part.cmd.as_str() {
            "exit" => return CommandStatus::Exit,
            "cd" => handle_dir_change(part.args),
            _ => execute_file(part),
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

fn execute_file(cmd: CmdPart) {
    match Command::new(&cmd.cmd).args(cmd.args).status() {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to run command '{}' due to {}", &cmd.cmd, e)
        }
    }
}
