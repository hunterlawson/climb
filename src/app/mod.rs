use self::{
    default::DefaultCommand,
    parse::{parse_args, Token},
};
use crate::{
    __command::{__COpt, __Command, __types::__FunctionResult},
    app::{parse::remove_dashes, help::{format_option_str, format_cmd_str}},
};
use std::env;

mod default;
mod help;
mod parse;

// Defaults
pub const APP_DEFAULT_NAME: &'static str = "unnamed_app";
pub const APP_DEFAULT_DESC: &'static str = "default_description";
pub const APP_DEFAULT_VERS: &'static str = "0.1.0";

#[macro_export]
macro_rules! default_app {
    () => {
        App::new(
            option_env!("CARGO_PKG_NAME").unwrap_or($crate::app::APP_DEFAULT_NAME),
            option_env!("CARGO_PKG_DESCRIPTION").unwrap_or($crate::app::APP_DEFAULT_DESC),
            option_env!("CARGO_PKG_VERSION").unwrap_or($crate::app::APP_DEFAULT_VERS),
        )
    };
}

macro_rules! option_match {
    ($op:expr, $check:ident) => {
        match $op {
            Some(s) if s == $check => true,
            Some(_) => false,
            None => false,
        }
    };
}

// Store the state of the default command (if it has been set or not)
// This is used when parsing the command line arguments to determine if inputs
// are command arguments or command aliases
pub(crate) enum DefCmdState {
    Set,
    Def,
}

pub struct App {
    name: &'static str,
    desc: &'static str,
    vers: &'static str,
    commands: Vec<Box<dyn __Command>>,
    // Default command that is executed when the application is called without any arguments
    default_command: Box<dyn __Command>,
    def_cmd_state: DefCmdState, // State of the default command (if it has been changed or not)

    // Some help menu things that can be overridden:
    help_usage: String,
    help_commands: String,
    help_footer: String,
}

impl App {
    /// Create a new app with the given name, description, and version.
    #[inline(always)]
    pub fn new(name: &'static str, desc: &'static str, version: &'static str) -> Self {
        let default_command = Box::new(DefaultCommand {
            args: vec![],
            opts: vec![
                __COpt::new("help", Some("h"), "Print help"),
                __COpt::new("version", Some("v"), "Print version"),
            ],
            opt_args: vec![],
        });

        App {
            name,
            desc,
            vers: version,
            commands: vec![],
            default_command,
            def_cmd_state: DefCmdState::Def,
            help_usage: format!("Usage: {} [COMMAND] [OPTIONS]", name),
            help_commands: format!("Available commands:"),
            help_footer: format!(
                "Use `{} [COMMAND] --help` to see more information about a certain command",
                name
            ),
        }
    }

    // TODO: Implement this as a compiler macro
    // Commands don't need to be added at runtime and duplicates can raise compiler errors

    /// Add commands to the app
    pub fn commands(mut self, mut commands: Vec<Box<dyn __Command>>) -> Self {
        // Check if there exist commands with the given aliases already
        for command in &self.commands {
            for new_command in &commands {
                if command.alias() == new_command.alias() {
                    panic!("Command already exists with alias: {}", command.alias());
                }
                if let Some(alias_short) = command.alias_short() {
                    if let Some(new_alias_short) = new_command.alias_short() {
                        if alias_short == new_alias_short {
                            panic!("Command already exists with short alias: {alias_short}");
                        }
                    }
                }
            }
        }

        self.commands.append(&mut commands);
        self
    }

    /// Set the default command of the app.
    ///
    /// This is the command that is executed when the application is ran without
    /// any input arguments. By default, the application will just display the help
    /// menu if no arguments are provided.
    pub fn default_command(mut self, command: Box<dyn __Command>) -> Self {
        self.default_command = command;
        self.def_cmd_state = DefCmdState::Set;
        self
    }

    /// Run the application by capturing input from the command line arguments.
    pub fn run(&self) -> __FunctionResult {
        // Skip over the name of the application
        self.run_custom(env::args().skip(1).collect())
    }

