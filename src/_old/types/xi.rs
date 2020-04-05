use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
/// A Xi is a value that must be bound to an Alpha.
pub enum Xi {
    /// A real number used in numeric calculations.
    Real(f64),
    /// A symbolic placeholder used in algebraic calculations.
    Symbolic(String),
}


impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Xi::Real(ref n) => write!(f, "ξ({})", n),
            Xi::Symbolic(ref s) => write!(f, "ξ{}", s),
        }
    }
}
