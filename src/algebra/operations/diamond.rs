use crate::algebra::{project, Component, MultiVector};

/// The diamond conjugate of a MultiVector is defined as
///     M_diamond = 2<M>0 - M
/// It negates everything with a 'direction' (e.g. not ap)
pub fn diamond(mvec: &MultiVector) -> MultiVector {
    let mut res = project(mvec, &Component::Point) * 2 - mvec.clone();
    res.simplify();

    res
}
