use std::borrow::BorrowMut;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use lalrpop_util::lalrpop_mod;
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::{CompletionType, Config, EditMode, Editor, OutputStreamType};
use signal_hook::consts::SIGINT;

use crate::shell::common::state::{new_state, State};
use crate::shell::common::types::Cmd;
use crate::shell::prompt::prompt::{read_input, PromptError};
use shell::handle_command::{handle_command, CommandStatus};
use shell::parse_command::parse_input;
use shell::parse_command::ParseError;
use shell::rl_helper::RLHelper;
use std::fs::File;
use std::io;
use std::io::{BufRead, ErrorKind};
use std::path::Path;
use std::process::exit;

mod shell;

lalrpop_mod!(pub grammar);
lalrpop_mod!(pub expansions);
lalrpop_mod!(pub replacements);
lalrpop_mod!(pub prompt);

fn main() {
    let mut state = new_state().expect("vrsh");

    let history_file = format!("{}/.vrsh_history", state.home);

    signal_handling();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();

    let helper = RLHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter {},
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(helper));

    match rl.load_history(history_file.as_str()) {
        Err(e) => println!("vrsh: failed to read history file 📖: {}", e),
        _ => {}
    };

    // Read the init_file line by line.
    let init_file = format!("{}/.vrshrc", state.home);
    println!("vrsh: using init file 📄 '{}'", init_file);
    let init_lines = read_or_create_init_file(init_file);
    for line in init_lines.into_iter() {
        let cmd = parse_input(line.clone(), &mut state);
        handle_cmd(cmd, &line, &mut state)
    }

    loop {
        let input = match read_input(rl.borrow_mut(), &mut state) {
            Ok(v) => v,
            Err(e) => match e {
                PromptError::Ignore => continue,
                _ => {
                    println!("vrsh: failed to read input 🔎: {}", e);
                    continue;
                }
            },
        };
        rl.add_history_entry(input.clone());
        let cmd = parse_input(input.clone(), &mut state);
        handle_cmd(cmd, &input, &mut state);

        match rl.save_history(history_file.as_str()) {
            Ok(_) => {}
            Err(e) => println!(
                "vrsh: failed to save to history file 📄 '{}': {}",
                history_file, e
            ),
        }
    }
}

fn handle_cmd(cmd: Result<Cmd, ParseError>, line: &str, state: &mut State) {
    // 📜 🔨 😄
    match cmd {
        Ok(command) => match handle_command(command, state) {
            Ok(val) => match val {
                CommandStatus::Ok => {}
                CommandStatus::Exit => exit(0),
            },
            Err(e) => println!("vrsh 😇: {}", e),
        },
        Err(ParseError::InputEmpty) => {}
        Err(ParseError::Comment) => {}
        Err(e) => println!("vrsh: failed to parse '{}' due to {} 😭", line, e),
    }
}

// 🚦
fn signal_handling() {
    match signal_hook::flag::register(SIGINT, Arc::new(AtomicBool::new(false))) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to setup signal handling 🚦 {}", e)
        }
    }
}

fn read_or_create_init_file(file_path: String) -> Vec<String> {
    let path = Path::new(&file_path);
    let file = match File::open(&path) {
        Ok(v) => v, // 👓
        Err(e) => {
            // 🖌
            match e.kind() {
                ErrorKind::NotFound => {
                    println!(
                        "vrsh: unable to find init file '{}', creating a new one 🖌",
                        file_path
                    );
                    File::create(file_path).expect("vrsh: failed to create init file, aborting");
                    return vec![];
                }
                _ => panic!("vrsh: unable to open init file: {} ❌👓❌", e),
            }
        }
    };

    io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect(&format!(
            "vrsh: failed to read init file {} ❌👓❌",
            file_path
        ))
}
