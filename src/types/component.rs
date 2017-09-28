use std::collections::HashSet;
use std::fmt;

use super::index::*;
use {ArError, Result};


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

        let v: Vec<&str> = ix.split("").filter(|&c| c != "").collect();

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
