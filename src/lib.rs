//! # Climb
//! 
//! Climb is a simple Rust crate for creating CLI applications. 
//! Allows for functions to accept inputs, options, and optional inputs. 
//! Climb handles all input argument validation and parsing and guarantees 
//! that only the correct number of inputs and only valid options are passed 
//! into your functions. Climb will also generate help menus for your 
//! application and commands.
//! 
//! For usage examples, see the [Climb page on crates.io](https://crates.io/crates/climb).

mod app;
mod command;
mod help;
mod types;

pub use app::App;
pub use command::*;
pub use types::*;
