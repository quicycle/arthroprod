/// Simpler variadic generation of [`Alpha`] values.
/// This is the recommended way of creating raw alpha values if they are needed. Arguments
/// are u8s in the range 0-3.
///
/// # Panics
///
/// Panics if the specified alpha indices do not correspond to an allowed alpha (see
/// [`ALLOWED_ALPHA_FORMS`]).
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use arthroprod::algebra::*;
///
/// let a1 = alpha!(0 2 3);
/// let a2 = -alpha!(0 1);
/// let a3 = alpha!();
///
/// assert_eq!(a1, Alpha::new(Sign::Pos, Form::Trivector(Axis::T, Axis::Y, Axis::Z)).unwrap());
/// assert_eq!(a2, Alpha::new(Sign::Neg, Form::Bivector(Axis::T, Axis::X)).unwrap());
/// assert_eq!(a3, Alpha::new(Sign::Pos, Form::Point).unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! alpha(
    ($($num:expr) *) => {
        {
            let sign = Sign::Pos;
            let mut axes = Vec::new();
            $(axes.push(Axis::try_from_u8($num).unwrap());)*

            Alpha::try_from_axes(sign, &axes).unwrap()
        }
    };
);

/// Simpler variadic generation of [`Term`] values.
/// Terms created this way will have a default value (if one is not provided) and a
/// magnitude of 1. See [`alpha`] for more information on how the underlying [`Alpha`]
/// value is generated. It is also possible to specify a set of [`Xi`] values to use
/// for the term by providing a list of strings to use as the Xi symbolic values.
///
/// # Panics
///
/// Panics if the specified alpha indices do not correspond to an allowed alpha (see
/// [`ALLOWED_ALPHA_FORMS`]) or if the [`Xi`] values can not be converted to Strings.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use arthroprod::algebra::*;
///
/// let t1 = term!(0 2 3);
/// let t2 = -term!("symbolic", 0 1);
/// let t3 = term!(["X", "Y"], 2);
///
/// let a1 = Alpha::new(Sign::Pos, Form::Trivector(Axis::T, Axis::Y, Axis::Z)).unwrap();
/// let a2 = Alpha::new(Sign::Neg, Form::Bivector(Axis::T, Axis::X)).unwrap();
/// let a3 = Alpha::new(Sign::Pos, Form::Vector(Axis::Y)).unwrap();
///
/// assert_eq!(t1, Term::new(None, a1));
/// assert_eq!(t2, Term::new(Some("symbolic".to_string()), a2));
/// assert_eq!(t3, Term::from_xis_and_alpha(vec!["X".to_string(), "Y".to_string()], a3));
/// # }
/// ```
#[macro_export]
macro_rules! term(
    ($($num:expr) *) => {
        {
            let sign = Sign::Pos;
            let mut axes = Vec::new();
            $(axes.push(Axis::try_from_u8($num).unwrap());)*
            let alpha = Alpha::try_from_axes(sign, &axes).unwrap();

            Term::new(None, alpha)
        }
    };

    ([$($xi:expr),+], $($num:expr) *) => {
        {
            let sign = Sign::Pos;
            let mut axes = Vec::new();
            let mut xis = vec![];
            $(xis.push(String::from($xi));)+
            $(axes.push(Axis::try_from_u8($num).unwrap());)*
            let alpha = Alpha::try_from_axes(sign, &axes).unwrap();

            Term::from_xis_and_alpha(xis, alpha)
        }
    };

    ($sym:tt, $($num:expr) +) => {
        {
            let sign = Sign::Pos;
            let mut axes = Vec::new();
            let symbol = String::from($sym);
            $(axes.push(Axis::try_from_u8($num).unwrap());)+
            let alpha = Alpha::try_from_axes(sign, &axes).unwrap();

            Term::new(Some(symbol), alpha)
        }
    };
);

/// Simpler variadic generation of [`MultiVector`] values.
/// Each argument must impliment the AR trait so that it is possible to convert them to
/// [`Term`]s, with the resulting MultiVector is the sum of all terms generated this way.
///
/// # Panics
///
/// Panics if any of the arguments do not impliment the AR trait.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use arthroprod::algebra::*;
///
/// let m1 = mvec![alpha!(1), -term!(0 3)];
/// let mut m2 = MultiVector::new();
/// m2.push(Term::new(None, alpha!(1)));
/// m2.push(-term!(0 3));
///
/// assert_eq!(m1, m2);
/// # }
/// ```
#[macro_export]
macro_rules! mvec(
    [$($ar_elem:expr),+] => {
        {
            let mut terms = Vec::new();
            $(terms.extend($ar_elem.as_terms());)+

            MultiVector::from_terms(terms)
        }
    };
);

/// A simple helper for constructing hashmaps with less verbosity.
/// # Examples
///
/// ```
/// # #[macro_use] extern crate arthroprod; fn main() {
/// use std::collections::HashMap;
///
/// let m = map!{
///     "foo" => vec![1, 2, 3],
///     "bar" => vec![4, 5, 6]
/// };
///
/// assert_eq!(m.get("foo"), Some(&vec![1, 2, 3]));
/// # }
/// ```
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(_map.insert($key, $value);)+
            _map
        }
    };
);
