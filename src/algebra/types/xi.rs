use std::convert;
use std::fmt;
use std::ops;

use crate::algebra::Magnitude;

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
pub struct Xi {
    magnitude: Magnitude,
    symbol: String,
}

impl Xi {
    pub fn new(s: String) -> Xi {
        Xi {
            magnitude: Magnitude::from(1),
            symbol: s,
        }
    }

    pub fn new_weighted(r: Magnitude, s: String) -> Xi {
        Xi {
            magnitude: r,
            symbol: s,
        }
    }
}

impl convert::From<(Magnitude, String)> for Xi {
    fn from(pair: (Magnitude, String)) -> Self {
        Xi::new_weighted(pair.0, pair.1)
    }
}

impl convert::Into<(Magnitude, String)> for Xi {
    fn into(self) -> (Magnitude, String) {
        (self.magnitude, self.symbol)
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.magnitude == 1 {
            write!(f, "ξ{}", self.symbol)
        } else {
            write!(f, "({})ξ{}", self.magnitude, self.symbol)
        }
    }
}

impl ops::Mul<usize> for Xi {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Xi {
            magnitude: self.magnitude * rhs,
            symbol: self.symbol,
        }
    }
}

impl ops::Mul<Xi> for usize {
    type Output = Xi;

    fn mul(self, rhs: Xi) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Magnitude> for Xi {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Xi {
            magnitude: self.magnitude * rhs,
            symbol: self.symbol,
        }
    }
}

impl ops::Mul<Xi> for Magnitude {
    type Output = Xi;

    fn mul(self, rhs: Xi) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Magnitude> for Xi {
    type Output = Xi;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Xi {
            magnitude: self.magnitude / rhs,
            symbol: self.symbol,
        }
    }
}
