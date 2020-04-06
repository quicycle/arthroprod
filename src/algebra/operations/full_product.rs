//! The full product between two elements within AR is defined as an extension of the traditional
//! Clifford product from a Clifford Algebera: we form the Cartesian product of the terms in left
//! and right using the AR full product.

use crate::algebra::{ar_product, MultiVector, Term, AR};

/// Form the full, Cartesian product of left and right.
/// If the terms provided are real in nature (f64 values) then we combine them using traditional
/// scalar multiplication, if they are both symbolic (string values) then we concatenate them with
/// a separator. Mixed real and symbolic terms will result in a panic calculations should either
/// be entirely real or entirely symbolic.
pub fn full<L: AR, R: AR>(left: &L, right: &R) -> MultiVector {
    let mut terms: Vec<Term> = vec![];

    for tleft in left.as_terms() {
        let aleft = tleft.alpha();
        let (wleft, sleft) = tleft.xi().weight_and_symbol();

        for tright in right.as_terms() {
            let aright = tright.alpha();
            let (wright, sright) = tleft.xi().weight_and_symbol();

            let weight = wleft * wright;
            let symbol = format!("{}.{}", sleft, sright);
            let alpha = ar_product(&aleft, &aright);

            terms.push(weight * Term::new(symbol, alpha));
        }
    }

    MultiVector::from_terms(terms)
}