    pub fn run_custom(&self, args: Vec<String>) -> __FunctionResult {
        // No command given or no arguments: run default function
        let parsed_args = parse_args(args, &self.def_cmd_state);
        // println!("Parsed args: {:?}", parsed_args);

        enum State {
            Start,
            NeedOptArg(String, usize),
        }

        let mut state = State::Start;
        // The command being executed by this function
        let mut this_command = &self.default_command;
        let mut args = vec![];
        let mut options = vec![false; this_command.options().len()];
        let mut opt_args = vec![None; this_command.optional_args().len()];

        // Parse loop
        for token in parsed_args {
            match token {
                Token::Cmd(alias) => {
                    // Check that it is a valid command
                    let search_command = self
                        .commands
                        .iter()
                        .find(|&c| c.alias() == alias || option_match!(c.alias_short(), alias));
                    if let Some(c) = search_command {
                        this_command = c;
                        // Resize the things
                        options = vec![false; this_command.options().len()];
                        opt_args = vec![None; this_command.optional_args().len()];
                    } else {
                        println!("The given command does not exist: `{alias}`");
                        self.print_app_help();
                        return Ok(None);
                    }
                }
                Token::Opt(alias) => match state {
                    State::Start => {
                        let alias_nopref = remove_dashes(alias.clone());

                        // Check that it is a valid option for the current command
                        let mut index = 0;
                        let valid_opt = this_command.options().iter().any(|opt| {
                            index += 1;
                            opt.alias == alias_nopref
                                || option_match!(opt.alias_short, alias_nopref)
                        });
                        if valid_opt {
                            options[index - 1] = true;
                            continue;
                        }

                        // Check that it is a valid optional argument and requires further input
                        index = 0;
                        let valid_opt_arg = this_command.optional_args().iter().any(|oarg| {
                            index += 1;
                            oarg.alias == alias_nopref
                                || option_match!(oarg.alias_short, alias_nopref)
                        });
                        if valid_opt_arg {
                            state = State::NeedOptArg(alias, index - 1);
                            continue;
                        }

                        // If it gets here, then the option is not valid
                        println!("Invalid option provided: `{alias}`");
                        return Ok(None);
                    }
                    State::NeedOptArg(alias, _) => {
                        println!("Expected an optional argument `{alias}`.");
                        return Ok(None);
                    }
                },
                Token::Arg(value) => match state {
                    State::Start => {
                        // Check that the argument is valid for the current command
                        if args.len() < this_command.args().len() {
                            args.push(value);
                            continue;
                        } else {
                            println!("Too many arguments provided");
                            self.print_cmd_help(&this_command);
                            return Ok(None);
                        }
                    }
                    State::NeedOptArg(_, index) => {
                        opt_args[index] = Some(value);
                        state = State::Start;
                    }
                },
            }
        }

        if let State::NeedOptArg(alias, _) = state {
            println!("Expected an optional argument `{alias}`");
            return Ok(None);
        }

        // Check that the right number of arguments were parsed
        if args.len() != this_command.args().len() {
            println!("Not enough arguments provided");
            self.print_cmd_help(&this_command);
            return Ok(None);
        }

        // Call the command with the parsed input
        this_command.send_input(args, options, opt_args);
        this_command.execute()
    }

    /* -------------------------------------------------------------------------- */
    /*                              Setter Functions                              */
    /* -------------------------------------------------------------------------- */

    /// Set the name of the application.
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    /// Set the description of the application.
    pub fn desc(mut self, desc: &'static str) -> Self {
        self.desc = desc;
        self
    }

    /// Set the version of the application.
    pub fn vers(mut self, vers: &'static str) -> Self {
        self.vers = vers;
        self
    }

    /// Set the help usage text.
    ///
    /// By default it is: `Usage: <app_name> [COMMAND] [OPTIONS]`.
    pub fn help_usage(mut self, text: &'static str) -> Self {
        self.help_usage = String::from(text);
        self
    }

    /// Set the commands help text.
    ///
    /// By default it is: `Available commands:`.
    pub fn help_commands(mut self, text: &'static str) -> Self {
        self.help_commands = String::from(text);
        self
    }

    /// Set the help footer.
    ///
    /// By default it is: `Use \`<app_name> [COMMAND] --help\` to see more
    /// information about a certain command`.
    pub fn help_footer(mut self, text: &'static str) -> Self {
        self.help_footer = String::from(text);
        self
    }

    fn print_app_help(&self) {
        println!("{}\n", self.desc);
        println!("{}\n", self.help_usage);
        println!("Options:");
        for option in self.default_command.options() {
            println!("{}", format_option_str(option));
        }
        println!("\n{}", self.help_commands);
        for command in &self.commands {
            println!("{}", format_cmd_str(command.as_ref()));
        }
        println!("\n{}", self.help_footer);
    }

    fn print_cmd_help(&self, cmd: &Box<dyn __Command>) {
        todo!();
    }
}

impl Default for App {
    fn default() -> Self {
        App::new(APP_DEFAULT_NAME, APP_DEFAULT_DESC, APP_DEFAULT_VERS)
    }
}
