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
pub struct Arg {
    pub word: String,
    pub is_string: bool,
}

#[derive(Debug, Clone)]
pub enum Redirect {
    In(String),
    Out(String),
}
