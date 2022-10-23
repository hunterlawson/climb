use crate::{command::{CommandOption, Command}, App};

use colored::*;

pub(crate) fn print_help_app(app: &App, error_msg: Option<String>) {
    println!("{}\n", app.description);

    println!("USAGE:\n\t{} [OPTIONS] [COMMAND]", app.name);

    println!("\nOPTIONS:");
    for option in &app.options {
        // Print the short alias first, if possible
        println!("{}", format_option_str(option));
    }

    println!("\nCOMMANDS:");
    for command in &app.commands {
        // Print the long aliases first
        println!("{}", format_command_str(command));
    }

    println!("\nRun `{} [COMMAND] --help` to see help information for a specific command", app.name);

    // If there was an error, print the message at the bottom of the help screen
    if let Some(msg) = error_msg {
        println!("\n{}", msg.red());
    }
}

pub(crate) fn print_help_command(app: &App, command: &Command, error_msg: Option<String>) {
    println!("{}\n", command.description);

    println!("USAGE:\n{}\n", format_command_usage_str(app, command));

    if !command.arguments.is_empty() {
        println!("ARGS:");
        for arg in &command.arguments {
            println!("\t<{}>", arg);
        }
        println!();
    }

    if !command.options.is_empty() {
        println!("OPTIONS:");
        for option in &command.options {
            // Print the short alias first, if possible
            println!("{}", format_option_str(option));
        }
    }

    if let Some(msg) = error_msg {
        println!("\n{}", msg.red());
    }
}

fn format_command_usage_str(app: &App, command: &Command) -> String {
    let mut command_str = String::from("\t");

    command_str.push_str(format!("{} {}", app.name, command.alias_long).as_str());

    if !command.options.is_empty() {
        command_str.push_str(" [OPTIONS]");
    }

    for arg_name in &command.arguments {
        command_str.push_str(format!(" <{}>", arg_name).as_str());
    }

    command_str
}

fn format_option_str(option: &CommandOption) -> String {
    let mut option_str = String::from("\t");

    if let Some(alias_short) = &option.alias_short {
        option_str.push_str(format!("{}, ", alias_short).as_str());
    } else {
        option_str.push_str("    ");
    }

    option_str.push_str(&option.alias_long.as_str());

    if let Some(argument_name) = &option.argument {
        option_str.push_str(format!(" <{}>", argument_name).as_str());
    }

    option_str = format!("{:<30}{}", option_str, &option.description);
    option_str
}

fn format_command_str(command: &Command) -> String {
    let mut command_str = String::from("\t");

    command_str.push_str(format!("{}", &command.alias_long).as_str());

    if let Some(alias_short) = &command.alias_short {
        command_str.push_str(format!(", {}", alias_short).as_str());
    }

    command_str = format!("{:<12}{}", command_str, &command.description);
    command_str
}

pub(crate) fn print_version(app: &App) {
    println!("{} {}", app.name, app.version);
}