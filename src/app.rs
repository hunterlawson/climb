use std::env;

use crate::command::{Command, CommandOption};
use crate::help::*;
use crate::types::*;

pub struct App {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) version: String,
    pub(crate) commands: Vec<Command>,
    pub(crate) options: Vec<CommandOption>,
}

// Macro to create an app and initialize it with the current crate information:
// name, description, version
#[macro_export]
macro_rules! create_app {
    () => {
        App::new()
            .name(option_env!("CARGO_PKG_NAME").unwrap_or("unnamed_app"))
            .description(option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("default_description"))
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"))
    };
}

impl App {
    // App constructor
    // Use the create_app! macro instead to construct and return an app with values for the
    // name, description, and version taken from the crate's Cargo.toml file
    pub fn new() -> Self {
        // Implicit functions: help, version
        let options = vec![
            CommandOption::new("help", "Print help information").alias("h"),
            CommandOption::new("version", "Print version").alias("v"),
        ];

        App {
            name: String::new(),
            description: String::new(),
            version: String::new(),
            commands: vec![],
            options,
        }
    }

    // Set the name of the app
    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    // Set the description of the app
    pub fn description(mut self, desc: &str) -> Self {
        self.description = String::from(desc);
        self
    }

    // Set the version of the app
    pub fn version(mut self, version: &str) -> Self {
        self.version = String::from(version);
        self
    }

    // Add a command to the app
    pub fn command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    // Collects arguments from the command line, parses them, and runs the correct function
    // If any errors occur unrelated to the function, this function prints the help menu and the error
    // Otherwise, this function returns the result of the executed command
    pub fn run(&self) -> Result<Option<String>, String> {
        self.run_custom(env::args().collect())
    }

    // Same as `run`, but supports input of custom arguments
    pub fn run_custom(&self, args: Vec<String>) -> Result<Option<String>, String> {
        // Print help if there are no arguments
        if args.len() <= 1 {
            print_help_app(self, None);
            return Ok(None);
        }

        // If the first argument is an option, then we are not running a command
        // Check if the option is help or version
        let first_arg = args.get(1).unwrap();
        if first_arg.starts_with("-") {
            match first_arg.as_str() {
                "-h" | "--help" => print_help_app(self, None),
                "-v" | "--version" => print_version(self),
                x => print_help_app(
                    self,
                    Some(format!("The given option does not exist: `{}`", x)),
                ),
            }
            return Ok(None);
        }

        // Otherwise, the first argument is a command
        // Get the command if it exists, else print a help screen
        let alias = args.get(1).unwrap();
        let command = match self.lookup_command(alias) {
            Some(c) => c,
            None => {
                print_help_app(
                    self,
                    Some(format!("The given command does not exist: `{}`", alias)),
                );
                return Ok(None);
            }
        };

        // Parse the arguments and pass them into the command to be executed
        let (input, options) = match self.parse_args(command, args) {
            Ok(Some((i, o))) => (i, o),
            Ok(None) => return Ok(None),
            Err(e) => {
                print_help_command(self, command, Some(e));
                return Ok(None);
            }
        };

        // Run the command function
        let function = command.function;
        function(input, options)
    }

    fn parse_args(
        &self,
        command: &Command,
        args: Vec<String>,
    ) -> Result<Option<(FunctionInput, FunctionOptions)>, String> {
        let mut inputs = Vec::<String>::new();
        let mut options = Vec::<FunctionOption>::new();

        // Create an iterator over the arguments and skip the first two elements
        // (exe name, function name)
        let mut it = args.iter().skip(2);

        // Parse the arguments
        while let Some(arg) = it.next() {
            // `arg` is an option
            if arg.chars().nth(0).unwrap() == '-' {
                // Check if the option is `-h` or `--help`
                match arg.as_str() {
                    "-h" | "--help" => {
                        print_help_command(self, command, None);
                        return Ok(None);
                    }
                    _ => (),
                }

                // Check if the given option exists for the function
                let option = match command.has_option(arg) {
                    Some(o) => o,
                    None => return Err(format!("Given option does not exist: `{}`", arg)),
                };

                // If the option takes an argument, get it and continue
                if let Some(option_name) = &option.argument {
                    let next_arg = match it.next() {
                        Some(a) => a,
                        None => {
                            return Err(format!(
                                "{} not provided for option: `{}`",
                                option_name, arg
                            ))
                        }
                    };

                    // If the argument is another option, return error
                    if next_arg.chars().nth(0).unwrap() == '-' {
                        return Err(format!(
                            "{} not provided for option: `{}`",
                            option_name, arg
                        ));
                    }

                    options.push(FunctionOption(
                        option.alias_long.clone(),
                        Some(next_arg.clone()),
                    ));
                } else {
                    options.push(FunctionOption(option.alias_long.clone(), None::<String>));
                }
            } else {
                // `arg` is a command argument
                // Check if the command takes an argument
                inputs.push(arg.clone());
            }
        }

        if inputs.len() != command.arguments.len() {
            return Err(format!(
                "Incorrect amount of arguments provided for command: {}",
                &command.alias_long
            ));
        }
        if options.len() > command.options.len() {
            return Err(format!(
                "Too many options provided for command: {}",
                &command.alias_long
            ));
        }

        Ok(Some((inputs, options)))
    }

    fn lookup_command(&self, alias: &String) -> Option<&Command> {
        for command in &self.commands {
            let equals_alias_short = if let Some(alias_short) = &command.alias_short {
                *alias_short == *alias
            } else {
                false
            };

            if *command.alias_long == *alias || equals_alias_short {
                return Some(command);
            }
        }

        None
    }
}
