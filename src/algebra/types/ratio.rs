//! Raito is a simple rational number type that is intended for tracking term weights within
//! algebraic calculations. It is not intended for use within numeric computation or simulation.
//! The numerator and denominator are always stored in lowest terms and operations will panic if
//! the denominator is set to zero.

use std::cmp;
use std::convert;
use std::fmt;
use std::ops;

fn gcd(n: isize, m: isize) -> isize {
    let mut a = n.abs();
    let mut b = m.abs();

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    return a;
}

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord)]
pub struct Ratio {
    numerator: isize,
    denominator: isize,
}

impl Ratio {
    pub fn new(numerator: isize, denominator: isize) -> Ratio {
        let mut r = Ratio::new_unchecked(numerator, denominator);
        r.reduce();
        r
    }

    fn new_unchecked(numerator: isize, denominator: isize) -> Ratio {
        Ratio {
            numerator,
            denominator,
        }
    }

    fn reduce(&mut self) {
        if self.denominator == 0 {
            panic!("ratio denominator is 0")
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

        // Ensure that we store the sign information in the numerator
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl cmp::PartialEq<isize> for Ratio {
    fn eq(&self, other: &isize) -> bool {
        self.denominator == 1 && self.numerator == *other
    }
}

impl cmp::PartialEq<Ratio> for isize {
    fn eq(&self, other: &Ratio) -> bool {
        other == self
    }
}

impl convert::From<isize> for Ratio {
    fn from(num: isize) -> Self {
        Ratio::new_unchecked(num, 1)
    }
}

// e.g. let rat: Ratio = (5, 3).into();
impl convert::From<(isize, isize)> for Ratio {
    fn from(pair: (isize, isize)) -> Self {
        Ratio::new(pair.0, pair.1)
    }
}

impl ops::Neg for Ratio {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Ratio::new(-self.numerator, self.denominator)
    }
}

impl ops::Add for Ratio {
    type Output = Self;

    fn add(self, rhs: Ratio) -> Self::Output {
        let num = (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator);
        let den = self.denominator * rhs.denominator;

        Ratio::new(num, den)
    }
}

impl ops::Add<isize> for Ratio {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        Ratio::new(self.numerator + rhs * self.denominator, self.denominator)
    }
}

impl ops::Add<Ratio> for isize {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Self::Output {
        Ratio::new(rhs.numerator + self * rhs.denominator, rhs.denominator)
    }
}

impl ops::Mul for Ratio {
    type Output = Self;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Ratio::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Mul<isize> for Ratio {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Ratio::new(self.numerator * rhs, self.denominator)
    }
}

impl ops::Mul<Ratio> for isize {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Ratio::new(self * rhs.numerator, rhs.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 4, Ratio { numerator: 1, denominator: 2 })]
    #[test_case(-9, 3, Ratio { numerator: -3, denominator: 1 })]
    #[test_case(12, -15, Ratio { numerator: -4, denominator: 5 })]
    #[test_case(-9, -15, Ratio { numerator: 3, denominator: 5 })]
    fn reduction_on_creation_works(a: isize, b: isize, expected: Ratio) {
        assert_eq!(Ratio::new(a, b), expected);
    }

    #[test_case(2, 4)]
    #[test_case(-9, 3)]
    #[test_case(5, -7)]
    #[test_case(-9, -15)]
    fn negation_works(a: isize, b: isize) {
        assert_eq!(-Ratio::new(a, b), Ratio::new(-a, b));
    }

    #[test_case(Ratio::new(1, 2), Ratio::new(3, 4), Ratio::new(5, 4))]
    #[test_case(Ratio::new(1, 2), Ratio::new(-1, 2), Ratio::new(0, 1))]
    #[test_case(Ratio::new(-3, 5), Ratio::new(-4, 3), Ratio::new(-29, 15))]
    fn addition_of_ratios_works(a: Ratio, b: Ratio, expected: Ratio) {
        assert_eq!(a + b, expected);
    }

    #[test_case(Ratio::new(1, 2), 1, Ratio::new(3, 2))]
    #[test_case(Ratio::new(1, 2), -2, Ratio::new(-3, 2))]
    #[test_case(Ratio::new(-3, 5), 3, Ratio::new(12, 5))]
    fn addition_of_ratios_and_isize_works(a: Ratio, b: isize, expected: Ratio) {
        assert_eq!(a.clone() + b, expected);
        assert_eq!(b + a, expected);
    }

    #[test_case(Ratio::new(1, 2), Ratio::new(3, 4), Ratio::new(3, 8))]
    #[test_case(Ratio::new(2, 3), Ratio::new(-1, 2), Ratio::new(-1, 3))]
    #[test_case(Ratio::new(-1, 2), Ratio::new(2, 3), Ratio::new(-1, 3))]
    #[test_case(Ratio::new(-3, 5), Ratio::new(-4, 3), Ratio::new(4, 5))]
    fn multiplication_of_ratios_works(a: Ratio, b: Ratio, expected: Ratio) {
        assert_eq!(a * b, expected);
    }

    #[test_case(Ratio::new(1, 2), 2, Ratio::new(1, 1))]
    #[test_case(Ratio::new(1, 2), -3, Ratio::new(-3, 2))]
    #[test_case(Ratio::new(-3, 5), 5, Ratio::new(-3, 1))]
    #[test_case(Ratio::new(-2, 5), -4, Ratio::new(8, 5))]
    fn multiplication_of_ratios_and_isize_works(a: Ratio, b: isize, expected: Ratio) {
        assert_eq!(a.clone() * b, expected);
        assert_eq!(b * a, expected);
    }
}
