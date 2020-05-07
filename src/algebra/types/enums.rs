use std::cmp;
use std::fmt;
use std::ops;

use super::ALLOWED_ALPHA_FORMS;

/// Simple vector directed sign (positive or negative)
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Sign {
    Pos,
    Neg,
}

impl Sign {
    /// Combine together two Signs using conventional rules of arithmetic
    pub fn combine(&self, other: &Sign) -> Sign {
        if self == other {
            Sign::Pos
        } else {
            Sign::Neg
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sign::Pos => write!(f, "+"),
            Sign::Neg => write!(f, "-"),
        }
    }
}

impl ops::Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Neg => Sign::Pos,
            Sign::Pos => Sign::Neg,
        }
    }
}

/// A single Space-Time axis. One of the four basis elements for the coordinate system we work in
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Index {
    Zero,
    One,
    Two,
    Three,
}

impl Index {
    /// Allow for construction of Index values using 0-3 notation
    pub fn try_from_u8(x: u8) -> Result<Index, String> {
        match x {
            0 => Ok(Index::Zero),
            1 => Ok(Index::One),
            2 => Ok(Index::Two),
            3 => Ok(Index::Three),
            _ => Err(format!("{:?} is not a valid index", x)),
        }
    }
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

/// An AR geometric form based on grade (number of ixs) involved. While any set of [`Index`]
/// values can be used to construct a Form, those that are not found within [`ALLOWED_ALPHA_FORMS`]
/// will result in Errors when used in calculations.
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Form {
    Point,
    Vector(Index),
    Bivector(Index, Index),
    Trivector(Index, Index, Index),
    Quadrivector(Index, Index, Index, Index),
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Form::Point => write!(f, "p"),
            Form::Vector(i) => write!(f, "{}", i),
            Form::Bivector(i, j) => write!(f, "{}{}", i, j),
            Form::Trivector(i, j, k) => write!(f, "{}{}{}", i, j, k),
            Form::Quadrivector(i, j, k, l) => write!(f, "{}{}{}{}", i, j, k, l),
        }
    }
}

impl Form {
    /// Attempt to construct a Form from an arbitrary vector of [`Index`] values.
    /// This will Error if indices.len() > 4
    pub fn try_from_indices(ixs: &Vec<Index>) -> Result<Form, String> {
        match ixs.len() {
            0 => Ok(Form::Point),
            1 => Ok(Form::Vector(ixs[0])),
            2 => Ok(Form::Bivector(ixs[0], ixs[1])),
            3 => Ok(Form::Trivector(ixs[0], ixs[1], ixs[2])),
            4 => Ok(Form::Quadrivector(ixs[0], ixs[1], ixs[2], ixs[3])),
            _ => Err(format!("Invalid component indices {:?}", ixs)),
        }
    }

    /// Extract the underlying [`Index`] values contained in this Form.
    pub fn as_vec(&self) -> Vec<Index> {
        match *self {
            Form::Point => vec![],
            Form::Vector(i) => vec![i],
            Form::Bivector(i, j) => vec![i, j],
            Form::Trivector(i, j, k) => vec![i, j, k],
            Form::Quadrivector(i, j, k, l) => vec![i, j, k, l],
        }
    }
}

impl cmp::Ord for Form {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let i1 = ALLOWED_ALPHA_FORMS
            .iter()
            .position(|f| f == self)
            .expect(&format!("{} is an invalid space-time Form", self));
        let i2 = ALLOWED_ALPHA_FORMS
            .iter()
            .position(|f| f == other)
            .expect(&format!("{} is an invalid space-time Form", other));

        i1.cmp(&i2)
    }
}

impl cmp::PartialOrd for Form {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
