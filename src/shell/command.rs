pub struct Cmd {
    pub parts: Vec<CmdPart>
}

pub struct CmdPart {
    pub cmd: String,
    pub args: Vec<String>,
}
