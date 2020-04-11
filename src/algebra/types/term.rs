use std::fmt;
use std::ops;

use crate::algebra::{ar_product, invert_alpha, Alpha, Form, Magnitude, Sign, AR};

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
enum XiValue {
    Raw(String),
    Xi(Box<Vec<Xi>>),
}

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
struct Xi {
    value: XiValue,
    inverted: bool,
    partials: Vec<Form>,
}

impl Xi {
    fn new(value: String) -> Xi {
        Xi {
            value: XiValue::Raw(value),
            inverted: false,
            partials: Vec::new(),
        }
    }

    fn new_xi(value: String) -> Xi {
        let xi = Xi::new(value);
        Xi {
            value: XiValue::Xi(Box::new(vec![xi])),
            inverted: false,
            partials: Vec::new(),
        }
    }
}

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
pub struct Term {
    // positive definite scalar magnitude as a ratio
    magnitude: Magnitude,
    // The space-time form of the term (including directed sign)
    alpha: Alpha,
    // Partial derivatives taken for the term
    partials: Vec<Form>,
    // Xi product elements accumulated for the term
    xis: Vec<Xi>,
}

impl AR for Term {
    type Output = Self;

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
    pub fn new(val: Option<String>, alpha: Alpha) -> Term {
        let xi = if let Some(v) = val {
            Xi::new(v)
        } else {
            Xi::new(format!("{}", alpha.form()))
        };

        Term {
            magnitude: 1.into(),
            alpha: alpha,
            partials: vec![],
            xis: vec![xi],
        }
    }

    pub fn from_xis_and_alpha(xis: Vec<String>, alpha: Alpha) -> Term {
        Term {
            magnitude: 1.into(),
            alpha: alpha,
            partials: vec![],
            xis: xis.iter().map(|s| Xi::new_xi(s.clone())).collect(),
        }
    }

    pub fn form(&self) -> Form {
        self.alpha.form()
    }

    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }

    pub fn magnitude(&self) -> Magnitude {
        self.magnitude
    }

    pub fn inverted(&self) -> Term {
        let mut t = self.clone();
        t.xis = t
            .xis
            .iter()
            .map(|t| {
                let mut term = t.clone();
                term.inverted = !term.inverted;
                return term;
            })
            .collect();
        t.alpha = invert_alpha(&t.alpha);
        t.magnitude = 1 / t.magnitude;
        return t;
    }

    pub fn set_alpha(&mut self, a: Alpha) {
        self.alpha = a;
    }

    pub fn add_partial(&mut self, wrt: &Alpha) {
        self.partials.push(wrt.form());
        self.partials.sort();
    }

    pub fn set_partials(&mut self, partials: Vec<Form>) {
        self.partials = partials;
        self.partials.sort();
    }

    pub fn xi_str(&self) -> String {
        dotted_xi_str(&self.xis)
    }

    pub fn try_add(&self, other: &Term) -> Option<Term> {
        fn sub_mag(a: &Term, b: &Term) -> Term {
            // For subtraction we need to make sure that magnitude stays positive
            // so we flip the sign of the alpha if needed and make use of the fact
            // that A - B == -(B - A)
            let mut t = a.clone();
            if t.magnitude > b.magnitude {
                t.magnitude -= b.magnitude;
            } else {
                t.magnitude = b.magnitude - t.magnitude;
                t.alpha = -t.alpha;
            }
            return t;
        }

        if self.summation_key() == other.summation_key() {
            Some(match (self.alpha.sign(), other.alpha.sign()) {
                (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => {
                    let mut t = self.clone();
                    t.magnitude += other.magnitude;
                    t
                }
                (Sign::Pos, Sign::Neg) => sub_mag(self, other), // sub other from self
                (Sign::Neg, Sign::Pos) => sub_mag(other, self), // sub self from other
            })
        } else {
            None
        }
    }

    pub fn form_product_with(&self, other: &Term) -> Term {
        let mut xis = vec![self.extract_xi(), other.extract_xi()];
        xis.sort();

        Term {
            magnitude: self.magnitude * other.magnitude,
            alpha: ar_product(&self.alpha.clone(), &other.alpha.clone()),
            partials: Vec::new(),
            xis: xis,
        }
    }

    fn extract_xi(&self) -> Xi {
        Xi {
            value: XiValue::Xi(Box::new(self.xis.clone())),
            partials: self.partials.clone(),
            inverted: false,
        }
    }

    pub fn summation_key(&self) -> (Form, String) {
        (self.form(), self.xi_str())
    }
}

// NOTE: Arithmetic operation impls

impl ops::Mul<usize> for Term {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut t = self.clone();
        t.magnitude = t.magnitude * rhs;
        return t;
    }
}

impl ops::Mul<isize> for Term {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let mut t = self.clone();
        if rhs < 0 {
            t.magnitude = t.magnitude * (-rhs) as usize;
            t.alpha = -t.alpha;
        } else {
            t.magnitude = t.magnitude * rhs as usize;
        }

        return t;
    }
}

impl ops::Mul<Magnitude> for Term {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        let mut t = self.clone();
        t.magnitude = t.magnitude * rhs;
        return t;
    }
}

impl ops::Div<Magnitude> for Term {
    type Output = Term;

    fn div(self, rhs: Magnitude) -> Self::Output {
        let mut t = self.clone();
        t.magnitude = t.magnitude / rhs;
        return t;
    }
}

impl ops::Neg for Term {
    type Output = Term;

    fn neg(self) -> Self::Output {
        let mut t = self.clone();
        t.alpha = -t.alpha;
        return t;
    }
}

// NOTE: flipped variants for primary impls above

impl ops::Mul<Term> for usize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Term> for isize {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Term> for Magnitude {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        rhs * self
    }
}

// NOTE: fmt::Display impls and helper functions

fn partial_str(partials: &Vec<Form>) -> String {
    partials
        .iter()
        .fold(String::new(), |acc, p| acc + &format!("∂{}", p))
}

fn dotted_xi_str(xis: &Vec<Xi>) -> String {
    match xis.len() {
        1 => format!("{}", xis[0]),
        _ => xis[1..xis.len()]
            .iter()
            .fold(format!("{}", xis[0]), |acc, x| format!("{}.{}", acc, x)),
    }
}

impl fmt::Display for XiValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XiValue::Raw(s) => write!(f, "ξ{}", s),
            XiValue::Xi(x) => write!(f, "{}", dotted_xi_str(x)),
        }
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", partial_str(&self.partials), self.value)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m_str = if self.magnitude != 1 {
            format!("({})", self.magnitude)
        } else {
            String::new()
        };
        let p_str = partial_str(&self.partials);

        write!(f, "{}{}{}({})", self.alpha, m_str, p_str, self.xi_str())
    }
}
