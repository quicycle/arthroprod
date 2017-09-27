use {ArError, Result};
use std::fmt;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
/// A single vector index of space or time.
///
/// The generators of the algebra are the standard (t, x, y, z) components of
/// Euclidian Space. For ease of expression we denote them using numeric
/// indices 0 through 3 with 0 representing the single time component.
pub enum Index {
    Zero,
    One,
    Two,
    Three,
}


impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Index::Zero => write!(f, "0"),
            Index::One => write!(f, "1"),
            Index::Two => write!(f, "2"),
            Index::Three => write!(f, "3"),
        }
    }
}

impl Index {
    /// Try to parse a string as an Index.
    ///
    /// Only values of 0, 1, 2 or 3 will succeed.
    pub fn try_from_str(s: &str) -> Result<Index> {
        match s {
            "0" => Ok(Index::Zero),
            "1" => Ok(Index::One),
            "2" => Ok(Index::Two),
            "3" => Ok(Index::Three),
            &_ => Err(ArError::InvalidIndex(String::from(s))),
        }
    }
}
