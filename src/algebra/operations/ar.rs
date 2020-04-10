use crate::algebra::{Alpha, Term};

/// Types that implement AR are able to be consumed by any of the library operations
/// provided by arthroprod. The return of these library functions is typically something
/// that also impliments AR:
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use arthroprod::algebra::*;
///
/// let a1 = alpha!(0 2 3);
/// let a2 = -alpha!(0 1);
///
/// // full takes two arguments that implement AR and tries to return an AR impl. Some
/// // AR impls will panic if constructed incorrectly (i.e. constructing an alpha from
/// // a vector of multiple values)
/// let res_alpha: Alpha = full(&a1, &a2);
/// let res_mvec: MultiVector = full(&a1, &a2);
///
/// assert_eq!(res_alpha, -alpha!(1 2 3));
/// assert_eq!(res_mvec, mvec![-term!("023.01", 1 2 3)]);
/// # }
/// ```
pub trait AR {
    fn as_terms(&self) -> Vec<Term>;
    fn from_terms(terms: Vec<Term>) -> Self;
}

// Provide some simple default impls to avoid the need to wrap things in a full-fat AR
// impl in order to be able to work with them.

impl AR for Vec<Term> {
    fn as_terms(&self) -> Vec<Term> {
        self.clone()
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        terms
    }
}

impl AR for Vec<Alpha> {
    fn as_terms(&self) -> Vec<Term> {
        self.iter().map(|a| Term::from_alpha(a.clone())).collect()
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        terms.iter().map(|t| t.alpha()).collect()
    }
}
