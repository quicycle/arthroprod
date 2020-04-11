use std::fmt;
use std::ops;

use crate::algebra::{Axis, Form, Sign, Term, AR};

/// When creating Alphas only the following forms are valid
pub const ALLOWED_ALPHA_FORMS: [Form; 16] = [
    // Zet B
    Form::Point,
    Form::Bivector(Axis::Y, Axis::Z),
    Form::Bivector(Axis::Z, Axis::X),
    Form::Bivector(Axis::X, Axis::Y),
    // Zet T
    Form::Vector(Axis::T),
    Form::Trivector(Axis::T, Axis::Y, Axis::Z),
    Form::Trivector(Axis::T, Axis::Z, Axis::X),
    Form::Trivector(Axis::T, Axis::X, Axis::Y),
    // Zet A
    Form::Trivector(Axis::X, Axis::Y, Axis::Z),
    Form::Vector(Axis::X),
    Form::Vector(Axis::Y),
    Form::Vector(Axis::Z),
    // Zet E
    Form::Quadrivector(Axis::T, Axis::X, Axis::Y, Axis::Z),
    Form::Bivector(Axis::T, Axis::X),
    Form::Bivector(Axis::T, Axis::Y),
    Form::Bivector(Axis::T, Axis::Z),
];

/// An Alpha represents a pure element of the algebra without magnitude.
/// It is composed of 0-4 Dimensions with the number of dimensions determining
/// its nature: i.e. scalar, vector, bivector, trivector, quadrivector
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Alpha {
    sign: Sign,
    form: Form,
}

impl Alpha {
    pub fn new(sign: Sign, form: Form) -> Result<Alpha, String> {
        if ALLOWED_ALPHA_FORMS.iter().any(|&f| f == form) {
            Ok(Alpha { sign, form })
        } else {
            Err(format!("Invalid Alpha index: {:?}", form))
        }
    }

    pub fn try_from_axes(sign: Sign, axes: &Vec<Axis>) -> Result<Alpha, String> {
        let form = Form::try_from_axes(axes)?;

        Alpha::new(sign, form)
    }

    pub fn is_point(&self) -> bool {
        self.form == Form::Point
    }

    pub fn form(&self) -> Form {
        self.form.clone()
    }

    pub fn sign(&self) -> Sign {
        self.sign.clone()
    }
}

impl AR for Alpha {
    type Output = Self;

    fn as_terms(&self) -> Vec<Term> {
        vec![Term::new(None, self.clone())]
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
            form: self.form,
        }
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}", self.sign, self.form)
    }
}
