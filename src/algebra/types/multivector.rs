use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::algebra::{Component, Ratio, Term, ALLOWED_ALPHA_COMPONENTS, AR};

/// A MultiVector is an unordered collection of a Terms representing a particular
/// composite quantity within the Algebra. In its simplest form, a MultiVector is
/// a simple linear sum of Alphas, though it is possible for there to be significantly
/// more structure.
///
/// In practice, almost all arpy computations are done using MultiVectors as their
/// primary data structure so there are a number of methods designed for aiding in
/// simplifying such computations.
#[derive(Debug, PartialEq, Clone)]
pub struct MultiVector {
    terms: Vec<Term>,
}

impl AR for MultiVector {
    fn as_terms(&self) -> Vec<Term> {
        self.terms.clone()
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        MultiVector { terms }
    }
}

impl MultiVector {
    pub fn new() -> MultiVector {
        MultiVector { terms: vec![] }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    pub fn get(&self, c: &Component) -> Vec<Term> {
        self.terms
            .iter()
            .filter(|t| &t.alpha().component() == c)
            .map(|t| t.clone())
            .collect()
    }

    pub fn cancel_terms(&mut self) {
        let mut seen: HashMap<Term, Vec<Term>> = HashMap::new();

        self.terms.sort();

        self.terms.iter().for_each(|t| {
            let neg = -(t.clone());
            if let Some(current) = seen.get_mut(&neg) {
                if current.len() == 1 {
                    seen.remove(&t);
                }
            } else {
                if let Some(current) = seen.get_mut(&t) {
                    current.push(t.clone());
                } else {
                    seen.insert(t.clone(), vec![t.clone()]);
                }
            };
        });

        self.terms = seen.drain().map(|(_, v)| v).flatten().collect()
    }
}

impl ops::Mul<isize> for MultiVector {
    type Output = MultiVector;

    fn mul(self, rhs: isize) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| t.clone() * rhs).collect())
    }
}

impl ops::Mul<MultiVector> for isize {
    type Output = MultiVector;

    fn mul(self, rhs: MultiVector) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Ratio> for MultiVector {
    type Output = MultiVector;

    fn mul(self, rhs: Ratio) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| t.clone() * rhs).collect())
    }
}

impl ops::Mul<MultiVector> for Ratio {
    type Output = MultiVector;

    fn mul(self, rhs: MultiVector) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Ratio> for MultiVector {
    type Output = MultiVector;

    fn div(self, rhs: Ratio) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| t.clone() / rhs).collect())
    }
}

impl ops::Add for MultiVector {
    type Output = MultiVector;

    fn add(self, rhs: MultiVector) -> Self::Output {
        let mut terms = self.terms.clone();
        let mut rhs_terms = rhs.terms.clone();
        terms.append(&mut rhs_terms);

        MultiVector::from_terms(terms)
    }
}

impl ops::Add<Term> for MultiVector {
    type Output = MultiVector;

    fn add(self, rhs: Term) -> Self::Output {
        let mut terms = self.terms.clone();
        terms.push(rhs);

        MultiVector::from_terms(terms)
    }
}

impl ops::Sub for MultiVector {
    type Output = MultiVector;

    fn sub(self, rhs: MultiVector) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| -t.clone()).collect())
    }
}

impl fmt::Display for MultiVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = ALLOWED_ALPHA_COMPONENTS
            .iter()
            .map(|c| {
                let for_comp = self.get(c);
                if for_comp.len() > 0 {
                    Some(format!("{}),", {
                        let mut vec_str = for_comp
                            .iter()
                            .fold(String::from(format!("  a{}: (", c)), |s, val| {
                                format!("{}{}, ", s, val.xi())
                            });
                        let desired_len = vec_str.len() - 2;
                        vec_str.split_off(desired_len);
                        vec_str
                    }))
                } else {
                    None
                }
            })
            .filter_map(|s| s)
            .fold(String::from(""), |s, line| format!("{}\n{}", s, line));

        write!(f, "{{{}\n}}", s)
    }
}
