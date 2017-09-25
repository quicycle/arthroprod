//! Constants and config options that are used to define the algebra.

use super::types::{Component, Index, KeyVec, Sign};
use std::collections::{HashMap, HashSet};


lazy_static! {
    /// ALLOWED is a set of all allowed positive component values in the algebra.
    /// There are 16 in total: 1 scalar, 4 vectors, 6 bivectors, 4 trivectors
    /// and one quadrivector.
    pub static ref ALLOWED: HashSet<Component> = {
    let mut s = HashSet::new();
    s.insert(Component::Point);
    s.insert(Component::Vector(Index::Zero));
    s.insert(Component::Vector(Index::One));
    s.insert(Component::Vector(Index::Two));
    s.insert(Component::Vector(Index::Three));
    s.insert(Component::Bivector(Index::One, Index::Zero));
    s.insert(Component::Bivector(Index::Two, Index::Zero));
    s.insert(Component::Bivector(Index::Three, Index::Zero));
    s.insert(Component::Bivector(Index::Two, Index::Three));
    s.insert(Component::Bivector(Index::Three, Index::One));
    s.insert(Component::Bivector(Index::One, Index::Two));
    s.insert(Component::Trivector(Index::Zero, Index::Two, Index::Three));
    s.insert(Component::Trivector(Index::Zero, Index::Three, Index::One));
    s.insert(Component::Trivector(Index::Zero, Index::One, Index::Two));
    s.insert(Component::Trivector(Index::One, Index::Two, Index::Three));
    s.insert(Component::Quadrivector(
        Index::Zero,
        Index::One,
        Index::Two,
        Index::Three,
    ));
    s
    };
}


lazy_static! {
    /// TARGETS allows for finding the correct element from ALLOWED that should
    /// be returned from a given valid component with indices in an arbitrary order.
    pub static ref TARGETS: HashMap<KeyVec, Component> = {
        let mut s = HashMap::new();
        for a in ALLOWED.iter() {
            if *a != Component::Point {
                s.insert(KeyVec::new(a.to_vec()), a.clone());
            }
        }
        s
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
