use std::{fmt, ops::Neg};

use crate::algebra::{Alpha, Xi};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Term {
    xi: Xi,
    alpha: Alpha,
}

impl Neg for Term {
    type Output = Term;

    fn neg(self) -> Self::Output {
        Term {
            xi: self.xi,
            alpha: -self.alpha,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}

impl Term {
    pub fn new(val: f64, sym: String, alpha: Alpha) -> Term {
        let xi = Xi::new(val, sym);

        Term { xi, alpha }
    }

    pub fn new_f64(val: f64, alpha: Alpha) -> Term {
        let xi = Xi::new_f64(val);

        Term { xi, alpha }
    }

    pub fn new_sym(sym: String, alpha: Alpha) -> Term {
        let xi = Xi::new_symbolic(sym);

        Term { xi, alpha }
    }

    pub fn from_alpha(alpha: Alpha) -> Term {
        let xi = Xi::new_symbolic(format!("{}", alpha.component()));
        Term { xi, alpha }
    }

    pub fn xi(&self) -> Xi {
        self.xi.clone()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }
}
