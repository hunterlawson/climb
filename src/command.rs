pub type CommandInput = Option<Vec<String>>;
pub type CommandOptions = Option<Vec<CommandOption>>;
pub type CommandResult = Result<Option<String>, String>;
pub type ClimbFunction = fn(CommandInput, CommandOptions) -> CommandResult;

#[derive(PartialEq)]
pub struct Command<'a> {
    pub function: ClimbFunction,
    pub name: &'a str,
    pub alias: &'a str,
    pub description: &'a str,
    pub options: Vec<&'a str>,
    pub option_descriptions: Vec<&'a str>,
    pub num_inputs: usize,
    pub input_names: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn _run_command(&self, input: (CommandInput, CommandOptions)) -> CommandResult {
        let x = self.function;
        x(input.0, input.1)
    }
}

pub struct CommandOption(pub String, pub Option<String>);
