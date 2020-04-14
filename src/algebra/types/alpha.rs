use std::fmt;
use std::ops;

use crate::algebra::{ar_product, Form, Index, Sign, Term, AR};

/// When creating Alphas only the following forms are valid
pub const ALLOWED_ALPHA_FORMS: [Form; 16] = [
    // Zet B
    Form::Point,
    Form::Bivector(Index::Two, Index::Three),
    Form::Bivector(Index::Three, Index::One),
    Form::Bivector(Index::One, Index::Two),
    // Zet T
    Form::Vector(Index::Zero),
    Form::Trivector(Index::Zero, Index::Two, Index::Three),
    Form::Trivector(Index::Zero, Index::Three, Index::One),
    Form::Trivector(Index::Zero, Index::One, Index::Two),
    // Zet A
    Form::Trivector(Index::One, Index::Two, Index::Three),
    Form::Vector(Index::One),
    Form::Vector(Index::Two),
    Form::Vector(Index::Three),
    // Zet E
    Form::Quadrivector(Index::Zero, Index::One, Index::Two, Index::Three),
    Form::Bivector(Index::Zero, Index::One),
    Form::Bivector(Index::Zero, Index::Two),
    Form::Bivector(Index::Zero, Index::Three),
];

pub(crate) const ALLOWED_ALPHA_STRINGS: [&'static str; 16] = [
    "p", "23", "31", "12", "0", "023", "031", "012", "123", "1", "2", "3", "0123", "01", "02", "03",
];

/// An Alpha represents a pure element of the algebra without magnitude.
/// It is composed of 0-4 Dimensions with the number of dimensions determining
/// its form: i.e. scalar, vector, bivector, trivector, quadrivector
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Alpha {
    sign: Sign,
    form: Form,
}

impl Alpha {
    /// Construct a new Alpha value from scratch. Errors if the Form given is
    /// not found in [`ALLOWED_ALPHA_FORMS`].
    pub fn new(sign: Sign, form: Form) -> Result<Alpha, String> {
        if ALLOWED_ALPHA_FORMS.iter().any(|&f| f == form) {
            Ok(Alpha { sign, form })
        } else {
            Err(format!("Invalid Alpha index: {:?}", form))
        }
    }

    /// Allow or construction of Alpha values from a dynamically created vector of
    /// [`Index`] values. Errors if the given vector does not map to one of the allowed
    /// forms given in [`ALLOWED_ALPHA_FORMS`].
    pub fn try_from_indices(sign: Sign, indices: &Vec<Index>) -> Result<Alpha, String> {
        let form = Form::try_from_indices(indices)?;

        Alpha::new(sign, form)
    }

    /// Take a copy of this Alphas [`Form`]
    pub fn form(&self) -> Form {
        self.form.clone()
    }

    /// Take a copy of this Alphas [`Sign`]
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

    fn inverse(&self) -> Self::Output {
        Alpha::new(self.sign.combine(&ar_product(&self, &self).sign), self.form).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_consts_match() {
        let forms: Vec<String> = ALLOWED_ALPHA_FORMS
            .iter()
            .map(|f| format!("{}", f))
            .collect();
        assert_eq!(forms, ALLOWED_ALPHA_STRINGS);
    }
}
