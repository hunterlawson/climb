use crate::types::*;

///  Holds information about command options.
///
/// `CommandOption`s are the options that can be passed in when calling
/// application commands from the command line. They can also accept an
/// argument.
///
/// They can be added to commands using the [Command] API.
///
/// # Examples
///
/// Creating a command option and assigning it an alias and argument:
///
/// ```
/// let my_option = CommandOption::new(
///     "delete-folder",
///     "This option tells the command you want to delete a folder"
/// )
///
/// my_option.alias("d");
///
/// my_option.arg("folder_name")
/// ```
///
/// This option can be used with the command like this:
///
/// `[COMMAND] --delete-folder <folder_name> ...`
///
/// or
///
/// `[COMMAND] -d <folder_name> ...`
pub struct CommandOption {
    pub(crate) alias_long: String,
    pub(crate) alias_short: Option<String>,
    pub(crate) argument: Option<String>,
    pub(crate) description: String,
}

impl CommandOption {
    /// Construct and returns a default CommandOption.
    ///
    /// Creates the command option and initializes it with the given alias
    /// and description. The provided alias is the long alias. It will be
    /// prepended with `--` automatically when the application is constructed.
    ///
    /// # Arguments
    ///
    /// * `alias_long` - The long alias of the option (more than one character)
    /// * `desc` - The description of the command
    ///
    /// # Examples
    ///
    /// Creating a new command. This command will be called using the
    /// `--recursive` option in the command line:
    ///
    /// ```
    /// let my_option = CommandOption::new(
    ///     "recursive",
    ///     "This option lets the command behave recursively"
    /// )
    /// ```
    pub fn new(alias_long: &str, desc: &str) -> Self {
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
            description: String::from(desc),
        }
    }

    /// Assign a short alias to the option
    ///
    /// The short alias is a single character alias prepended by
    /// a single dash `-`. For example, `--help` can also be called with
    /// `-h`. Options do not have a short alias by default.
    ///
    /// # Arguments
    /// * `alias` - A string slice holding the short alias of the option.
    /// Must be a single character.
    ///
    /// # Examples
    ///
    /// Creating an option and adding a short alias. This option can
    /// be called using `--recursive` or `-r`:
    ///
    /// ```
    /// let my_option = CommandOption::new(
    ///     "recursive",
    ///     "This option lets the command behave recursively"
    /// )
    ///
    /// my_option.alias("r");
    /// ```
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

    /// Assign an argument to the option
    ///
    /// Option arguments are passed in after the option is provided
    /// in the command line arguments.
    ///
    /// # Arguments:
    /// * `argument_name` - The name of the argument
    ///
    /// # Examples
    ///
    /// Creating an option and adding an argument named `folder_name`.
    /// This option argument can be used like this:
    ///
    /// \[command_name\] --delete-folder <folder_name> ...
    ///
    /// ```
    /// let my_option = CommandOption::new(
    ///     "delete-folder",
    ///     "This tells the command you want to delete a folder"
    /// )
    ///
    /// my_option.arg("folder_name");
    /// ```
    pub fn arg(mut self, argument_name: &str) -> Self {
        self.argument = Some(argument_name.to_uppercase());
        self
    }
}

/// Holds information about the commands the application can call.
///
/// A command stores an alias to call it by, valid options, arguments, and
/// a function that is called whenever the command is executed. This function
/// is the code that you should write to perform the logic of the command.
///
/// When you call the command from the command line, Climb will parse the arguments
/// and pass them into your function. See more about this in the [CommandFunction]
/// documentation.
///
/// # Examples
///
/// Creating a command, changing some of its values, and adding it to
/// an application:
///
/// ```
/// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
///     println!("my example function");
/// }
///
/// let my_command: Command::new(
///     "cmd_name",
///     "cmd_desc",
///     example_cmd_fn
/// );
///
/// my_command.alias("c");
/// my_command.option(CommandOption::new(
///     "option_name",
///     "option_desc"
/// ));
/// my_command.arg("arg1");
/// my_command.arg("arg2");
///
/// let my_app: App = create_app!()
///     .command(my_command);
/// ```
pub struct Command {
    pub(crate) function: CommandFunction,
    pub(crate) alias_long: String,
    pub(crate) alias_short: Option<String>,
    pub(crate) options: Vec<CommandOption>,
    pub(crate) args: Vec<String>,
    pub(crate) desc: String,
}

impl Command {
    /// Construct and return a command with the given alias, desc,
    /// and function.
    ///
    /// # Arguments
    /// * `alias` - String slice that holds the alias used to call the
    /// command from the terminal
    /// * `desc` - String slice that holds the command description
    /// * `function` - Function that matches the [CommandFunction] signature
    ///
    /// # Examples
    ///
    /// Construct a new command:
    ///
    /// ```
    /// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
    ///     println!("my example function");
    /// }
    ///
    /// let my_command: Command::new(
    ///     "cmd_name",
    ///     "cmd_desc",
    ///     example_cmd_fn
    /// );
    /// ```
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
            options: vec![CommandOption::new("help", "Print help information").alias("h")],
            args: vec![],
            desc: String::from(desc),
        }
    }

    /// Assign an short alias to the command (single character).
    ///
    /// The command can be called using either the normal long alias or this
    /// shorter one.
    ///
    /// # Arguments
    /// * `alias` - String slice holding the short alias (must be a single
    /// character)
    ///
    /// # Examples
    ///
    /// Construct a new command and assign a new alias `c`:
    ///
    /// ```
    /// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
    ///     println!("my example function");
    /// }
    ///
    /// let my_command: Command::new(
    ///     "cmd_name",
    ///     "cmd_desc",
    ///     example_cmd_fn
    /// );
    ///
    /// my_command.alias("c");
    /// ```
    ///
    /// The command can now be called using either `cmd_name` or `c`.
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

    /// Add an option to the command.
    ///
    /// You can add as many options as you like to a command. For more
    /// about options, see [CommandOption].
    ///
    /// # Arguments
    /// * `option` - A `CommandOption` struct holding the option to be added
    ///
    /// # Examples
    ///
    /// Construct a command and add an option:
    ///
    /// ```
    /// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
    ///     println!("my example function");
    /// }
    ///
    /// let my_command: Command::new(
    ///     "cmd_name",
    ///     "cmd_desc",
    ///     example_cmd_fn
    /// );
    ///
    /// my_command.option(CommandOption::new(
    ///     "option_name",
    ///     "option_desc"
    /// ));
    /// ```
    ///
    /// The option can now be called using `[cmd_name] --option_name ...`
    pub fn option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    /// Add an argument to the command.
    ///
    /// Command arguments allow you to pass data into the command from the
    /// command line. For example, if you wanted a command to perform actions on
    /// a file, you could pass in the file name as an argument.
    ///
    /// # Arguments
    /// * `name` - String slice that holds the name of the argument
    ///
    /// # Examples
    ///
    /// Construct a new command and add an argument to it:
    ///
    /// ```
    /// fn example_cmd_fn(_: FunctionInput, _: FunctionOptions) -> FunctionResult {
    ///     println!("my example function");
    /// }
    ///
    /// let my_command: Command::new(
    ///     "cmd_name",
    ///     "cmd_desc",
    ///     example_cmd_fn
    /// );
    ///
    /// my_command.arg("arg1");
    /// ```
    ///
    /// The command can be called using:
    ///
    /// `[app_name] cmd_name <arg1> ...`
    pub fn arg(mut self, name: &str) -> Self {
        self.args.push(name.to_uppercase());
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
