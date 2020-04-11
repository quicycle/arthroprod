//! Magnitude is a simple rational number type that is intended for tracking term magnitudes within
//! algebraic calculations. It is not intended for use within numeric computation or simulation.
//! The numerator and denominator are always stored in lowest terms and opemagnitudens will panic
//! if the denominator is set to zero.
//! NOTE: division of Magnitudes is defined in standard (lhs / rhs) not (lhs \ rhs) as with division
//!       for AR. This is handled when working with Xi terms but should be taken into account if
//!       you want to manipulate raw Magnitude values.

use std::cmp;
use std::convert;
use std::fmt;
use std::ops;

/// A Magnitude is a strictly positive rational number. Sign (as it pertains to directed elements)
/// is stored in the Alpha value describine the element.
#[derive(Hash, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Magnitude {
    numerator: usize,
    denominator: usize,
}

impl Magnitude {
    pub fn new(numerator: usize, denominator: usize) -> Magnitude {
        let mut r = Magnitude::new_unchecked(numerator, denominator);
        r.reduce();
        r
    }

    fn new_unchecked(numerator: usize, denominator: usize) -> Magnitude {
        Magnitude {
            numerator,
            denominator,
        }
    }

    fn reduce(&mut self) {
        if self.denominator == 0 {
            panic!("magnitude denominator is 0")
        }
        if self.numerator == 0 {
            self.denominator = 1;
            return;
        }
        if self.numerator == self.denominator {
            self.numerator = 1;
            self.denominator = 1;
            return;
        }

        let g = gcd(self.numerator, self.denominator);
        self.numerator /= g;
        self.denominator /= g;
    }
}

fn gcd(n: usize, m: usize) -> usize {
    let mut a = n;
    let mut b = m;

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    return a;
}

impl fmt::Display for Magnitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.denominator {
            1 => write!(f, "{}", self.numerator),
            _ => write!(f, "{}/{}", self.numerator, self.denominator),
        }
    }
}

impl cmp::PartialEq<usize> for Magnitude {
    fn eq(&self, other: &usize) -> bool {
        self.denominator == 1 && self.numerator == *other
    }
}

impl cmp::PartialEq<Magnitude> for usize {
    fn eq(&self, other: &Magnitude) -> bool {
        other == self
    }
}

impl cmp::Eq for Magnitude {}

impl cmp::Ord for Magnitude {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // NOTE: this is in danger of overflowing but for our use case we will typically be fine.
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}

impl cmp::PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl convert::From<usize> for Magnitude {
    fn from(num: usize) -> Self {
        Magnitude::new_unchecked(num, 1)
    }
}

// e.g. let rat: Magnitude = (5, 3).into();
impl convert::From<(usize, usize)> for Magnitude {
    fn from(pair: (usize, usize)) -> Self {
        Magnitude::new(pair.0, pair.1)
    }
}

impl convert::Into<(usize, usize)> for Magnitude {
    fn into(self) -> (usize, usize) {
        (self.numerator, self.denominator)
    }
}

impl ops::Add for Magnitude {
    type Output = Self;

    fn add(self, rhs: Magnitude) -> Self::Output {
        let num = (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator);
        let den = self.denominator * rhs.denominator;

        Magnitude::new(num, den)
    }
}

impl ops::Add<usize> for Magnitude {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Magnitude::new(self.numerator + rhs * self.denominator, self.denominator)
    }
}

impl ops::Add<Magnitude> for usize {
    type Output = Magnitude;

    fn add(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(rhs.numerator + self * rhs.denominator, rhs.denominator)
    }
}

impl ops::AddAssign for Magnitude {
    fn add_assign(&mut self, other: Self) {
        self.numerator =
            (self.numerator * other.denominator) + (other.numerator * self.denominator);
        self.denominator = self.denominator * other.denominator;
    }
}

impl ops::SubAssign for Magnitude {
    fn sub_assign(&mut self, other: Self) {
        self.numerator =
            (self.numerator * other.denominator) - (other.numerator * self.denominator);
        self.denominator = self.denominator * other.denominator;
    }
}

impl ops::Sub for Magnitude {
    type Output = Self;

    fn sub(self, rhs: Magnitude) -> Self::Output {
        let num = (self.numerator * rhs.denominator) - (rhs.numerator * self.denominator);
        let den = self.denominator * rhs.denominator;

        Magnitude::new(num, den)
    }
}

impl ops::Sub<usize> for Magnitude {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Magnitude::new(self.numerator - rhs * self.denominator, self.denominator)
    }
}

impl ops::Sub<Magnitude> for usize {
    type Output = Magnitude;

    fn sub(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(rhs.numerator - self * rhs.denominator, rhs.denominator)
    }
}

impl ops::Mul for Magnitude {
    type Output = Self;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Mul<usize> for Magnitude {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Magnitude::new(self.numerator * rhs, self.denominator)
    }
}

impl ops::Mul<Magnitude> for usize {
    type Output = Magnitude;

    fn mul(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(self * rhs.numerator, rhs.denominator)
    }
}

impl ops::Div for Magnitude {
    type Output = Self;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl ops::Div<usize> for Magnitude {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Magnitude::new(self.numerator, self.denominator * rhs)
    }
}

