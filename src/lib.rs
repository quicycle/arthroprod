//! arthroprod is a Rust re-implementation of the Python framework
//! [arpy](https://github.com/sminez/arpy).  It is intended to provide better
//! confidence in the correctness of the algorithms and also provide a speed up
//! in computation for when we need to iterate on large calculations.
//!
//! (arpy is a module for performing calculations within the theory of Absolute
//! Relativity as devised by [Dr J.G.Williamson](http://www.gla.ac.
//! uk/schools/engineering/staff/johnwilliamson/).)
//!
//! The primary project can be found [here](https://github.com/sminez/arpy)
//! and this may become a Python extension module in the future.
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate proptest;


pub mod algebra;
pub mod calcfile;
pub mod config;
pub mod consts;
pub mod types;
mod error;

pub use error::ArError;

/// Result type to provide consitant error messages.
pub type Result<T> = std::result::Result<T, ArError>;
