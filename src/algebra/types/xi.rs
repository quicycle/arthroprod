use std::convert;
use std::fmt;
use std::ops;

use crate::algebra::Ratio;

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord)]
pub struct Xi {
    weight: Ratio,
    symbol: String,
}

impl Xi {
    pub fn new(s: String) -> Xi {
        Xi {
            weight: Ratio::from(1),
            symbol: s,
        }
    }

    pub fn new_weighted(r: Ratio, s: String) -> Xi {
        Xi {
            weight: r,
            symbol: s,
        }
    }

    pub fn weight_and_symbol(&self) -> (Ratio, String) {
        (self.weight, self.symbol.clone())
    }
}

impl convert::From<(Ratio, String)> for Xi {
    fn from(pair: (Ratio, String)) -> Self {
        Xi::new_weighted(pair.0, pair.1)
    }
}

impl convert::Into<(Ratio, String)> for Xi {
    fn into(self) -> (Ratio, String) {
        (self.weight, self.symbol)
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

impl ops::Mul<Ratio> for Xi {
    type Output = Self;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Xi {
            weight: self.weight * rhs,
            symbol: self.symbol,
        }
    }
}

impl ops::Mul<Xi> for Ratio {
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
