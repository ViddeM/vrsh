use rustyline_derive::{Helper};
use rustyline::validate::Validator;
use rustyline::error::ReadlineError;
use rustyline::completion::{FilenameCompleter, Pair, Completer};
use rustyline::hint::{HistoryHinter, Hinter};
use rustyline::highlight::{MatchingBracketHighlighter, Highlighter};
use rustyline::Context;


#[derive(Helper)]
pub struct RLHelper {
    pub completer: FilenameCompleter,
    pub hinter: HistoryHinter,
    pub colored_prompt: MatchingBracketHighlighter
}

impl Validator for RLHelper {

}

impl Completer for RLHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Highlighter for RLHelper {

}

impl Hinter for RLHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}