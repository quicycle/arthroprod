use crate::algebra::{full, Alpha, Axis, MultiVector, Sign, AR};

/// The dual of a Multivector is defined as being '-a0123 ^ M' and is denoted
/// with an overbar.
pub fn dual<T: AR>(arg: &T) -> MultiVector {
    let axes = [0, 1, 2, 3]
        .iter()
        .map(|n| Axis::try_from_u8(*n).unwrap())
        .collect();
    let q = Alpha::try_from_axes(Sign::Neg, &axes).unwrap();

    full(&q, arg)
}

/// Compute the product of M ^ dual(M)
pub fn mm_bar<T: AR>(arg: &T, cancel_term: bool) -> MultiVector {
    let mut result = full(arg, &dual(arg));
    if cancel_term {
        result.cancel_terms();
    };

    return result;
}
