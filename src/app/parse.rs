// First token is always a command alias or option
// If the first token is an option, run the default command with that option

use super::DefCmdState;

#[derive(Debug)]
pub enum Token {
    Cmd(String),
    Opt(String),
    Arg(String),
}

// Get a list of tokens from the parsed cli arguments
#[inline(always)]
pub(crate) fn parse_args(args: Vec<String>, def_cmd_state: &DefCmdState) -> Vec<Token> {
    // println!("Parser input: {:?}", args);

    let mut tokens = vec![];

    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("-") {
            tokens.push(Token::Opt(arg.clone()));
        } else if i == 0 && matches!(def_cmd_state, DefCmdState::Def) {
            tokens.push(Token::Cmd(arg.clone()));
        } else {
            tokens.push(Token::Arg(arg.clone()));
        }
    }
    
    tokens
}

// Remove the beginning dashes from an option alias
#[inline(always)]
pub fn remove_dashes(mut opt: String) -> String {
    for _ in 0..2 {
        if opt.starts_with("-") {
            opt.remove(0);
        }
    }

    opt
}