//! Given that we are working within a non-commutative algebera, we need to define a convention
//! for division to establish the divisor. In practice we are chosing between 1/A ^ B or A ^ 1/B.
//! Our current thinking is that we need to define division as the former (dividing A into B).

use crate::algebra::{ar_product, diamond, full, hermitian, invert_alpha, MultiVector, Term, AR};

/// Divide left into right. When left and right are both terms or alphas, this is a relatively
/// simple inversion of left and then forming the full product. For MultiVectors this requires
/// a full general inverse using the Van Der Mark
pub fn div<L: AR, R: AR>(left: &L, right: &R) -> MultiVector {
    let lterms = left.as_terms();
    let rterms = right.as_terms();

    if lterms.len() == 1 && rterms.len() == 1 {
        div_single_terms(&lterms[0], &rterms[0])
    } else {
        apply_van_der_mark(left, right)
    }
}

// dividing left into right (left \ right)
fn div_single_terms(left: &Term, right: &Term) -> MultiVector {
    let (wleft, sleft) = left.xi().into();
    let (wright, sright) = right.xi().into();

    let weight = wright / wleft; // dividing into not by
    let symbol = format!("{}\\{}", sleft, sright);
    let alpha = ar_product(&invert_alpha(&left.alpha()), &right.alpha());

    let mut m = MultiVector::new();
    m.add_term(weight * Term::new(symbol, alpha));
    return m;
}

// dividing left into right (left \ right)
fn apply_van_der_mark<L: AR, R: AR>(left: &L, right: &R) -> MultiVector {
    let l_dagger = hermitian(left);
    let l_phi = full(left, &l_dagger);
    let l_diamond_phi = diamond(&l_phi);

    // guaranteed to be a single ap term when computing phi ^ diamond(phi)
    let (divisor, _) = full(&l_phi, &l_diamond_phi).as_terms()[0].xi().into();
    let inverse = full(&l_dagger, &l_diamond_phi);

    full(&inverse, right) / divisor
}
