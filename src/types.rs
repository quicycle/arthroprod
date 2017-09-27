//! Base types for carrying out calculations within the Absolute Relativity
//! framework.

use super::consts::ALLOWED;
use {ArError, Result};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug, Clone)]
/// A vector that is hashed based on it's sorted order.
pub struct KeyVec(Vec<Index>);

impl KeyVec {
    /// Generate a new KeyVec from a vector of indices.
    ///
    /// The elements are sorted when the KeyVec is created.
    pub fn new(v: Vec<Index>) -> KeyVec {
        let mut v = v.clone();
        v.sort();
        KeyVec(v)
    }
}

impl Hash for KeyVec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let KeyVec(ref elems) = *self;
        for elem in elems.iter() {
            elem.hash(state);
        }
    }
}

/////////////////
// .: Types :. //
/////////////////

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

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
/// An element of the algebra of order 0 through 4.
///
/// Components (along with an associated Sign) make up an Alpha value.
/// Functionally, components are tuples of Indices and for ease of writing
/// we denote higher order components in a contracted form:
///
/// ```ignore
/// α1,α2 == α12
/// α0,α2,α3 == α023
/// ```
///
/// In Clifford Algebra terms, these are the Blades of the algebra.
pub enum Component {
    Point,
    Vector(Index),
    Bivector(Index, Index),
    Trivector(Index, Index, Index),
    Quadrivector(Index, Index, Index, Index),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
/// The base element for computation with Absolute Relativity.
///
/// An Alpha is a Component along with an associated Sign. All values and
/// mathematical operators within the algebra are required to be paired with
/// their correct Alpha value under the principle of Absolute Relativity.
pub struct Alpha {
    index: Component,
    sign: Sign,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
/// A Xi is a value that must be bound to an Alpha.
pub enum Xi {
    /// A real number used in numeric calculations.
    Real(f64),
    /// A symbolic placeholder used in algebraic calculations.
    Symbolic(String),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
/// A Pair is either a real or symbolic Xi value and an paired Alpha.
pub struct Pair {
    xi: Xi,
    alpha: Alpha,
}

////////////////////////////////////////
// .: Type Display Implementations :. //
////////////////////////////////////////

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

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Component::Point => write!(f, "p"),
            Component::Vector(ref i) => write!(f, "{}", i),
            Component::Bivector(ref i, ref j) => write!(f, "{}{}", i, j),
            Component::Trivector(ref i, ref j, ref k) => write!(f, "{}{}{}", i, j, k),
            Component::Quadrivector(ref i, ref j, ref k, ref l) => write!(f, "{}{}{}{}", i, j, k, l),
        }
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Pos => write!(f, "α{}", self.index),
            Sign::Neg => write!(f, "-α{}", self.index),
        }
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Xi::Real(ref n) => write!(f, "ξ({})", n),
            Xi::Symbolic(ref s) => write!(f, "ξ{}", s),
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.alpha, self.xi)
    }
}

///////////////////////////////////////
// .: Type Method Implementations :. //
///////////////////////////////////////

impl Sign {
    /// Combine two signs (positive, negative) and return their product under
    /// the standard rules of arithmetic.
    pub fn combine_with(&self, j: &Sign) -> Sign {
        if self == j { Sign::Pos } else { Sign::Neg }
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

impl Component {
    /// Construct a new Component and verify that it is an allowed element of
    /// the algebra.
    pub fn new(ix: &str, allowed: &HashSet<Component>) -> Result<Component> {
        let index = Component::unsafe_new(ix)?;
        if !allowed.contains(&index) {
            return Err(ArError::ComponentNotAllowed(String::from(ix)));
        }
        Ok(index)
    }

    /// Construct a new Component without verification.
    pub fn unsafe_new(ix: &str) -> Result<Component> {
        if ix == "p" {
            return Ok(Component::Point);
        }

        let v: Vec<&str> = ix.split("")
                             .filter(|&c| c != "")
                             .collect();

        match v.len() {
            1 => {
                let i = Index::try_from_str(v[0])?;
                Ok(Component::Vector(i))
            }
            2 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                Ok(Component::Bivector(i1, i2))
            }
            3 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                Ok(Component::Trivector(i1, i2, i3))
            }
            4 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                let i4 = Index::try_from_str(v[3])?;
                Ok(Component::Quadrivector(i1, i2, i3, i4))
            }
            _ => return Err(ArError::InvalidComponentOrder(String::from(ix))),
        }
    }

    // TODO :: look at https://doc.rust-lang.org/std/convert/trait.Into.html
    /// Extract the indices of a component as a Vector.
    pub fn as_vec(&self) -> Vec<Index> {
        match *self {
            Component::Vector(i) => vec![i],
            Component::Bivector(i, j) => vec![i, j],
            Component::Trivector(i, j, k) => vec![i, j, k],
            Component::Quadrivector(i, j, k, l) => vec![i, j, k, l],
            Component::Point => vec![],
        }
    }
}

impl Alpha {
    /// create a new alpha from an string index containing an optional sign
    /// prefix.
    ///
    /// NOTE: This will panic if the index is invalid in order to prevent the
    /// user from running inconsistant calculations.
    pub fn new(ix: &str) -> Result<Alpha> {
        let sign = match ix.starts_with("-") {
            true => Sign::Neg,
            false => Sign::Pos,
        };

        let ix = ix.trim_matches('-');

        let index = Component::new(ix, &ALLOWED.indices())?;
        Ok(Alpha { index, sign })
    }

    /// new_override allows the caller to explicitly specify an index, sign and
    /// allowed set of alphas when creating an alpha.
    pub fn new_override(ix: &str, sign: Sign, allowed: &HashSet<Component>) -> Result<Alpha> {
        let index = Component::new(ix, allowed)?;
        Ok(Alpha { index, sign })
    }

    /// Construct an Alpha explicitly from a Component and a Sign.
    pub fn from_index(index: &Component, sign: &Sign) -> Alpha {
        Alpha {
            index: index.clone(),
            sign: sign.clone(),
        }
    }

    /// Check to see if an alpha is +/-αp
    pub fn is_point(&self) -> bool {
        self.index == Component::Point
    }

    /// Return a copy of this Alpha's index
    pub fn index(&self) -> &Component {
        &self.index
    }

    /// Return a copy of this Alpha's sign
    pub fn sign(&self) -> &Sign {
        &self.sign
    }

    /// Return a Vector of Indices representing this Alpha's Indices
    pub fn as_vec(&self) -> Vec<Index> {
        self.index.as_vec()
    }
}


impl Pair {
    /// Create a new Pair from a Xi and an Alpha.
    pub fn new(xi: Xi, alpha: Alpha) -> Pair {
        Pair { xi, alpha }
    }

    /// Create a default Symbolic Pair from an string Alpha index.
    pub fn sym(ix: &str) -> Result<Pair> {
        let alpha = Alpha::new(ix)?;
        // Remove any sign information before converting to a Xi
        let ix = ix.trim_matches('-');
        let xi = Xi::Symbolic(String::from(ix));
        Ok(Pair { xi, alpha })
    }

    /// The xi element of the Pair
    pub fn xi(&self) -> &Xi {
        &self.xi
    }

    /// The alpha element of the Pair
    pub fn alpha(&self) -> &Alpha {
        &self.alpha
    }
}
