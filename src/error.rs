use std::error::Error;
use std::fmt;

/// Errors that can be thrown in arthroprod
#[derive(Debug)]
pub enum ArError {
    /// The index passed was not one of 0, 1, 2 or 3.
    InvalidIndex(String),
    /// Components can only be of order 0, 1, 2, 3 or 4.
    InvalidComponentOrder(String),
    /// The component provided is not a member of ALLOWED.
    ComponentNotAllowed(String),
}

impl Error for ArError {
    fn description(&self) -> &str {
        "Error performing AR calculation"
    }
}

impl fmt::Display for ArError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArError::InvalidIndex(ref i) => write!(f, "The index provided was not one of 0, 1, 2 or 3: {}", i),
            ArError::InvalidComponentOrder(ref c) => write!(f, "Attempt to construct a component of order > 4: {}", c),
            ArError::ComponentNotAllowed(ref c) => write!(f, "Attempt to use invalid component: {}", c),
        }
    }
}
