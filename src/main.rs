use std::process::{Command, exit};
use std::io::{stdout, Write, stdin};
use std::path::Path;
use std::env::set_current_dir;

struct Cmd {
    cmd: String,
    args: Vec<String>,
}

fn main() {
    loop {
        let cmd = parse_input();
        match cmd {
            Some(command) => {
                handle_command(command);
            },
            None => println!("Failed to parse command")
        }
    }
}

fn handle_command(command: Cmd) {
    match command.cmd.as_str() {
        "exit" => exit(0),
        "cd" => handle_dir_change(command.args),
        _ => execute_file(command)
    }
}

fn handle_dir_change(args: Vec<String>) {
    match args.len() {
        0 => println!("Must provide an argument"),
        1 => {
            match args.first() {
                Some(arg) => {
                    match set_current_dir(&Path::new(arg)) {
                        Err(e) => println!("Failed to change working dir {}", e),
                        _ => {}
                    }
                },
                None => println!("Failed to handle args")
            }
        }
        num => println!("Invalid number of arguments {}", num)
    }
}

fn execute_file(command: Cmd) {
    match Command::new(&command.cmd)
        .args(command.args)
        .status() {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to run command '{}' due to {}", &command.cmd, e)
        }
    }
}

fn parse_input() -> Option<Cmd> {
    print!("> ");
    // Make sure it is printed immediately
    stdout().flush().ok()?;

    let mut s = String::new();
    stdin().read_line(&mut s).ok()?;
    let command  = s.trim().split_whitespace().map(str::to_string).collect::<Vec<String>>();
    let cmd_args = command.split_first()?;

    return Some(Cmd {
        cmd: cmd_args.0.to_string(),
        args: cmd_args.1.into_iter().map(String::to_string).collect::<Vec<String>>(),
    })
}
