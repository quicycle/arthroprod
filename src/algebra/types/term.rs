use std::fmt;
use std::ops;

use crate::algebra::{Alpha, Magnitude, Xi, AR};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash, Eq, Ord, Serialize, Deserialize)]
pub struct Term {
    xi: Xi,
    alpha: Alpha,
}

impl AR for Term {
    fn as_terms(&self) -> Vec<Term> {
        vec![self.clone()]
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        if terms.len() != 1 {
            panic!("Can only construct an Term from a single term")
        };

        terms[0].clone()
    }
}

impl Term {
    pub fn new(sym: String, alpha: Alpha) -> Term {
        Term {
            xi: Xi::new(sym),
            alpha: alpha,
        }
    }

    pub fn new_with_xi(xi: Xi, alpha: Alpha) -> Term {
        Term { xi, alpha }
    }

    pub fn from_alpha(alpha: Alpha) -> Term {
        Term {
            xi: Xi::new(format!("{}", alpha.component())),
            alpha: alpha,
        }
    }

    pub fn xi(&self) -> Xi {
        self.xi.clone()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }

    pub fn magnitude(&self) -> Magnitude {
        let (mag, _) = self.xi.clone().into();

        mag
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}

impl ops::Mul<usize> for Term {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Term::new_with_xi(self.xi * rhs, self.alpha)
    }
}

impl ops::Mul<Term> for usize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<isize> for Term {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            Term::new_with_xi(self.xi * (-rhs) as usize, -self.alpha)
        } else {
            Term::new_with_xi(self.xi * rhs as usize, self.alpha)
        }
    }
}

impl ops::Mul<Term> for isize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Magnitude> for Term {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Term::new_with_xi(self.xi * rhs, self.alpha)
    }
}

impl ops::Mul<Term> for Magnitude {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Magnitude> for Term {
    type Output = Term;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Term::new_with_xi(self.xi / rhs, self.alpha)
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
