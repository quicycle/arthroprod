use std::mem;

use super::ar_product;
use crate::algebra::types::{Alpha, Form, Index, Sign, Term};

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
/// assert_eq!(res_mvec, mvec![-term!(["023", "01"], 1 2 3)]);
/// # }
/// ```
pub trait AR {
    type Output: AR;

    /// Decompose self into a Vector of underlying Terms
    fn as_terms(&self) -> Vec<Term>;

    /// Construct a concrete type from a Vector of Terms
    fn from_terms(terms: Vec<Term>) -> Self;

    /// The product inverse through ap of all terms within self
    fn inverse(&self) -> Self::Output {
        Self::Output::from_terms(self.as_terms().iter().map(|t| t.inverse()).collect())
    }

    /// Decompose self into a Vector of underlying Alphas
    fn as_alphas(&self) -> Vec<Alpha> {
        self.as_terms().iter().map(|t| t.alpha()).collect()
    }

    /// Construct a concrete type from a Vector of Alphas with default Xi
    /// values derived from each Alpha
    fn from_alphas(alphas: Vec<Alpha>) -> Self::Output {
        Self::Output::from_terms(alphas.iter().map(|a| Term::new(None, a.clone())).collect())
    }

    /// Check to see if self is entirely composed of scalar elements within
    /// the algebra (i.e. Point: nothing of a higher grade)
    fn is_scalar(&self) -> bool {
        self.as_terms()
            .iter()
            .fold(true, |acc, t| acc && t.form() == Form::Point)
    }

    /// Reverse the order basis elements within an object and then resolve back into
    /// permitted Alpha values. In notation, this is denoted with an over tilde (~).
    ///
    /// By inspection we can show that for alphas with a single index,
    /// or the Quadrivector, the net sign following pops is unchanged.
    /// For Bivectors and Trivectors the sign is reversed:
    ///
    /// a -> a       (no pops: no sign change)
    /// ab -> ba     (1 pop:   sign change)
    /// abc -> cba   (3 pops:  sign change)
    ///   -acb
    ///    cab
    ///   -cba
    /// abcd -> dcba (6 pops: no sign change)
    ///   -abdc
    ///    adbc
    ///   -dabc
    ///    dacb
    ///   -dcab
    ///    dcba
    ///
    /// Even though we only need to carry this operation out for objects of
    /// grade 0 -> 4, we can show that the number of pops required for reversing
    /// an Alpha of grade n is the (n-1)th triangular number.
    fn reversed(&self) -> Self::Output {
        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .map(|t| match t.alpha().form() {
                    Form::Vector(_) | Form::Quadrivector(_, _, _, _) => t.clone(),
                    _ => -t.clone(),
                })
                .collect(),
        )
    }

    /// Implementation of the grade-projection operator <A>n: filter terms, leaving only
    /// those that are of the specified grade. 'grade' is required only to give the
    /// desired output grade, the value of the component passed is ignored.
    fn project(&self, grade: &Form) -> Self::Output {
        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .filter(|t| mem::discriminant(&t.form()) == mem::discriminant(grade))
                .cloned()
                .collect(),
        )
    }

    /// Compute the Hermitian conjugate (dagger) of the argument. This has the
    /// effect of negating all terms whos alphas square to -ap.
    ///
    /// The Hermitian Conjugate of a Multivector is defined to be 'a0 ^ rev(M) ^ a0'
    /// with the notation signifying that the product is formed individually for each
    /// term within the MultiVector.
    fn hermitian(&self) -> Self::Output {
        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .map(|t| match ar_product(&t.alpha(), &t.alpha()).sign() {
                    Sign::Neg => -t.clone(),
                    Sign::Pos => t.clone(),
                })
                .collect(),
        )
    }

    /// Alias for the Hermitian conjugate
    fn dagger(&self) -> Self::Output {
        self.hermitian()
    }

    /// The diamond conjugate is defined as `M_diamond = 2<M>0 - M`
    /// It negates everything with a space-time 'direction' (i.e. everything but Point)
    fn diamond(&self) -> Self::Output {
        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .map(|t| match t.form() {
                    Form::Point => t.clone(),
                    _ => -t.clone(),
                })
                .collect(),
        )
    }

    /// The double dagger conjugate is defined as `M_double_dagger = 2<M>2 - M`
    /// It negates everything but the Bivector components (the fields).
    fn double_dagger(&self) -> Self::Output {
        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .map(|t| match t.form() {
                    Form::Bivector(_, _) => t.clone(),
                    _ => -t.clone(),
                })
                .collect(),
        )
    }

    /// The dual of a Multivector is defined as being '-a0123 ^ M' and is denoted
    /// with an overbar.
    /// It is the inverse of an element through a0123 as opposed to ap, meaning that
    /// the product of an element with its dual is always a0123.
    fn dual(&self) -> Self::Output {
        let indices = [0, 1, 2, 3]
            .iter()
            .map(|n| Index::try_from_u8(*n).unwrap())
            .collect();
        let q = Term::new(None, Alpha::try_from_indices(Sign::Neg, &indices).unwrap());

        Self::Output::from_terms(
            self.as_terms()
                .iter()
                .map(|t| q.form_product_with(&t))
                .collect(),
        )
    }
}

// Provide some simple default impls to avoid the need to wrap things in a full-fat AR
// impl in order to be able to work with them.

impl AR for Vec<Term> {
    type Output = Self;

    fn as_terms(&self) -> Vec<Term> {
        self.clone()
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        terms
    }
}

impl AR for Vec<Alpha> {
    type Output = Self;

    fn as_terms(&self) -> Vec<Term> {
        self.iter().map(|a| Term::new(None, a.clone())).collect()
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        terms.iter().map(|t| t.alpha()).collect()
    }
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
            let conjugate = alpha.hermitian();

            assert_eq!(conjugate, Alpha::new(sign, *c).unwrap());
        }
    }

    #[test]
    fn hermitian_conjugation_is_correct_for_terms() {
        for c in ALLOWED_ALPHA_FORMS.iter() {
            let alpha = Alpha::new(Sign::Pos, *c).unwrap();
            let sign = ar_product(&alpha, &alpha).sign();
            let term = Term::new(None, alpha.clone());
            let conjugate = term.hermitian();

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

        let conjugate = MultiVector::from_terms(terms).hermitian();
        assert_eq!(conjugate, MultiVector::from_terms(negated));
    }
}
