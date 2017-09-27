use std::fmt;

use super::alpha::*;
use super::xi::*;
use Result;



#[derive(Debug, PartialOrd, PartialEq, Clone)]
/// A Pair is either a real or symbolic Xi value and an paired Alpha.
pub struct Pair {
    xi: Xi,
    alpha: Alpha,
}


impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}


impl Pair {
    /// Create a new Pair from a Xi and an Alpha.
    pub fn new(xi: Xi, alpha: Alpha) -> Pair {
        Pair { xi, alpha }
    }

    /// Create a default Symbolic Pair from an string Alpha index.
    pub fn sym(ix: &str) -> Result<Pair> {
        let alpha = Alpha::new(ix)?;
        // Remove any sign information before converting to a Xi
        let ix = ix.trim_matches('-');
        let xi = Xi::Symbolic(String::from(ix));
        Ok(Pair { xi, alpha })
    }

    /// The xi element of the Pair
    pub fn xi(&self) -> &Xi {
        &self.xi
    }

    /// The alpha element of the Pair
    pub fn alpha(&self) -> &Alpha {
        &self.alpha
    }
}
