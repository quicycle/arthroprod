use std::fmt;
use std::ops;

use crate::algebra::{Alpha, Ratio, Xi, AR};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash, Eq, Ord)]
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
        let mut t = Term {
            xi: Xi::new(sym),
            alpha: alpha,
        };
        t.ensure_sign_on_alpha();

        return t;
    }

    pub fn new_with_xi(xi: Xi, alpha: Alpha) -> Term {
        let mut t = Term { xi, alpha };
        t.ensure_sign_on_alpha();

        return t;
    }

    pub fn from_alpha(alpha: Alpha) -> Term {
        let xi = Xi::new(format!("{}", alpha.component()));
        Term { xi, alpha }
    }

    pub fn xi(&self) -> Xi {
        self.xi.clone()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }

    fn ensure_sign_on_alpha(&mut self) {
        let (weight, _) = self.xi.clone().into();
        if weight < 0.into() {
            self.xi = -self.xi.clone();
            self.alpha = -self.alpha;
        }
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
        Term::new_with_xi(self.xi * rhs, self.alpha)
    }
}

impl ops::Mul<Term> for isize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Ratio> for Term {
    type Output = Self;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Term::new_with_xi(self.xi * rhs, self.alpha)
    }
}

impl ops::Mul<Term> for Ratio {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Ratio> for Term {
    type Output = Term;

    fn div(self, rhs: Ratio) -> Self::Output {
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
