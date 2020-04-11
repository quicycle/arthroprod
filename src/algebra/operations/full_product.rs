use crate::algebra::{Term, AR};

/// The full product between two elements within AR is defined as an extension of the traditional
/// Clifford product from a Clifford Algebera: we form the Cartesian product of the terms in left
/// and right using the AR full product.
pub fn full<L: AR, R: AR, T: AR>(left: &L, right: &R) -> T {
    T::from_terms(
        left.as_terms()
            .iter()
            .flat_map(|t_left| {
                right
                    .as_terms()
                    .iter()
                    .map(|t_right| t_left.form_product_with(&t_right))
                    .collect::<Vec<Term>>()
            })
            .collect(),
    )
}
