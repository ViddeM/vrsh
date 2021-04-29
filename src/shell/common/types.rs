use std::fmt::Formatter;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Cmd {
    pub parts: Vec<CmdPart>,
}

#[derive(Debug, Clone)]
pub struct CmdPart {
    pub cmd: String,
    pub args: Vec<Arg>,
    pub redirects: Vec<Redirect>,
}

#[derive(Debug, Clone)]
pub enum CmdPartSection {
    Redirect(Redirect),
    Arg(Arg),
}

#[derive(Debug, Clone)]
pub enum Arg {
    Word(String),
    String(String),
    Assignment(String, Assignment),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Word(w) => write!(f, "{}", w),
            Arg::String(s) => write!(f, "{}", s),
            Arg::Assignment(w, a) =>
                write!(f, "{}={}", w, match a {
                    Assignment::Word(wi) => wi,
                    Assignment::String(si) => si,
                })
        }
    }
}

#[derive(Debug, Clone)]
pub enum Assignment {
    Word(String),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Redirect {
    In(String),
    Out(String),
}

// Initial pass

#[derive(Debug, Clone)]
pub struct InitialCmd {
    pub text: String,
    pub parts: Vec<InitialCmdPart>
}

#[derive(Debug, Clone)]
pub enum InitialCmdPart {
    String(String),
    Calculation(InitialCmd)
}

// Replacements pass

#[derive(Debug, Clone)]
pub struct ReplacementsCmd {
    pub parts: Vec<ReplacementPart>
}

#[derive(Debug, Clone)]
pub enum ReplacementPart {
    String(String),
    Word(String)
}