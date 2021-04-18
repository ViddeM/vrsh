use std::process::{Command, exit};
use std::io::{stdout, Write, stdin};

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
        _ => execute_file(command)
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
