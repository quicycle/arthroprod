//! Base types for carrying out calculations within the Absolute Relativity
//! framework.

use std::hash::{Hash, Hasher};

mod alpha;
mod component;
mod index;
mod pair;
mod sign;
mod xi;

pub use self::alpha::*;
pub use self::component::*;
pub use self::index::*;
pub use self::pair::*;
pub use self::sign::*;
pub use self::xi::*;


#[derive(Eq, PartialEq, Debug, Clone)]
/// A vector that is hashed based on it's sorted order.
pub struct KeyVec(Vec<Index>);

impl KeyVec {
    /// Generate a new KeyVec from a vector of indices.
    ///
    /// The elements are sorted when the KeyVec is created.
    pub fn new(v: Vec<Index>) -> KeyVec {
        let mut v = v.clone();
        v.sort();
        KeyVec(v)
    }
}

impl Hash for KeyVec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let KeyVec(ref elems) = *self;
        for elem in elems.iter() {
            elem.hash(state);
        }
    }
}
