use std::{collections::HashMap, ops::Add, option};

use crate::command::*;

pub struct ClimbApp<'a> {
    command_table: HashMap<&'a str, Command<'a>>,
    default_command: Command<'a>,
    app_name: &'a str,
}

impl<'a> ClimbApp<'a> {
    // Create a new application
    pub fn new(
        app_name: &'a str,
        mut default_command: Command<'a>,
    ) -> Result<ClimbApp<'a>, String> {
        ClimbApp::validate_command_struct(&mut default_command)?;
        let app = ClimbApp {
            command_table: HashMap::<&'a str, Command<'a>>::new(),
            default_command,
            app_name,
        };

        Ok(app)
    }

    // Add a command to the application
    pub fn add_command(&mut self, mut command: Command<'a>) -> Result<&ClimbApp, String> {
        ClimbApp::validate_command_struct(&mut command)?;

        // Add the command to the command table
        self.command_table.insert(command.alias, command);

        Ok(self)
    }

    // Validate command settings, return nothing on success
    fn validate_command_struct(command: &mut Command<'a>) -> Result<(), String> {
        // Validate the option strings
        for option in &command.options {
            if !option
                .chars()
                .all(|c| c.is_alphabetic() || c == '?' || c == '-')
            {
                return Err(format!(
                    "Invalid option string given for command {}",
                    command.name
                ));
            }
        }

        // Validate the alias string
        if !command.alias.chars().all(|c| c.is_alphabetic()) {
            return Err(format!(
                "Invalid alias string given for command {}",
                command.name
            ));
        }

        // Validate input names, option descriptions
        if command.num_inputs != command.input_names.len() {
            return Err(format!(
                "num_inputs and size of input_names do not match for command {}",
                command.name
            ));
        }

        if command.options.len() != command.option_descriptions.len() {
            return Err(format!(
                "The number of options and number of option descriptions do not match for command {}",
                command.name
            ))
        }

        // Add `--help` to the command options
        command.options.push("-help");
        command.option_descriptions.push("Show this help menu");

        Ok(())
    }

    // Run the application and handle any errors
    pub fn run(&self, args: Vec<String>) -> Result<(), String> {
        let result = self.run_command(args);

        match result {
            Ok(r) => match r {
                Some(s) => println!("{}", s),
                None => (),
            },
            Err(e) => println!("Error: {} \n\tTry using `--help`", e),
        }

        Ok(())
    }

    // Run the command and propagate any errorst that occur to the run function
    fn run_command(&self, args: Vec<String>) -> CommandResult {
        if args.len() <= 1 {
            self.print_help(HelpPrintMode::PrintApp);
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
            match self.parse_args(args, &self.default_command)? {
                Some(parsed_input) => return self.default_command._run_command(parsed_input),
                None => return Ok(None),
            }
        }

        // Check if the given command exists
        if let Some(command) = self.command_table.get(first_arg.as_str()) {
            // Run the command
            let args = args.into_iter().skip(2).collect();
            match self.parse_args(args, &command)? {
                Some(parsed_input) => return command._run_command(parsed_input),
                None => return Ok(None),
            }
        } else {
            return Err(format!("The given command does not exist: `{}`", first_arg));
        }
    }

    fn parse_args(
        &self,
        args: Vec<String>,
        command: &Command,
    ) -> Result<Option<(CommandInput, CommandOptions)>, String> {
        let mut command_input = Vec::<String>::new();
        let mut command_options = Vec::<CommandOption>::new();

        let mut contains_help = false;
        let mut it = args.into_iter();
        while let Some(mut arg) = it.next() {
            // Check if the arg is an option
            if arg.chars().nth(0).unwrap() == '-' {
                if arg.as_str() == "--help" {
                    if contains_help {
                        return Err(format!("Multiple uses of the option `--help`"));
                    }
                    contains_help = true;
                    continue;
                }
                arg.remove(0);
                // Normal option
                if command.options.contains(&arg.as_str()) {
                    command_options.push(CommandOption(arg, None));
                } else if command.options.contains(&arg.clone().add("?").as_str()) {
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

        // Print the help menu and don't run the function
        if contains_help {
            if *command == self.default_command {
                self.print_help(HelpPrintMode::PrintApp);
            } else {
                self.print_help(HelpPrintMode::PrintCommand(command));
            }
            return Ok(None);
        }

        let command_input_option = match command_input.len() {
            x if x == command.num_inputs && command.num_inputs != 0 => Some(command_input),
            x if x < command.num_inputs => {
                return Err(format!("Not enough amount of arguments provided"))
            }
            x if x > command.num_inputs => return Err(format!("Too many arguments provided")),
            _ => None,
        };

        let command_options_option = match command_options.len() {
            0 => None,
            _ => Some(command_options),
        };

        Ok(Some((command_input_option, command_options_option)))
    }

    fn print_help(&self, mode: HelpPrintMode) {
        println!(" --------- {} ---------", self.app_name);
        // Print the application's help menu
        match mode {
            HelpPrintMode::PrintApp => {
                println!("OPTIONS");
                for option in &self.default_command.options {
                    println!("\t-{}", option);
                }
                println!("COMMANDS");
                for (_, command) in &self.command_table {
                    println!("\t{}", self.format_print_command(command));
                }
            }
            HelpPrintMode::PrintCommand(command) => {
                println!("{}", command.name.to_uppercase());
                println!("\t{}\n", self.format_print_command(command));
                println!("\t{}", command.description);
                println!("OPTIONS");
                for (i, option_name) in command.options.iter().enumerate() {
                    let option_desc = command.option_descriptions.get(i).unwrap();
                    let mut option_help = String::new();
                    if option_name.chars().last().unwrap() == '?' {
                        let mut option_name_new = option_name.chars();
                        option_name_new.next_back();
                        option_help.push_str(format!("\t-{}", option_name_new.collect::<String>()).as_str());
                        option_help.push_str(format!(" <INPUT>").as_str());
                    } else {
                        option_help.push_str(format!("\t-{}", option_name).as_str());
                    }

                    option_help.push_str(format!("\t{}", option_desc).as_str());
                    println!("{}", option_help);
                }
            }
        }
    }

    fn format_print_command(&self, command: &Command) -> String {
        let mut answer = String::new();
        answer.push_str(command.alias);
        if command.options.len() >= 1 {
            answer.push_str(" [OPTIONS]");
        }
        for input_name in &command.input_names {
            answer.push_str(format!(" <{}>", input_name).as_str());
        }

        answer
    }
}

enum HelpPrintMode<'a> {
    PrintCommand(&'a Command<'a>),
    PrintApp,
}
