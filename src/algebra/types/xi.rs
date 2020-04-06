use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Xi {
    weight: f64,
    symbol: String,
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weight == 1.0 {
            write!(f, "ξ{}", self.symbol)
        } else {
            write!(f, "({})ξ{}", self.weight, self.symbol)
        }
    }
}

impl Xi {
    pub fn new(n: f64, s: String) -> Xi {
        Xi {
            weight: n,
            symbol: s,
        }
    }

    pub fn new_f64(n: f64) -> Xi {
        Xi {
            weight: n,
            symbol: String::new(),
        }
    }

    pub fn new_symbolic(s: String) -> Xi {
        Xi {
            weight: 1.0,
            symbol: s,
        }
    }

    pub fn weight_and_symbol(&self) -> (f64, String) {
        (self.weight, self.symbol.clone())
    }
}
