use std::fmt;
use std::ops;

use crate::algebra::{Alpha, Xi};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash, Eq, Ord)]
pub struct Term {
    xi: Xi,
    alpha: Alpha,
}

impl Term {
    pub fn new(sym: String, alpha: Alpha) -> Term {
        let xi = Xi::new(1, sym);

        Term { xi, alpha }
    }

    pub fn from_alpha(alpha: Alpha) -> Term {
        let xi = Xi::new(1, format!("{}", alpha.component()));
        Term { xi, alpha }
    }

    pub fn xi(&self) -> Xi {
        self.xi.clone()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}

impl ops::Mul<isize> for Term {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Term {
            xi: self.xi * rhs,
            alpha: self.alpha,
        }
    }
}

impl ops::Mul<Term> for isize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Neg for Term {
    type Output = Term;

    fn neg(self) -> Self::Output {
        Term {
            xi: self.xi,
            alpha: -self.alpha,
        }
    }
}
