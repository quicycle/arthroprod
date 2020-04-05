use std::fmt;

use crate::algebra::{Alpha, Xi};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Term {
    xi: Xi,
    alpha: Alpha,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}

impl Term {
    pub fn new_f64(val: f64, alpha: Alpha) -> Term {
        let xi = Xi::Real(val);

        Term { xi, alpha }
    }

    pub fn new_sym(val: String, alpha: Alpha) -> Term {
        let xi = Xi::Symbolic(val);

        Term { xi, alpha }
    }

    pub fn from_alpha(alpha: Alpha) -> Term {
        let xi = Xi::Symbolic(format!("{}", alpha.component()));
        Term { xi, alpha }
    }

    pub fn xi(&self) -> Xi {
        self.xi.clone()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }
}
