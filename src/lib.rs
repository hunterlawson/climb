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
