//! # Climb
//!
//! Climb is a simple Rust crate for creating CLI applications.
//! Allows for creating commands that accept inputs, options, and optional inputs.
//! Climb handles all input argument validation and parsing and guarantees
//! that only the correct number of inputs and only valid options are passed
//! into your functions. Climb will also generate help menus for your
//! application and commands.
//!
//! This is an example calculator application "`cool_calc`"" that was created with
//! climb. For a walkthrough of how to create this example application, check out the
//! README on the [Climb page on crates.io](https://crates.io/crates/climb).
//!
//! ```text
//! $ cool_calc
//!
//! This app does some cool math
//!
//! USAGE:
//!         cool_calc [OPTIONS] [COMMAND]
//!
//! OPTIONS:
//!         -h, --help                   Print help information
//!         -v, --version                Print version
//!
//! COMMANDS:
//!         add        Add two numbers
//!         div        Divide two numbers
//!
//! Run `cool_calc [COMMAND] --help` to see help information for a specific command
//! ```
//!
//! ```text
//! $ cool_calc add 10 20
//!
//! 30
//! ```
//!
//! ```text
//! $ cool_calc div --round 100 3
//!
//! 33
//! ```
//!
//! ```text
//! $ cool_calc div --help
//!
//! Divide two numbers
//! USAGE:
//!         cool_calc div [OPTIONS] <NUMBER_A> <NUMBER_B>
//!
//! ARGS:
//!         <NUMBER_A>
//!         <NUMBER_B>
//!
//! OPTIONS:
//!         -h--help                     Print help information
//!             --round                  Round the result
//! ```

mod app;
mod command;
mod help;
mod types;

pub use app::App;
pub use command::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use crate::*;

    fn add_fn(input: FunctionInput, _: FunctionOptions) -> FunctionResult {
        let num_a: i32 = input.get(0).unwrap().parse().unwrap();
        let num_b: i32 = input.get(1).unwrap().parse().unwrap();

        let result = num_a + num_b;

        Ok(Some(result.to_string()))
    }

    fn div_fn(input: FunctionInput, options: FunctionOptions) -> FunctionResult {
        let num_a: f32 = input.get(0).unwrap().parse().unwrap();
        let num_b: f32 = input.get(1).unwrap().parse().unwrap();

        let mut result = num_a / num_b;

        if options.contains(&FunctionOption(String::from("--round"), None)) {
            result = result.round();
        }

        Ok(Some(result.to_string()))
    }

    #[test]
    fn command_new() {
        let command = Command::new("command_alias", "command_desc", add_fn);

        assert_eq!("command_alias", command.alias_long.as_str());
        assert_eq!("command_desc", command.desc.as_str());
    }

    #[test]
    fn app_name() {
        let app = create_app!().name("testing_name");
        assert_eq!("testing_name", app.name.as_str());
    }

    #[test]
    fn app_desc() {
        let app = create_app!().desc("test_description");
        assert_eq!("test_description", app.desc.as_str());
    }

    #[test]
    fn app_version() {
        let app = create_app!().version("1.2.4");
        assert_eq!("1.2.4", app.version.as_str());
    }

    #[test]
    fn app_functionality_add() {
        let add_cmd = Command::new("add", "Add two numbers", add_fn)
            .arg("a")
            .arg("b");

        let app = create_app!().name("app_name").command(add_cmd);

        let res = app.run_custom(vec![
            "app_name".to_string(),
            "add".to_string(),
            "9".to_string(),
            "10".to_string(),
        ]);

        assert_eq!("19", res.unwrap().unwrap());
    }

    #[test]
    fn app_functionality_add_invalid_input() {
        let add_cmd = Command::new("add", "Add two numbers", add_fn)
            .arg("a")
            .arg("b");

        let app = create_app!().name("app_name").command(add_cmd);

        let res = app.run_custom(vec![
            "app_name".to_string(),
            "add".to_string(),
            "9".to_string(),
        ]);

        assert_eq!(None, res.unwrap());
    }

    #[test]
    fn app_functionality_div() {
        let add_cmd = Command::new("div", "Divide two numbers", div_fn)
            .arg("a")
            .arg("b");

        let app = create_app!().name("app_name").command(add_cmd);

        let res = app.run_custom(vec![
            "app_name".to_string(),
            "div".to_string(),
            "10".to_string(),
            "3".to_string(),
        ]);

        assert!(res.unwrap().unwrap().starts_with("3.33"));
    }

    
    #[test]
    fn app_functionality_div_option() {
        let add_cmd = Command::new("div", "Divide two numbers", div_fn)
            .arg("a")
            .arg("b")
            .option(CommandOption::new("round", "round the result"));

        let app = create_app!().name("app_name").command(add_cmd);

        let res = app.run_custom(vec![
            "app_name".to_string(),
            "div".to_string(),
            "--round".to_string(),
            "10".to_string(),
            "3".to_string(),
        ]);

        assert_eq!("3", res.unwrap().unwrap());
    }

    #[test]
    fn app_functionality_div_invalid_option() {
        let add_cmd = Command::new("div", "Divide two numbers", div_fn)
            .arg("a")
            .arg("b")
            .option(CommandOption::new("round", "round the result"));

        let app = create_app!().name("app_name").command(add_cmd);

        let res = app.run_custom(vec![
            "app_name".to_string(),
            "div".to_string(),
            "--invalid-option".to_string(),
            "10".to_string(),
            "3".to_string(),
        ]);

        assert_eq!(None, res.unwrap());
    }
}
