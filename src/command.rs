use std::collections::HashSet;

pub type CommandInput = Option<Vec<String>>;
pub type CommandOptions = Option<Vec<CommandOption>>;
pub type CommandResult = Result<Option<String>, String>;
pub type CommandFunction = fn(CommandInput, CommandOptions) -> CommandResult;

pub struct Command<'a> {
    pub function: CommandFunction,
    pub name: &'a str,
    pub alias: &'a str,
    pub options: HashSet<&'a str>,
    pub num_inputs: usize,
}

impl<'a> Command<'a> {
    pub fn _run_command(&self, input: (CommandInput, CommandOptions)) -> CommandResult {
        let x = self.function;
        x(input.0, input.1)
    }
}

pub struct CommandOption(pub String, pub Option<String>);

impl PartialEq for CommandOption {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}
