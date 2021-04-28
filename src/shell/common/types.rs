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

