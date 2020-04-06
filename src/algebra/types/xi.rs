use std::fmt;
use std::ops;

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord)]
pub struct Xi {
    weight: isize,
    symbol: String,
}

impl Xi {
    pub fn new(n: isize, s: String) -> Xi {
        Xi {
            weight: n,
            symbol: s,
        }
    }

    pub fn weight_and_symbol(&self) -> (isize, String) {
        (self.weight, self.symbol.clone())
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weight == 1 {
            write!(f, "ξ{}", self.symbol)
        } else {
            write!(f, "({})ξ{}", self.weight, self.symbol)
        }
    }
}

impl ops::Mul<isize> for Xi {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Xi {
            weight: self.weight * rhs,
            symbol: self.symbol,
        }
    }
}

impl ops::Mul<Xi> for isize {
    type Output = Xi;

    fn mul(self, rhs: Xi) -> Self::Output {
        rhs * self
    }
}

impl ops::Neg for Xi {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Xi {
            weight: -self.weight,
            symbol: self.symbol,
        }
    }
}
