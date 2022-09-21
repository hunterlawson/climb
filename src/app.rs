use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::command::*;

pub struct ClimbApp<'a> {
    command_table: HashMap<&'a str, Command<'a>>,
    default_command: Command<'a>,
}

impl<'a> ClimbApp<'a> {
    // Create a new application
    pub fn new(default_command: Command<'a>) -> Result<ClimbApp<'a>, String> {
        ClimbApp::validate_command(&default_command)?;
        let app = ClimbApp {
            command_table: HashMap::<&'a str, Command<'a>>::new(),
            default_command: default_command,
        };

        Ok(app)
    }

    // Add a command to the application
    pub fn add_command(&mut self, command: Command<'a>) -> Result<&ClimbApp, String> {
        ClimbApp::validate_command(&command)?;

        // Add the command to the command table
        self.command_table.insert(command.alias, command);

        Ok(self)
    }

    // Validate command settings, return nothing on success
    fn validate_command(command: &Command<'a>) -> Result<(), String> {
        // Validate the option strings
        for option in &command.options {
            if !option.chars().all(|c| c.is_alphabetic() || c == '?') {
                return Err(format!(
                    "Invalid option string given for function {}",
                    command.name
                ));
            }
        }

        // Validate the alias string
        if !command.alias.chars().all(|c| c.is_alphabetic()) {
            return Err(format!(
                "Invalid alias string given for function {}",
                command.name
            ));
        }

        Ok(())
    }

    // Run the application, parse the input, and execute the corresponding function
    pub fn run(&self, args: Vec<String>) -> Result<Option<String>, String> {
        if args.len() <= 1 {
            self.print_help();
            return Ok(None);
        }

        // Check if executing a command or a default option
        let first_arg = match args.get(1) {
            Some(arg) => arg,
            None => return Err(format!("There has been an error parsing the arguments",)),
        };

        // Default command
        if let Some('-') = first_arg.chars().nth(0) {
            let args = args.into_iter().skip(1).collect();
            match self.parse_args(args, &self.default_command.options, self.default_command.num_inputs)? {
                Some(parsed_input) => return Ok(self.default_command._run_command(parsed_input)?),
                None => return Err(format!("Incorrect use of options")),
            }
        }

        // Check if the given command exists
        if let Some(command) = self.command_table.get(first_arg.as_str()) {
            // Run the command
            let args = args.into_iter().skip(2).collect();
            match self.parse_args(args, &command.options, command.num_inputs)? {
                Some(parsed_input) => return Ok(command._run_command(parsed_input)?),
                None => return Err(format!("Incorrect use of command {}", command.name)),
            }
        } else {
            return Err(format!("The given command does not exist: `{}`", first_arg));
        }
    }

    fn parse_args(
        &self,
        args: Vec<String>,
        options: &HashSet<&str>,
        num_inputs: usize,
    ) -> Result<Option<(CommandInput, CommandOptions)>, String> {
        let mut command_input = Vec::<String>::new();
        let mut command_options = Vec::<CommandOption>::new();

        let mut it = args.into_iter();
        while let Some(mut arg) = it.next() {
            // Check if the arg is an option
            if arg.chars().nth(0).unwrap() == '-' {
                arg.remove(0);
                // Normal option
                if options.contains(arg.as_str()) {
                    command_options.push(CommandOption(arg, None));
                } else if options.contains(arg.clone().add("?").as_str()) {
                    // Get the next argument as input
                    if let Some(next_arg) = it.next() {
                        // If the next argument is an option, return error message
                        if next_arg.chars().nth(0).unwrap() != '-' {
                            command_options.push(CommandOption(arg, Some(next_arg)));
                        } else {
                            return Err(format!("No argument provided for option `{}`", arg));
                        }
                    } else {
                        // If there isn't another argument, return error message
                        return Err(format!("No argument provided for option `{}`", arg));
                    }
                } else {
                    // The option doest exist, return error message
                    return Err(format!("Invalid option `{}`", arg));
                }
            } else {
                // If the argument isnt an option, then it is input text
                command_input.push(arg);
            }
        }

        let command_input_option = match command_input.len() {
            x if x == num_inputs && num_inputs != 0 => Some(command_input),
            x if x < num_inputs => return Err(format!("Not enough amount of arguments provided")),
            x if x > num_inputs => return Err(format!("Too many arguments provided")),
            _ => None,
        };

        let command_options_option = match command_options.len() {
            0 => None,
            _ => Some(command_options),
        };

        Ok(Some((command_input_option, command_options_option)))
    }

    fn print_help(&self) {
        println!("HELP SCREEN");
    }
}
