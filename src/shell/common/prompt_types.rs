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
    FGColorStart(Argument),
    FGColorEnd,
    BGColorStart(Argument),
    BGColorEnd,
}

pub enum Argument {
    Number(u8),
    Word(String),
}