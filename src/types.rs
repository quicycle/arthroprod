use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A vector that is hashed based on it's sorted order.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct KeyVec(Vec<Index>);

impl KeyVec {
    pub fn new(v: Vec<Index>) -> KeyVec {
        KeyVec(v)
    }
}

impl Hash for KeyVec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let KeyVec(ref elems) = *self;
        let mut elems = elems.clone();
        elems.sort();
        for elem in elems.iter() {
            elem.hash(state);
        }
    }
}

/////////////////
// .: Types :. //
/////////////////

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub enum Index {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Component {
    Point,
    Vector(Index),
    Bivector(Index, Index),
    Trivector(Index, Index, Index),
    Quadrivector(Index, Index, Index, Index),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Sign {
    Pos,
    Neg,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Alpha {
    pub index: Component,
    pub sign: Sign,
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
            Component::Quadrivector(ref i, ref j, ref k, ref l) => {
                write!(f, "{}{}{}{}", i, j, k, l)
            }
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

///////////////////////////////////////
// .: Type Method Implementations :. //
///////////////////////////////////////

impl Index {
    pub fn try_from_str(s: &str) -> Result<Index, &str> {
        match s {
            "0" => Ok(Index::Zero),
            "1" => Ok(Index::One),
            "2" => Ok(Index::Two),
            "3" => Ok(Index::Three),
            &_ => Err("Invalid index"),
        }
    }
}

impl Component {
    pub fn new<'a>(ix: &'a str, allowed: &HashSet<Component>) -> Result<Component, &'a str> {
        if ix == "p" {
            return Ok(Component::Point);
        }

        let v: Vec<&str> = ix.split("")
                             .filter(|&c| c != "")
                             .collect();

        let index = match v.len() {
            1 => {
                let i = Index::try_from_str(v[0])?;
                Component::Vector(i)
            }
            2 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                Component::Bivector(i1, i2)
            }
            3 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                Component::Trivector(i1, i2, i3)
            }
            4 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                let i4 = Index::try_from_str(v[3])?;
                Component::Quadrivector(i1, i2, i3, i4)
            }
            _ => return Err("A component has at most 4 indices."),
        };

        if !allowed.contains(&index) {
            return Err("Invalid index provided");
        }
        Ok(index)
    }

    // TODO :: look at https://doc.rust-lang.org/std/convert/trait.Into.html
    pub fn to_vec(&self) -> Vec<Index> {
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
    pub fn new<'a>(ix: &'a str, sign: Sign, allowed: &HashSet<Component>)
        -> Result<Alpha, &'a str> {
        let index = Component::new(ix, allowed)?;
        Ok(Alpha { index, sign })
    }

    pub fn is_point(&self) -> bool {
        self.index == Component::Point
    }
}
