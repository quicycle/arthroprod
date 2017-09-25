#[macro_use]
extern crate lazy_static;

pub mod consts;
pub mod types;
pub mod utils;
pub mod ops;
mod error;

pub use error::ArError;

/// Result type to consitant error messages.
pub type Result<T> = std::result::Result<T, ArError>;
