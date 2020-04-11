use std::fmt;
use std::ops::Neg;

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Sign {
    Pos,
    Neg,
}

impl Sign {
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

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Axis {
    T,
    X,
    Y,
    Z,
}

impl Axis {
    pub fn try_from_u8(x: u8) -> Result<Axis, String> {
        match x {
            0 => Ok(Axis::T),
            1 => Ok(Axis::X),
            2 => Ok(Axis::Y),
            3 => Ok(Axis::Z),
            _ => Err(format!("{:?} is not a valid axis", x)),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Axis::T => 0,
            Axis::X => 1,
            Axis::Y => 2,
            Axis::Z => 3,
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
    pub fn try_from_axes(ixs: &Vec<Axis>) -> Result<Form, String> {
        match ixs.len() {
            0 => Ok(Form::Point),
            1 => Ok(Form::Vector(ixs[0])),
            2 => Ok(Form::Bivector(ixs[0], ixs[1])),
            3 => Ok(Form::Trivector(ixs[0], ixs[1], ixs[2])),
            4 => Ok(Form::Quadrivector(ixs[0], ixs[1], ixs[2], ixs[3])),
            _ => Err(format!("Invalid component indices {:?}", ixs)),
        }
    }

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
