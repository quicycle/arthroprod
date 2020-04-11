use std::fmt;
use std::ops::Neg;

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

impl Neg for Sign {
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
pub enum Axis {
    T,
    X,
    Y,
    Z,
}

impl Axis {
    /// Allow for construction of Axis values using 0-3 notation
    pub fn try_from_u8(x: u8) -> Result<Axis, String> {
        match x {
            0 => Ok(Axis::T),
            1 => Ok(Axis::X),
            2 => Ok(Axis::Y),
            3 => Ok(Axis::Z),
            _ => Err(format!("{:?} is not a valid axis", x)),
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Axis::T => write!(f, "0"),
            Axis::X => write!(f, "1"),
            Axis::Y => write!(f, "2"),
            Axis::Z => write!(f, "3"),
        }
    }
}

/// An AR geometric form based on grade (number of axes) involved. While any set of [`Axis`]
/// values can be used to construct a Form, those that are not found within [`ALLOWED_ALPHA_FORMS`]
/// will result in Errors when used in calculations.
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Form {
    Point,
    Vector(Axis),
    Bivector(Axis, Axis),
    Trivector(Axis, Axis, Axis),
    Quadrivector(Axis, Axis, Axis, Axis),
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
    /// Attempt to construct a Form from an arbitrary vector of [`Axis`] values.
    /// This will Error if axes.len() > 4
    pub fn try_from_axes(axes: &Vec<Axis>) -> Result<Form, String> {
        match axes.len() {
            0 => Ok(Form::Point),
            1 => Ok(Form::Vector(axes[0])),
            2 => Ok(Form::Bivector(axes[0], axes[1])),
            3 => Ok(Form::Trivector(axes[0], axes[1], axes[2])),
            4 => Ok(Form::Quadrivector(axes[0], axes[1], axes[2], axes[3])),
            _ => Err(format!("Invalid component indices {:?}", axes)),
        }
    }

    /// Extract the underlying [`Axis`] values contained in this Form.
    pub fn as_vec(&self) -> Vec<Axis> {
        match *self {
            Form::Point => vec![],
            Form::Vector(i) => vec![i],
            Form::Bivector(i, j) => vec![i, j],
            Form::Trivector(i, j, k) => vec![i, j, k],
            Form::Quadrivector(i, j, k, l) => vec![i, j, k, l],
        }
    }
}
