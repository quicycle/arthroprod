use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::algebra::{Form, Magnitude, Term, ALLOWED_ALPHA_FORMS, AR};

/// A MultiVector is an ordered collection of a Terms representing a particular
/// composite quantity within the Algebra. In its simplest form, a MultiVector is
/// a simple linear sum of Alphas, though it is possible for there to be significantly
/// more structure.
///
/// In practice, almost all  computations are done using MultiVectors as their
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
        let mut m = MultiVector { terms };
        m.terms.sort();
        return m;
    }
}

impl MultiVector {
    /// Construct a new, empty MultiVector
    pub fn new() -> MultiVector {
        MultiVector { terms: vec![] }
    }

    /// Returns an iterator over terms contained in this MultiVector
    pub fn iter(&self) -> MvecIterator {
        MvecIterator {
            ix: 0,
            terms: &self.terms,
        }
    }

    /// Push a new term into this MultiVector and reorder the terms if needed.
    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
        self.terms.sort();
    }

    /// Extend this MultiVector by converting other to a Vector of [`Term`]s
    /// and then reorder if needed.
    pub fn extend<T: AR>(&mut self, other: T) {
        self.terms.extend(other.as_terms());
        self.terms.sort();
    }

    /// Extract a copy of the terms in this MultiVector that have the supplied [`Form`]
    pub fn get(&self, c: &Form) -> Option<Vec<Term>> {
        let mut terms: Vec<Term> = self
            .terms
            .iter()
            .filter(|t| &t.form() == c)
            .map(|t| t.clone())
            .collect();
        terms.sort();

        return if terms.len() > 0 { Some(terms) } else { None };
    }

    /// Combine together term weights where they have matching Form and Xi
    pub fn simplify(&mut self) {
        let mut groups: HashMap<(Form, String), Vec<Term>> = HashMap::new();

        self.terms.iter().cloned().for_each(|t| {
            groups.entry(t.summation_key()).or_insert(vec![]).push(t);
        });

        // Now that we are grouped by summation_key we are safe to unwrap the
        // try_combine call without blowing up
        // TODO: cancelling terms with zero magnitude still needs some thought
        //       John is pretty sure we need some additional checks before it is
        //       safe to drop terms.
        let mut terms: Vec<Term> = groups
            .drain()
            .map(|(_, v)| match v.len() {
                1 => v[0].clone(),
                _ => v[1..v.len()]
                    .iter()
                    .fold(v[0].clone(), |acc, t| acc.try_add(t).unwrap()),
            })
            .filter(|t| t.magnitude() != 0)
            .collect();

        terms.sort();
        self.terms = terms;
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
        let mut rows = vec![];
        let n_per_line = 6;

        for form in ALLOWED_ALPHA_FORMS.iter() {
            if let Some(terms) = self.get(form) {
                let form_rows = terms
                    .iter()
                    .map(|term| format!("{}{}", term.sign(), term.xi_str()))
                    .collect::<Vec<String>>()
                    .chunks(n_per_line)
                    .map(|c| c.join(", "))
                    .collect::<Vec<String>>();

                if terms.len() < n_per_line + (n_per_line / 2) {
                    rows.push(format!(
                        "  a{:<5}( {} )",
                        form.to_string(),
                        form_rows.join(" ")
                    ));
                } else {
                    rows.push(format!("  a{:<5}(", form.to_string()));
                    form_rows
                        .iter()
                        .for_each(|r| rows.push(format!("           {}", r.to_string())));
                    rows.push("  )".to_string());
                }
            }
        }

        write!(f, "{{\n{}\n}}", rows.join("\n"))
    }
}

pub struct MvecIterator<'a> {
    ix: usize,
    terms: &'a Vec<Term>,
}

impl<'a> Iterator for MvecIterator<'a> {
    type Item = &'a Term;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.terms.get(self.ix);
        self.ix += 1;
        return item;
    }
}
