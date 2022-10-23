//! # Climb
//!
//! Climb is a simple Rust crate for creating CLI applications.
//! Allows for functions to accept inputs, options, and optional inputs.
//! Climb handles all input argument validation and parsing and guarantees
//! that only the correct number of inputs and only valid options are passed
//! into your functions. Climb will also generate help menus for your
//! application and commands.
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
//!
//! For a walkthrough of how to create this example application, check out the README on the
//! [Climb page on crates.io](https://crates.io/crates/climb).

mod app;
mod command;
mod help;
mod types;

pub use app::App;
pub use command::*;
pub use types::*;
