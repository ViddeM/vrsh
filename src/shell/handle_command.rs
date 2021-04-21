use crate::shell::command::{Cmd, CmdPart};
use std::env::set_current_dir;
use std::path::Path;
use std::process::{Command, Child, Stdio};

pub enum CommandStatus {
    Ok,
    Exit,
}

// TODO: NEVER LOOK AT THIS CODE AGAIN, I THINK IT FUCKING WORKS!
pub fn handle_command(command: Cmd) -> CommandStatus {
    let mut all_prevs: Vec<Child> = Vec::new();
    let mut output = Some(Stdio::inherit());
    for (index, part) in command.parts.into_iter().enumerate().rev() {
        match part.cmd.as_str() {
            "exit" => return CommandStatus::Exit,
            "cd" => handle_dir_change(part.args),
            _ => {
                if let Some(mut c) = run_command(part,
                                                 match output {
                                                    Some(o) => {
                                                        output = None;
                                                        o
                                                    },
                                                    None => Stdio::piped()
                                                 },
                                                 if index == 0 { Stdio::inherit() }
                                                 else { Stdio::piped() } ) {

                    output = Some(match c.stdin.take() {
                        Some(v) => Stdio::from(v),
                        None => Stdio::inherit()
                    });

                    all_prevs.push(c);

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

fn run_command(part: CmdPart, output: Stdio, input: Stdio) -> Option<Child> {
    return match Command::new(&part.cmd)
        .args(part.args)
        .stdout(output)
        .stdin(input)
        .spawn() {
        Ok(c) => {
            Some(c)
        },
        Err(e) => {
            println!("Failed to spawn process {}", e);
            None
        }
    }
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
