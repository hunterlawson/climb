use crate::types::*;

pub struct CommandOption {
    pub(crate) alias_long: String,
    pub(crate) alias_short: Option<String>,
    pub(crate) argument: Option<String>,
    pub(crate) description: String,
}

impl CommandOption {
    // Construct a new option with the default long alias
    // Example:
    // --long-alias-name
    pub fn new(alias_long: &str, description: &str) -> Self {
        if alias_long.len() <= 1 {
            panic!(
                "Long option aliases should have more than a single character: `{}`",
                alias_long
            );
        }

        let mut alias = String::from(alias_long);
        alias.insert_str(0, "--");
        CommandOption {
            alias_long: alias,
            alias_short: None,
            argument: None,
            description: String::from(description),
        }
    }

    // Add a new short alias
    // Example:
    // -s, -b, etc.
    pub fn alias(mut self, alias: &str) -> Self {
        if alias.len() != 1 {
            panic!(
                "Short option aliases can only be a single character: `{}`",
                alias
            );
        }

        let mut alias = String::from(alias);
        alias.insert(0, '-');
        self.alias_short = Some(alias);
        self
    }

    pub fn arg(mut self, argument_name: &str) -> Self {
        self.argument = Some(argument_name.to_uppercase());
        self
    }
}

pub struct Command {
    pub(crate) function: CommandFunction,
    pub(crate) alias_long: String,
    pub(crate) alias_short: Option<String>,
    pub(crate) options: Vec<CommandOption>,
    pub(crate) arguments: Vec<String>,
    pub(crate) description: String,
}

impl Command {
    pub fn new(alias: &str, desc: &str, function: CommandFunction) -> Self {
        if alias.len() <= 1 {
            panic!(
                "Long command aliases should have more than a single character: `{}`",
                alias
            );
        }

        // Construct a default command with only the help menu option
        Command {
            function,
            alias_long: alias.to_lowercase(),
            alias_short: None,
            options: vec![
                CommandOption::new("help", "Print help information")
                    .alias("h")
            ],
            arguments: vec![],
            description: String::from(desc),
        }
    }

    pub fn alias(mut self, alias: &str) -> Self {
        if alias.len() != 1 {
            panic!(
                "Short command aliases can only be a single character: `{}`",
                alias
            );
        }

        self.alias_short = Some(alias.to_lowercase());
        self
    }

    pub fn option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn argument(mut self, name: &str) -> Self {
        self.arguments.push(name.to_uppercase());
        self
    }

    // If the command has the option, return a reference to it
    pub(crate) fn has_option(&self, alias: &String) -> Option<&CommandOption> {
        for option in &self.options {
            let equals_alias_short = if let Some(alias_short) = &option.alias_short {
                *alias == *alias_short
            } else {
                false
            };

            if option.alias_long == *alias || equals_alias_short {
                return Some(option);
            }
        }

        None
    }
}
