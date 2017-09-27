#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
/// A Sign can be positive or negative.
///
/// Ongoing work is being carried out to determine if this is expressive enough
/// for the algebra or whether we need to individually track the sign of a
/// component's magnitude, its handedness and its Alpha sign.
///
/// * magSign
///
/// * handSign
///
/// * uSign
///
/// The current implementation is to track Sign as a property of an Alpha and
/// to move sign changes within magnitude and handedness into this signle
/// representation when they occur.
pub enum Sign {
    Pos,
    Neg,
}


impl Sign {
    /// Combine two signs (positive, negative) and return their product under
    /// the standard rules of arithmetic.
    pub fn combine_with(&self, j: &Sign) -> Sign {
        if self == j { Sign::Pos } else { Sign::Neg }
    }
}
