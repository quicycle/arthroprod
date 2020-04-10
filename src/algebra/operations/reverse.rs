//! By inspection we can show that for alphas with a single index,
//! or the Quadrivector, the net sign following pops is unchanged.
//! For Bivectors and Trivectors the sign is reversed:
//!
//! a -> a       (no pops: no sign change)
//! ab -> ba     (1 pop:   sign change)
//! abc -> cba   (3 pops:  sign change)
//!   -acb
//!    cab
//!   -cba
//! abcd -> dcba (6 pops: no sign change)
//!   -abdc
//!    adbc
//!   -dabc
//!    dacb
//!   -dcab
//!    dcba
//!
//! Even though we only need to carry this operation out for objects of
//! grade 0 -> 4, we can show that the number of pops required for reversing
//! an Alpha of grade n is the (n-1)th triangular number.

use crate::algebra::{Component, Term, AR};

/// Reverse the order basis elements within an object and then resolve back into
/// permitted Alpha values. In notation, this is denoted with an over tilde (~).
pub fn rev<T: AR, U: AR>(arg: &T) -> U {
    let mut terms: Vec<Term> = vec![];

    for term in arg.as_terms() {
        match term.alpha().component() {
            Component::Vector(_) | Component::Quadrivector(_, _, _, _) => terms.push(-term),
            _ => terms.push(term),
        }
    }

    U::from_terms(terms)
}
