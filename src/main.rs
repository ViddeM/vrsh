use std::process::{exit};

use shell::handle_command::{handle_command, CommandStatus};
use shell::parse_command::{parse_input};

mod shell;

use rustyline::{Editor, CompletionType, Config, EditMode, OutputStreamType};
use std::borrow::{BorrowMut};
use rustyline::completion::{FilenameCompleter};
use rustyline::hint::{HistoryHinter};
use rustyline::highlight::{MatchingBracketHighlighter};
use crate::shell::rl_helper::RLHelper;

fn main() {
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