use shell::handle_command::{handle_command, CommandStatus};
use shell::parse_command::{parse_input};

mod shell;

use rustyline::{Editor, CompletionType, Config, EditMode, OutputStreamType};
use std::borrow::{BorrowMut};
use rustyline::completion::{FilenameCompleter};
use rustyline::hint::{HistoryHinter};
use rustyline::highlight::{MatchingBracketHighlighter};
use crate::shell::rl_helper::RLHelper;

use lalrpop_util::lalrpop_mod;
use signal_hook::consts::SIGINT;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use crate::shell::parse_command::{ParseError, get_home_dir};
use std::process::exit;
lalrpop_mod!(pub grammar);


fn main() {
    let home_dir = match get_home_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("vrsh: unable to find home directory: {}", e);
            exit(1);
        }
    };
    let history_file = format!("{}/.vrsh_history", home_dir);

    signal_handling();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();

    let helper = RLHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter{},
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(helper));

    match rl.load_history(history_file.as_str()) {
        _ => {}
    }

    loop {
        let cmd = parse_input(rl.borrow_mut());
        match cmd {
            Ok(command) => match handle_command(command) {
                Ok(val) => match val {
                    CommandStatus::Ok => {}
                    CommandStatus::Exit => break,
                }
                Err(e) => println!("vrsh: {}", e)
            },
            Err(ParseError::RLIgnore) => {},
            Err(e) => println!("vrsh parse error: {}", e),
        }
        match rl.save_history(history_file.as_str()) {
            Ok(_) => {}
            Err(e) => println!("vrsh: failed to save to history file '{}': {}", history_file, e)
        }
    }
}

fn signal_handling() {
    match signal_hook::flag::register(SIGINT, Arc::new(AtomicBool::new(false))) {
        Ok(_) => {},
        Err(e) => {println!("failed to setup signal handling {}", e)}
    }
}