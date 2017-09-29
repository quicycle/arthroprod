use std::collections::HashSet;
use std::fmt;
use std::ops;

use super::component::*;
use super::index::*;
use super::mvec::*;
use super::pair::*;
use super::sign::*;
use super::super::consts::{ALLOWED, METRIC};
use super::super::ops::{ArOps, find_prod_override};
use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
/// The base element for computation with Absolute Relativity.
///
/// An Alpha is a Component along with an associated Sign. All values and
/// mathematical operators within the algebra are required to be paired with
/// their correct Alpha value under the principle of Absolute Relativity.
pub struct Alpha {
    comp: Component,
    sign: Sign,
}


impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Pos => write!(f, "α{}", self.comp),
            Sign::Neg => write!(f, "-α{}", self.comp),
        }
    }
}

impl ops::Mul for Alpha {
    type Output = Alpha;

    fn mul(self, rhs: Self) -> Self {
        find_prod_override(&self, &rhs, &METRIC, &ALLOWED)
    }
}


impl ArOps<Alpha> for Alpha {
    fn ar_prod(&self, _rhs: &Alpha) -> Mvec {
        let alpha = find_prod_override(&self, _rhs, &METRIC, &ALLOWED);
        let mut mvec = Mvec::new();
        mvec.add_pair(Pair::from_alpha(alpha)).unwrap();
        mvec
    }

    fn ar_div_into(&self, _rhs: &Alpha) -> Mvec {
        panic!("Not implemented yet");
        // Mvec::new()
    }

    fn ar_div_by(&self, _rhs: &Alpha) -> Mvec {
        panic!("Not implemented yet");
        // Mvec::new()
    }

    fn ar_add(&self, _rhs: &Alpha) -> Mvec {
        let mut mvec = Mvec::new();
        mvec.add_pair(Pair::from_alpha(self.clone())).unwrap();
        mvec.add_pair(Pair::from_alpha(_rhs.clone())).unwrap();
        mvec
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

        let comp = Component::new(ix)?;
        Ok(Alpha { comp, sign })
    }

    /// new_override allows the caller to explicitly specify an index, sign and
    /// allowed set of alphas when creating an alpha.
    pub fn new_override(ix: &str, sign: Sign, allowed: &HashSet<Component>) -> Result<Alpha> {
        let comp = Component::new_override(ix, allowed)?;
        Ok(Alpha { comp, sign })
    }

    /// Construct an Alpha explicitly from a Component and a Sign.
    pub fn from_comp(comp: &Component, sign: &Sign) -> Alpha {
        Alpha {
            comp: comp.clone(),
            sign: sign.clone(),
        }
    }

    /// Check to see if an alpha is +/-αp
    pub fn is_point(&self) -> bool {
        self.comp == Component::Point
    }

    /// Return a copy of this Alpha's index
    pub fn comp(&self) -> &Component {
        &self.comp
    }

    /// Return a copy of this Alpha's sign
    pub fn sign(&self) -> &Sign {
        &self.sign
    }

    /// Return a Vector of Indices representing this Alpha's Indices
    pub fn as_vec(&self) -> Vec<Index> {
        self.comp.as_vec()
    }
}
