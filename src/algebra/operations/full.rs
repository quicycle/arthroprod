use crate::algebra::{ar_product, MultiVector, Term, AR};

pub fn full<L: AR, R: AR>(left: &L, right: &R) -> MultiVector {
    let mut terms: Vec<Term> = vec![];

    for tleft in left.as_terms() {
        let aleft = tleft.alpha();
        // let xleft = tleft.xi();

        for tright in right.as_terms() {
            let aright = tright.alpha();
            // let xright = tright.xi();

            let alpha = ar_product(&aleft, &aright);
            let xi = "TODO";
            terms.push(Term::new_sym(String::from(xi), alpha));
        }
    }

    return MultiVector::from_terms(terms);
}