impl ops::Div<Magnitude> for usize {
    type Output = Magnitude;

    fn div(self, rhs: Magnitude) -> Self::Output {
        Magnitude::new(self * rhs.denominator, rhs.numerator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, 5, Magnitude::new(3, 5))]
    #[test_case(4, 2, Magnitude::new(2, 1))]
    fn from_and_into_work(a: usize, b: usize, expected: Magnitude) {
        let r: Magnitude = (a, b).into();
        assert_eq!(r, expected);

        let (n, d) = expected.clone().into();
        assert_eq!(n, expected.numerator);
        assert_eq!(d, expected.denominator);
    }

    #[test_case(Magnitude::new(1, 2), Magnitude::new(1, 2), cmp::Ordering::Equal)]
    #[test_case(Magnitude::new(1, 2), Magnitude::new(5, 10), cmp::Ordering::Equal)]
    #[test_case(Magnitude::new(1, 3), Magnitude::new(2, 5), cmp::Ordering::Less)]
    #[test_case(Magnitude::new(3, 4), Magnitude::new(5, 9), cmp::Ordering::Greater)]
    fn comparison_works(left: Magnitude, right: Magnitude, ord: cmp::Ordering) {
        assert_eq!(left.cmp(&right), ord);
    }

    #[test]
    fn equality_works() {
        assert_eq!(Magnitude::new(2, 4), Magnitude::new(1, 2));
        assert_eq!(Magnitude::new(4, 2), 2);
        assert_eq!(5, Magnitude::new(15, 3));
    }

    #[test_case(2, 4, Magnitude { numerator: 1, denominator: 2 })]
    #[test_case(9, 3, Magnitude { numerator: 3, denominator: 1 })]
    fn reduction_on_creation_works(a: usize, b: usize, expected: Magnitude) {
        assert_eq!(Magnitude::new(a, b), expected);
    }

    #[test_case(Magnitude::new(1, 2), Magnitude::new(3, 4), Magnitude::new(5, 4))]
    #[test_case(Magnitude::new(3, 5), Magnitude::new(4, 3), Magnitude::new(29, 15))]
    fn addition_of_magnitudes_works(a: Magnitude, b: Magnitude, expected: Magnitude) {
        assert_eq!(a + b, expected);
    }

    #[test_case(Magnitude::new(1, 2), 1, Magnitude::new(3, 2))]
    #[test_case(Magnitude::new(3, 5), 3, Magnitude::new(18, 5))]
    fn addition_of_magnitudes_and_usize_works(a: Magnitude, b: usize, expected: Magnitude) {
        assert_eq!(a.clone() + b, expected);
        assert_eq!(b + a, expected);
    }

    #[test_case(Magnitude::new(3, 4), Magnitude::new(1, 2), Magnitude::new(1, 4))]
    #[test_case(Magnitude::new(1, 2), Magnitude::new(1, 2), Magnitude::new(0, 1))]
    fn subtraction_of_magnitudes_works(a: Magnitude, b: Magnitude, expected: Magnitude) {
        assert_eq!(a - b, expected);
    }

    #[test_case(Magnitude::new(3, 2), 1, Magnitude::new(1, 2))]
    #[test_case(Magnitude::new(12, 5), 2, Magnitude::new(2, 5))]
    fn subtraction_of_magnitudes_and_usize_works(a: Magnitude, b: usize, expected: Magnitude) {
        assert_eq!(a - b, expected);
    }

    #[test_case(Magnitude::new(1, 2), Magnitude::new(3, 4), Magnitude::new(3, 8))]
    #[test_case(Magnitude::new(2, 3), Magnitude::new(1, 2), Magnitude::new(1, 3))]
    fn multiplication_of_magnitudes_works(a: Magnitude, b: Magnitude, expected: Magnitude) {
        assert_eq!(a * b, expected);
    }

    #[test_case(Magnitude::new(1, 2), 2, Magnitude::new(1, 1))]
    #[test_case(Magnitude::new(3, 5), 5, Magnitude::new(3, 1))]
    #[test_case(Magnitude::new(2, 5), 4, Magnitude::new(8, 5))]
    fn multiplication_of_magnitudes_and_usize_works(a: Magnitude, b: usize, expected: Magnitude) {
        assert_eq!(a.clone() * b, expected);
        assert_eq!(b * a, expected);
    }

    #[test_case(Magnitude::new(1, 2), Magnitude::new(3, 4), Magnitude::new(2, 3))]
    #[test_case(Magnitude::new(1, 2), Magnitude::new(2, 3), Magnitude::new(3, 4))]
    #[test_case(Magnitude::new(3, 5), Magnitude::new(4, 3), Magnitude::new(9, 20))]
    fn division_of_magnitudes_works(a: Magnitude, b: Magnitude, expected: Magnitude) {
        assert_eq!(a / b, expected);
    }

    #[test_case(Magnitude::new(1, 2), 2, Magnitude::new(1, 4))]
    #[test_case(Magnitude::new(3, 5), 5, Magnitude::new(3, 25))]
    #[test_case(Magnitude::new(2, 5), 4, Magnitude::new(1, 10))]
    fn division_of_magnitudes_and_usize_works(a: Magnitude, b: usize, expected: Magnitude) {
        assert_eq!(a / b, expected);
    }
}
