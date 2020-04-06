//! Given that we are working within a non-commutative algebera, we need to define a convention
//! for division to establish the divisor. In practice we are chosing between 1/A ^ B or A ^ 1/B.
//! Our current thinking is that we need to define division as the former (dividing A into B).

use crate::algebra::{ar_product, invert_alpha, MultiVector, Term, AR};

/// Divide left into right. When left and right are both terms or alphas, this is a relatively
/// simple inversion of left and then forming the full product. For MultiVectors this requires
/// a full general inverse using the Van Der Mark
pub fn div<L: AR, R: AR>(left: &L, right: &R) -> MultiVector {
    let lterms = left.as_terms();
    let rterms = right.as_terms();

    if lterms.len() == 1 && rterms.len() == 1 {
        div_single_terms(&lterms[0], &rterms[0])
    } else {
        apply_van_der_mark(lterms, rterms)
    }
}

fn div_single_terms(left: &Term, right: &Term) -> MultiVector {
    let (wleft, sleft) = left.xi().weight_and_symbol();
    let (wright, sright) = right.xi().weight_and_symbol();

    let value = wleft * wright;
    let symbol = format!("{}\\{}", sleft, sright);
    let alpha = ar_product(&invert_alpha(&left.alpha()), &right.alpha());

    MultiVector::from_terms(vec![Term::new(value, symbol, alpha)])
}

fn apply_van_der_mark(left: Vec<Term>, right: Vec<Term>) -> MultiVector {
    panic!("TODO: need to implement other operations first")
}
