use std::fmt;
use std::ops;

use crate::algebra::{Axis, Component, Sign, Term, AR};

/// When creating Alphas only the following components are valid
pub const ALLOWED_ALPHA_COMPONENTS: [Component; 16] = [
    // Zet B
    Component::Point,
    Component::Bivector(Axis::Y, Axis::Z),
    Component::Bivector(Axis::Z, Axis::X),
    Component::Bivector(Axis::X, Axis::Y),
    // Zet T
    Component::Vector(Axis::T),
    Component::Trivector(Axis::T, Axis::Y, Axis::Z),
    Component::Trivector(Axis::T, Axis::Z, Axis::X),
    Component::Trivector(Axis::T, Axis::X, Axis::Y),
    // Zet A
    Component::Trivector(Axis::X, Axis::Y, Axis::Z),
    Component::Vector(Axis::X),
    Component::Vector(Axis::Y),
    Component::Vector(Axis::Z),
    // Zet E
    Component::Quadrivector(Axis::T, Axis::X, Axis::Y, Axis::Z),
    Component::Bivector(Axis::T, Axis::X),
    Component::Bivector(Axis::T, Axis::Y),
    Component::Bivector(Axis::T, Axis::Z),
];

/// An Alpha represents a pure element of the algebra without magnitude.
/// It is composed of 0-4 Dimensions with the number of dimensions determining
/// its nature: i.e. scalar, vector, bivector, trivector, quadrivector
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Alpha {
    sign: Sign,
    comp: Component,
}

impl Alpha {
    pub fn new(sign: Sign, comp: Component) -> Result<Alpha, String> {
        if ALLOWED_ALPHA_COMPONENTS.iter().any(|&c| c == comp) {
            Ok(Alpha { comp, sign })
        } else {
            Err(format!("Invalid Alpha index: {:?}", comp))
        }
    }

    pub fn try_from_axes(sign: Sign, axes: &Vec<Axis>) -> Result<Alpha, String> {
        let comp = Component::try_from_axes(axes)?;

        Alpha::new(sign, comp)
    }

    pub fn is_point(&self) -> bool {
        self.comp == Component::Point
    }

    pub fn component(&self) -> Component {
        self.comp.clone()
    }

    pub fn sign(&self) -> Sign {
        self.sign.clone()
    }
}

impl AR for Alpha {
    fn as_terms(&self) -> Vec<Term> {
        vec![Term::from_alpha(self.clone())]
    }

    fn from_terms(terms: Vec<Term>) -> Self {
        if terms.len() != 1 {
            panic!("Can only construct an Alpha from a single term")
        };

        terms[0].alpha()
    }
}

impl ops::Neg for Alpha {
    type Output = Alpha;

    fn neg(self) -> Self::Output {
        Alpha {
            sign: -self.sign,
            comp: self.comp,
        }
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}", self.sign, self.comp)
    }
}
