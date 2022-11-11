use std::env;

use crate::command::{Command, CommandOption};
use crate::help::*;
use crate::types::*;

/// Holds information about the application: commands, options, name, version, etc.
pub struct App {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) version: String,
    pub(crate) commands: Vec<Command>,
    pub(crate) options: Vec<CommandOption>,
}

/// Macro to create an app and initialize it with the current crate name, description, and version.
///
/// These are all pulled from the environment variables. If they can't be found, placeholders will be used instead.
///
/// # Examples
///
/// ```
/// use climb::*;
///
/// let my_app = create_app!();
/// ```
/// `my_app` now stores an App struct with the values pulled from the environment.
/// If your crate was named `cool`, the application name is `cool`, etc.
#[macro_export]
macro_rules! create_app {
    () => {
        App::new()
            .name(option_env!("CARGO_PKG_NAME").unwrap_or("unnamed_app"))
            .desc(option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("default_description"))
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"))
    };
}

impl App {
    /// Constructs and returns a default App.
    ///
    /// Initializes the name, description, and version with empty strings.
    ///
    /// Use the [create_app] macro instead if you want to construct and return an app with values for the
    /// name, description, and version taken from the crate's Cargo.toml file.
    pub fn new() -> Self {
        // Implicit functions: help, version
        let options = vec![
            CommandOption::new("help", "Print help information").alias("h"),
            CommandOption::new("version", "Print version").alias("v"),
        ];

        App {
            name: String::new(),
            desc: String::new(),
            version: String::new(),
            commands: vec![],
            options,
        }
    }

    /// Set the name of the application.
    ///
    /// The name is displayed whenever the application help menu is displayed.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the application
    ///
    /// # Examples
    ///
    /// Creating an app and change its name:
    ///
    /// ```
    /// use climb::*;
    ///
    /// let my_app = create_app!().name("new_name");
    /// ```
    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    /// Set the description of the application.
    ///
    /// The description is displayed whenever the application help menu is displayed.
    ///
    /// # Arguments
    ///
    /// * 'desc' - A string slice that holds the description of the application
    ///
    /// # Examples
    ///
    /// Creating an app and changing its description:
    ///
    /// ```
    /// use climb::*;
    ///
    /// let my_app = create_app!()
    ///     .desc("Super cool app that does a lot of stuff");
    /// ```
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = String::from(desc);
        self
    }

    /// Set the version of the application.
    ///
    /// The version is displayed whenever the `--version` option is passed.
    ///
    /// # Arguments
    ///
    /// * `desc` - A string slice that holds the version of the application
    ///
    /// # Examples
    ///
    /// Creating an app and changing its version:
    ///
    /// ```
    /// use climb::*;
    ///
    /// let my_app = create_app!().version("1.0.4");
    /// ```
    pub fn version(mut self, version: &str) -> Self {
        self.version = String::from(version);
        self
    }

    /// Add a command to the application
    ///
    /// Commands give functionality to your application. They are displayed
    /// in the application help menu. You can add as many commands as you want
    /// to an application.
    ///
    /// For more information on commands, see [Command]
    ///
    /// # Arguments
    ///
    /// * `command` - A command struct to add to the application
    ///
    /// # Examples
    ///
    /// Creating an application and adding a command to it:
    ///
    /// ```
    /// use climb::*;
    ///
    /// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
    ///     println!("my example function");
    ///     Ok(None)
    /// }
    ///
    /// let my_command = Command::new(
    ///     "cmd_name",
    ///     "cmd_desc",
    ///     example_cmd_fn
    /// );
    ///
    /// let my_app = create_app!()
    ///     .command(my_command);
    /// ```
    pub fn command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Runs the application with command line arguments
    ///
    /// Collects the arguments from the command line, parses them, and passes them into
    /// the correct function. The arguments passed into the function are guaranteed to be
    /// correct and valid.
    ///
    /// The result of running the command is returned by this function. If any errors
    /// occur when parsing the command line input (command not found, wrong option name, etc.),
    /// then this function will print a help menu and return `Ok(None)`.
    ///
    /// # Examples
    ///
    /// Creating an app, changing some of its values, and running it:
    ///
    /// ```
    /// use climb::*;
    ///
    /// let my_app = create_app!()
    /// .version("1.2.3");
    ///
    /// let app_result = my_app.run();
    /// ```
    pub fn run(&self) -> Result<Option<String>, String> {
        self.run_custom(env::args().collect())
    }

    /// Runs the application with custom arguments.
    ///
    /// Behaves exactly the same as [run](`App::run()`), but allows for inputting
    /// custom arguments instead of gathering them from the command line.
    ///
    /// # Arguments
    /// * `args` - A vector of strings representing the arguments to be parsed
    pub fn run_custom(&self, args: Vec<String>) -> Result<Option<String>, String> {
        // Print help if there are no arguments
        if args.len() <= 1 {
            print_help_app(self, None);
            return Ok(None);
        }

        // If the first argument is an option, then we are not running a command
        // Check if the option is help or version
        let first_arg = args.get(1).unwrap();
        if first_arg.starts_with('-') {
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
        let Some(command) = self.lookup_command(alias) else {
            print_help_app(
                self,
                Some(format!("The given command does not exist: `{}`", alias)),
            );
            return Ok(None);
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

    // Used internally by the run function to parse the arguments
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
            if arg.starts_with('-') {
                // Check if the option is `-h` or `--help`
                match arg.as_str() {
                    "-h" | "--help" => {
                        print_help_command(self, command, None);
                        return Ok(None);
                    }
                    _ => (),
                }

                // Check if the given option exists for the function
                let Some(option) = command.has_option(arg) else {
                    return Err(format!("Given option does not exist: `{}`", arg));
                };

                // If the option takes an argument, get it and continue
                if let Some(option_name) = &option.arg {
                    let Some(next_arg) = it.next() else {
                        return Err(format!(
                            "{} not provided for option: `{}`",
                            option_name, arg
                        ));
                    };

                    // If the argument is another option, return error
                    if next_arg.starts_with('-') {
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

        if inputs.len() != command.args.len() {
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

    // Used internally by the run function to return the corresponding command
    // given its short or long alias
    fn lookup_command(&self, alias: &String) -> Option<&Command> {
        for command in &self.commands {
            let Some(equals_alias_short) = if let Some(alias_short) = &command.alias_short {
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

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
