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

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Alpha {
    comp: Component,
    sign: Sign,
}

impl AR for Alpha {
    fn as_terms(&self) -> Vec<Term> {
        vec![Term::from_alpha(self.clone())]
    }
}

impl ops::Neg for Alpha {
    type Output = Alpha;

    fn neg(self) -> Self::Output {
        Alpha {
            comp: self.comp,
            sign: -self.sign,
        }
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}", self.sign, self.comp)
    }
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
