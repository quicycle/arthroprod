use crate::algebra::{project, Component, MultiVector, AR};

/// The diamond conjugate of a MultiVector is defined as
///     M_diamond = 2<M>0 - M
/// It negates everything with a 'direction' (e.g. not ap)
pub fn diamond<T: AR>(arg: &T) -> MultiVector {
    let scalar_terms_only = project(arg, &Component::Point) * 2;
    let mut res = scalar_terms_only - MultiVector::from_terms(arg.as_terms());
    res.cancel_terms();

    res
}
