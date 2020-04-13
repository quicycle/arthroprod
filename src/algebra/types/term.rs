use std::fmt;
use std::ops;

use super::xi::partial_str;
use crate::algebra::{ar_product, Alpha, Form, Magnitude, Sign, Xi, AR};

/// A Term represents a real scalar magnitude along with a paired [`Alpha`] giving the
/// proper Space-Time [`Form`] in accordence with the principle of Absolute Relativity.
#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
pub struct Term {
    magnitude: Magnitude,
    alpha: Alpha,
    partials: Vec<Form>,
    numerator: Xi,
    denominator: Xi,
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

    fn inverse(&self) -> Self::Output {
        Term {
            magnitude: 1 / self.magnitude,
            alpha: self.alpha.inverse(),
            partials: self.partials.clone(),
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
}

impl Term {
    /// Construct a new Term. The underlying symbolic value will be constructed
    /// from the Form of alpha if None is provided.
    pub fn new(val: Option<&str>, alpha: Alpha) -> Term {
        let xi = if let Some(v) = val {
            Xi::new(v)
        } else {
            Xi::new(&format!("{}", alpha.form()))
        };

        Term {
            magnitude: 1.into(),
            alpha: alpha,
            partials: vec![],
            numerator: xi,
            denominator: Xi::empty(),
        }
    }

    /// Construct a Term with compoud Xi values as opposed to raw symbols
    pub fn from_xis_and_alpha(xis: Vec<&str>, alpha: Alpha) -> Term {
        Term {
            magnitude: 1.into(),
            alpha: alpha,
            partials: vec![],
            numerator: Xi::merge(&xis.iter().map(|s| Xi::new(s)).collect()),
            denominator: Xi::empty(),
        }
    }

    /// Extract a copy of the Space-Time [`Form`] of this term
    pub fn form(&self) -> Form {
        self.alpha.form()
    }

    /// Extract a copy of the [`Alpha`] of this Term
    pub fn alpha(&self) -> Alpha {
        self.alpha.clone()
    }

    /// Extract the unsigned [`Magnitude`] of this Term
    pub fn magnitude(&self) -> Magnitude {
        self.magnitude
    }

    /// Override the Alpha value of this Term
    pub fn set_alpha(&mut self, a: Alpha) {
        self.alpha = a;
    }

    /// Add a single partial derivative and resort
    pub fn add_partial(&mut self, wrt: &Alpha) {
        self.partials.push(wrt.form());
        self.partials.sort();
    }

    /// Replace the current set of partial derivatives
    pub fn set_partials(&mut self, partials: Vec<Form>) {
        self.partials = partials;
        self.partials.sort();
    }

    /// Generate a string representation of the underlying Xi values for this term
    pub fn xi_str(&self) -> String {
        let numerator = if self.numerator.is_empty() {
            "1".to_string()
        } else {
            self.numerator.dotted_string()
        };

        if self.denominator.is_empty() {
            format!("{}", numerator)
        } else {
            format!("{}/{}", numerator, self.denominator.dotted_string())
        }
    }

    /// Attempt to add two Terms. This will only succeed if their summation_key
    /// of both Terms is the same. We use this as a method rather than implimenting
    /// ops::Add for Terms as we are not guaranteed to be able to return a result.
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

    /// Form the product of this term and another under the full product of the algebra
    pub fn form_product_with(&self, other: &Term) -> Term {
        Term {
            magnitude: self.magnitude * other.magnitude,
            alpha: ar_product(&self.alpha, &other.alpha),
            partials: Vec::new(),
            numerator: Xi::merge(&vec![self.numerator.clone(), other.numerator.clone()]),
            denominator: Xi::merge(&vec![self.denominator.clone(), other.denominator.clone()]),
        }
    }

    /// The elements of a Term that need to match for us to be able to sum them
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

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;
    use test_case::test_case;

    #[test_case(Term::new(None, alpha!(0 1)), Term::new(None, alpha!(0 1)))]
    #[test_case(Term::new(None, alpha!(2 3)), Term::new(None, -alpha!(2 3)))]
    #[test_case(term!(0 3 1), -term!(0 3 1))]
    #[test_case(term!("foo", 1 2 3), term!("foo", 1 2 3))]
    #[test_case(term!(["foo", "bar"], 0 1 2 3), -term!(["foo", "bar"], 0 1 2 3))]
    fn inversion_works(t: Term, u: Term) {
        let mut expected = u.clone();
        expected.numerator = u.denominator;
        expected.denominator = u.numerator;
        assert_eq!(t.inverse(), expected);
    }

    // TODO: This currently "works". Should it?
    // #[test_case(term!("foo", 0 2), term!(["foo"], 0 2), false)]
    #[test_case(term!("foo", 1 2 3), term!("foo", 1 2 3), true)]
    #[test_case(term!("foo", 1), -term!("foo", 1), true)]
    #[test_case(term!("foo", 1), term!("foo", 2), false)]
    #[test_case(term!("foo", 0 2), term!("bar", 0 2), false)]
    fn summation_key_correctly_identifies_terms(t: Term, u: Term, expected: bool) {
        assert_eq!(t.summation_key() == u.summation_key(), expected);
    }

    #[test_case(term!(1 2 3), term!(1 2 3), 2 as usize * term!(1 2 3))]
    #[test_case(term!(3 1), 2 as usize * -term!(3 1), -1 as isize * term!(3 1))]
    fn try_add_correctly_sums_terms(t: Term, u: Term, expected: Term) {
        assert_eq!(t.try_add(&u).unwrap(), expected);
    }

    #[test_case(term!("a", 2 3), term!("b", 1 2 3), -term!(["a", "b"], 1))]
    #[test_case(term!("a", 2 3), term!("a", 2 3), -term!(["a", "a"], ))]
    fn form_product_with_works_with_no_inversion(left: Term, right: Term, expected: Term) {
        assert_eq!(left.form_product_with(&right), expected)
    }

    // #[test]
    // fn form_product_with_works_inversion() {
    //     let left = term!("a", 2 3);
    //     let right = term!("b", 0 2).inverted();
    //     let mut expected = term!(["a", "b"], 0 3);
    //     if let XiValue::Xi(mut xis) = &expected.xis[1].value {
    //         xis[0].inverted = true;
    //     }

    //     assert_eq!(left.form_product_with(&right), expected)
    // }
}
