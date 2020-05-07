//! Algebraic operations on MultiVectors and other AR types.
//!
//! The operations in this module are derived from the Full Product of
//! the algebra, for differential based operations see the [`differentials`]
//! module.

mod ar;
mod ar_product;
mod division;
mod full_product;

pub use self::{ar::AR, ar_product::ar_product, division::div, full_product::full};
