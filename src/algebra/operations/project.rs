use std::mem;

use crate::algebra::{Component, MultiVector, Term, AR};

/// Implementation of the grade-projection operator <A>n: filter terms, leaving only those
/// that are of the specified grade. 'grade' is required only to give the desired output
/// grade, the value of the component passed is ignored.
pub fn project<T: AR>(arg: &T, grade: &Component) -> MultiVector {
    let terms: Vec<Term> = arg
        .as_terms()
        .iter()
        .filter(|t| mem::discriminant(&t.alpha().component()) == mem::discriminant(grade))
        .cloned()
        .collect();

    MultiVector::from_terms(terms)
}