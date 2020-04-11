//! Algebraic operations on MultiVectors and other AR types.
//!
//! The operations in this module are derived from the Full Product of
//! the algebra, for differential based operations see the [`differentials`]
//! module.

mod ar;
mod ar_product;
mod diamond;
mod division;
mod dual;
mod full_product;
mod hermitian;
mod project;
mod reverse;

pub use self::{
    ar::AR,
    ar_product::{ar_product, invert_alpha},
    diamond::diamond,
    division::div,
    dual::{dual, mm_bar},
    full_product::full,
    hermitian::{dagger, hermitian},
    project::project,
    reverse::rev,
};
