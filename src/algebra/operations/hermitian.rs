//! The Hermitian Conjugate of a Multivector is defined to be 'a0 ^ rev(M) ^ a0'
//! with the notation signifying that the product is formed individually for each
//! term within the MultiVector.

use crate::algebra::{ar_product, MultiVector, Sign, Term, AR};

/// Compute the Hermitian conjugate (dagger) of the argument. This has the
/// effect of negating all terms whos alphas square to -ap
pub fn hermitian<T: AR>(arg: &T) -> MultiVector {
    let terms: Vec<Term> = arg
        .as_terms()
        .iter()
        .map(|t| match ar_product(&t.alpha(), &t.alpha()).sign() {
            Sign::Neg => -t.clone(),
            Sign::Pos => t.clone(),
        })
        .collect();

    MultiVector::from_terms(terms)
}

/// Compute the Hermitian conjugate (dagger) of the argument. This has the
/// effect of negating all terms whos alphas square to -ap
pub fn dagger<T: AR>(arg: &T) -> MultiVector {
    hermitian(arg)
}
