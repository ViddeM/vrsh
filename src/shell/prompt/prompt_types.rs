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
    CwdHomeParents, // Only the parent directories of the current working directory
    CwdHomeCurrent, // Only the current directory (not the parents)
    FGColorStart(Argument),
    FGColorEnd,
    BGColorStart(Argument),
    BGColorEnd,
    Git,
}

pub enum Argument {
    Number(u8),
    Word(String),
}
