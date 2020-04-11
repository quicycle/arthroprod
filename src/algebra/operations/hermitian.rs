//! The Hermitian Conjugate of a Multivector is defined to be 'a0 ^ rev(M) ^ a0'
//! with the notation signifying that the product is formed individually for each
//! term within the MultiVector.

use crate::algebra::{ar_product, Sign, Term, AR};

/// Compute the Hermitian conjugate (dagger) of the argument. This has the
/// effect of negating all terms whos alphas square to -ap
pub fn hermitian<T: AR>(arg: &T) -> T {
    let terms: Vec<Term> = arg
        .as_terms()
        .iter()
        .map(|t| match ar_product(&t.alpha(), &t.alpha()).sign() {
            Sign::Neg => -t.clone(),
            Sign::Pos => t.clone(),
        })
        .collect();

    T::from_terms(terms)
}

/// Compute the Hermitian conjugate (dagger) of the argument. This has the
/// effect of negating all terms whos alphas square to -ap
pub fn dagger<T: AR>(arg: &T) -> T {
    hermitian(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{ar_product, Alpha, MultiVector, Term, ALLOWED_ALPHA_FORMS};

    #[test]
    fn hermitian_conjugation_is_correct_for_alphas() {
        for c in ALLOWED_ALPHA_FORMS.iter() {
            let alpha = Alpha::new(Sign::Pos, *c).unwrap();
            let sign = ar_product(&alpha, &alpha).sign();
            let conjugate = hermitian(&alpha);

            assert_eq!(conjugate, Alpha::new(sign, *c).unwrap());
        }
    }

    #[test]
    fn hermitian_conjugation_is_correct_for_terms() {
        for c in ALLOWED_ALPHA_FORMS.iter() {
            let alpha = Alpha::new(Sign::Pos, *c).unwrap();
            let sign = ar_product(&alpha, &alpha).sign();
            let term = Term::new(None, alpha.clone());
            let conjugate = hermitian(&term);

            assert_eq!(conjugate, Term::new(None, Alpha::new(sign, *c).unwrap()));
        }
    }

    #[test]
    fn hermitian_conjugation_is_correct_for_multivectors() {
        let mut terms: Vec<Term> = vec![];
        let mut negated: Vec<Term> = vec![];

        for c in ALLOWED_ALPHA_FORMS.iter() {
            let alpha = Alpha::new(Sign::Pos, *c).unwrap();
            let sign = ar_product(&alpha, &alpha).sign();
            terms.push(Term::new(None, alpha));
            negated.push(Term::new(None, Alpha::new(sign, *c).unwrap()));
        }

        let conjugate = hermitian(&MultiVector::from_terms(terms));
        assert_eq!(conjugate, MultiVector::from_terms(negated));
    }
}
