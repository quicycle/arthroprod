//! Constants and config options that are used to define the algebra.

use super::config::Allowed;
use super::types::{Index, Sign};
use std::collections::HashMap;

/// Each of the allowed Alpha indices in their string representations.
pub const ALPHAS: [&str; 16] = [
    "p",
    "0",
    "1",
    "2",
    "3",
    "01",
    "02",
    "03",
    "23",
    "31",
    "12",
    "023",
    "031",
    "012",
    "123",
    "0123",
];


lazy_static! {
    /// ALLOWED is a set of all allowed positive component values in the algebra.
    /// There are 16 in total: 1 scalar, 4 vectors, 6 bivectors, 4 trivectors
    /// and one quadrivector.
    pub static ref ALLOWED: Allowed = {
        let a = match Allowed::from_vec(ALPHAS.to_vec()) {
            Ok(v) => v,
            Err(e) => panic!(format!("{}", e)),
        };
        a
    };
}


lazy_static! {
    /// The METRIC determines which components square to -αp.
    pub static ref METRIC: HashMap<Index, Sign> = {
        let mut m = HashMap::new();
        m.insert(Index::Zero, Sign::Pos);
        m.insert(Index::One, Sign::Neg);
        m.insert(Index::Two, Sign::Neg);
        m.insert(Index::Three, Sign::Neg);
        m
    };
}
