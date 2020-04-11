use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::algebra::{Form, Magnitude, Term, ALLOWED_ALPHA_FORMS, AR};

/// A MultiVector is an unordered collection of a Terms representing a particular
/// composite quantity within the Algebra. In its simplest form, a MultiVector is
/// a simple linear sum of Alphas, though it is possible for there to be significantly
/// more structure.
///
/// In practice, almost all arpy computations are done using MultiVectors as their
/// primary data structure so there are a number of methods designed for aiding in
/// simplifying such computations.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MultiVector {
    terms: Vec<Term>,
}

impl AR for MultiVector {
    type Output = Self;

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
        self.terms.sort();
    }

    pub fn add_terms(&mut self, terms: Vec<Term>) {
        self.terms.extend(terms);
        self.terms.sort();
    }

    pub fn get(&self, c: &Form) -> Vec<Term> {
        self.terms
            .iter()
            .filter(|t| &t.form() == c)
            .map(|t| t.clone())
            .collect()
    }

    /// Combine together term weights with matching Forms and Xis
    pub fn simplify(&mut self) {
        let mut groups: HashMap<(Form, String), Vec<Term>> = HashMap::new();

        self.terms.iter().cloned().for_each(|t| {
            groups.entry(t.summation_key()).or_insert(vec![]).push(t);
        });

        // Now that we are grouped by summation_key we are safe to unwrap the
        // try_combine call without blowing up
        // TODO: cancelling terms with zero magnitude still needs some thought
        // .filter(|t| t.magnitude() != 0)
        self.terms = groups
            .drain()
            .map(|(_, v)| {
                v[1..v.len()]
                    .iter()
                    .fold(v[0].clone(), |acc, t| acc.try_add(t).unwrap())
            })
            .collect()
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

impl ops::Mul<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        MultiVector::from_terms(self.terms.iter().map(|t| t.clone() * rhs).collect())
    }
}

impl ops::Mul<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn mul(self, rhs: MultiVector) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn div(self, rhs: Magnitude) -> Self::Output {
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
        let s = ALLOWED_ALPHA_FORMS
            .iter()
            .map(|c| {
                let for_comp = self.get(c);
                if for_comp.len() > 0 {
                    Some(format!("{}),", {
                        let mut vec_str = for_comp
                            .iter()
                            .fold(String::from(format!("  a{}: (", c)), |acc, val| {
                                format!("{}{}, ", acc, val.xi_str())
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
