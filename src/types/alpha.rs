use std::collections::HashSet;
use std::fmt;

use super::component::*;
use super::index::*;
use super::sign::*;
use super::super::consts::ALLOWED;
use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
/// The base element for computation with Absolute Relativity.
///
/// An Alpha is a Component along with an associated Sign. All values and
/// mathematical operators within the algebra are required to be paired with
/// their correct Alpha value under the principle of Absolute Relativity.
pub struct Alpha {
    index: Component,
    sign: Sign,
}


impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Pos => write!(f, "α{}", self.index),
            Sign::Neg => write!(f, "-α{}", self.index),
        }
    }
}


impl Alpha {
    /// create a new alpha from an string index containing an optional sign
    /// prefix.
    ///
    /// NOTE: This will panic if the index is invalid in order to prevent the
    /// user from running inconsistant calculations.
    pub fn new(ix: &str) -> Result<Alpha> {
        let sign = match ix.starts_with("-") {
            true => Sign::Neg,
            false => Sign::Pos,
        };

        let ix = ix.trim_matches('-');

        let index = Component::new(ix, &ALLOWED.indices())?;
        Ok(Alpha { index, sign })
    }

    /// new_override allows the caller to explicitly specify an index, sign and
    /// allowed set of alphas when creating an alpha.
    pub fn new_override(ix: &str, sign: Sign, allowed: &HashSet<Component>) -> Result<Alpha> {
        let index = Component::new(ix, allowed)?;
        Ok(Alpha { index, sign })
    }

    /// Construct an Alpha explicitly from a Component and a Sign.
    pub fn from_index(index: &Component, sign: &Sign) -> Alpha {
        Alpha {
            index: index.clone(),
            sign: sign.clone(),
        }
    }

    /// Check to see if an alpha is +/-αp
    pub fn is_point(&self) -> bool {
        self.index == Component::Point
    }

    /// Return a copy of this Alpha's index
    pub fn index(&self) -> &Component {
        &self.index
    }

    /// Return a copy of this Alpha's sign
    pub fn sign(&self) -> &Sign {
        &self.sign
    }

    /// Return a Vector of Indices representing this Alpha's Indices
    pub fn as_vec(&self) -> Vec<Index> {
        self.index.as_vec()
    }
}
