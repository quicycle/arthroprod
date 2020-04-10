use crate::algebra::{ar_product, Term, AR};

/// The full product between two elements within AR is defined as an extension of the traditional
/// Clifford product from a Clifford Algebera: we form the Cartesian product of the terms in left
/// and right using the AR full product.
pub fn full<L: AR, R: AR, T: AR>(left: &L, right: &R) -> T {
    let mut terms: Vec<Term> = vec![];

    for tleft in left.as_terms() {
        let aleft = tleft.alpha();
        let (wleft, sleft) = tleft.xi().into();

        for tright in right.as_terms() {
            let aright = tright.alpha();
            let (wright, sright) = tright.xi().into();

            let weight = wleft * wright;
            let symbol = format!("{}.{}", sleft, sright);
            let alpha = ar_product(&aleft, &aright);

            terms.push(weight * Term::new(symbol, alpha));
        }
    }

    T::from_terms(terms)
}
