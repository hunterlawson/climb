use crate::__command::{__COpt, __Command};

pub fn format_option_str(option: &__COpt) -> String {
    let mut option_str = String::from("    ");

    if let Some(alias_short) = option.alias_short {
        option_str.push_str(format!("-{}", alias_short).as_str());
    } else {
        option_str.push_str("    ");
    }

    option_str.push_str(format!(", --{}", option.alias).as_str());

    option_str = format!("{:<20}{}", option_str, option.desc);
    option_str
}

pub fn format_cmd_str(command: &dyn __Command) -> String {
    let mut command_str = String::from("    ");

    command_str.push_str(command.alias());

    if let Some(alias_short) = command.alias_short() {
        command_str.push_str(format!(", {}", alias_short).as_str());
    }

    command_str = format!("{:<12}{}", command_str, command.desc());
    command_str
}