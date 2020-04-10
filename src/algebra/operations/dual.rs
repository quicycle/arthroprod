use crate::algebra::{full, Alpha, Axis, MultiVector, Sign, AR};

/// The dual of a Multivector is defined as being '-a0123 ^ M' and is denoted
/// with an overbar.
pub fn dual<T: AR>(arg: &T) -> T {
    let axes = [0, 1, 2, 3]
        .iter()
        .map(|n| Axis::try_from_u8(*n).unwrap())
        .collect();
    let q = Alpha::try_from_axes(Sign::Neg, &axes).unwrap();

    full(&q, arg)
}

/// Compute the product of M ^ dual(M)
pub fn mm_bar<T: AR>(arg: &T, cancel_term: bool) -> MultiVector {
    let arg_dual: T = dual(arg);
    let mut result: MultiVector = full(arg, &arg_dual);
    if cancel_term {
        result.simplify();
    };

    return result;
}
