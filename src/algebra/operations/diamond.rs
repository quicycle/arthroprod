use crate::algebra::{Form, MultiVector, AR};

/// The diamond conjugate of a MultiVector is defined as
///     M_diamond = 2<M>0 - M
/// It negates everything with a 'direction' (e.g. not ap)
pub fn diamond(mvec: &MultiVector) -> MultiVector {
    MultiVector::from_terms(
        mvec.as_terms()
            .iter()
            .map(|t| match t.alpha().form() {
                Form::Point => t.clone(),
                _ => -t.clone(),
            })
            .collect(),
    )
}
