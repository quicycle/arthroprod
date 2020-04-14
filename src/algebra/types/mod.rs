//! Data types used to drive computations within AR.
//! While most of the arthroprod library functions will accept anything that impliments
//! the [`AR`] trait, the primary unit of computation is the [`MultiVector`]. This is in
//! keeping with the way that we work by hand, with [`Term`] and [`Alpha`] values being
//! considered rarely. That said, in some cases we make use of "raw" Alphas to implement
//! certain conjugates and commutators.

mod alpha;
mod enums;
mod magnitude;
mod multivector;
mod term;
mod xi;

pub use self::alpha::{Alpha, ALLOWED_ALPHA_FORMS};
pub use self::enums::{Form, Index, Sign};
pub use self::magnitude::Magnitude;
pub use self::multivector::MultiVector;
pub use self::term::Term;
pub use self::xi::Xi;
