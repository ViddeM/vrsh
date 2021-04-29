use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use lalrpop_util::lalrpop_mod;
use rustyline::{CompletionType, Config, EditMode, Editor, OutputStreamType};
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use signal_hook::consts::SIGINT;

use shell::handle_command::{CommandStatus, handle_command};
use shell::parse_command::{ParseError};
use shell::parse_command::parse_input;
use shell::rl_helper::RLHelper;
use crate::shell::common::state::{new_state};

mod shell;

lalrpop_mod!(pub grammar);
lalrpop_mod!(pub expansions);
lalrpop_mod!(pub replacements);

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
        _ => {}
    }

    loop {
        let cmd = parse_input(rl.borrow_mut(), &mut state);
        match cmd {
            Ok(command) => {
                match handle_command(command, &mut state) {
                    Ok(val) => match val {
                        CommandStatus::Ok => {}
                        CommandStatus::Exit => break,
                    },
                    Err(e) => println!("vrsh: {}", e),
                }
            },
            Err(ParseError::RLIgnore) => {}
            Err(e) => println!("vrsh parse error: {}", e),
        }
        match rl.save_history(history_file.as_str()) {
            Ok(_) => {}
            Err(e) => println!(
                "vrsh: failed to save to history file '{}': {}",
                history_file, e
            ),
        }
    }
}

fn signal_handling() {
    match signal_hook::flag::register(SIGINT, Arc::new(AtomicBool::new(false))) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to setup signal handling {}", e)
        }
    }
}
