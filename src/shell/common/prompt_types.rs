pub type PromptCmd = Vec<PromptCmdPart>;

pub enum PromptCmdPart {
    Escaped(PromptEscape),
    Cmd(String),
}

pub enum PromptEscape {
    EscapeChar,
    Username,
    Cwd,
    CwdHome,
}
