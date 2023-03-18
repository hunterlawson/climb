#[path ="command/mod.rs"]
#[doc(hidden)]
pub mod __command;

pub mod app;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::default_app;
}