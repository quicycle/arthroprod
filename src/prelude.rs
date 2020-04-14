//! Common data structures and operators used in the algebra
#![allow(non_snake_case, non_upper_case_globals)]

use crate::algebra::types::ALLOWED_ALPHA_STRINGS;
use crate::algebra::ArDifferential;
use crate::algebra::{Alpha, Form, Index, MultiVector, Sign, Term, AR};

// helper for defining common multivectors
macro_rules! __default_mvec_impl(
    ($name:ident $term_str:expr) => {
        pub fn $name() -> MultiVector {
            let mut terms = vec![];
            for s in $term_str.split_whitespace() {
                if s == "p" {
                    terms.push(Term::new(None, Alpha::new(Sign::Pos, Form::Point).unwrap()))
                } else {
                    let indices = s.chars().map(|c|
                        match c {
                            '0' => Index::Zero,
                            '1' => Index::One,
                            '2' => Index::Two,
                            '3' => Index::Three,
                            _ => panic!("invalid index {}", c)
                        }
                    ).collect();
                    let alpha = Alpha::try_from_indices(Sign::Pos, &indices).unwrap();
                    terms.push(Term::new(None, alpha));
                }
            }
            MultiVector::from_terms(terms)
        }
    }
);

// helper for defining common operators
macro_rules! __default_differential_operator_impl(
    ($name:ident $alpha_str:expr) => {
        pub fn $name() -> ArDifferential {
            let mut alphas = vec![];
            for s in $alpha_str.split_whitespace() {
                if s == "p" {
                    alphas.push(Alpha::new(Sign::Pos, Form::Point).unwrap())
                } else {
                    let indices = s.chars().map(|c|
                        match c {
                            '0' => Index::Zero,
                            '1' => Index::One,
                            '2' => Index::Two,
                            '3' => Index::Three,
                            _ => panic!("invalid index {}", c)
                        }
                    ).collect();
                    alphas.push(Alpha::try_from_indices(Sign::Pos, &indices).unwrap());
                }
            }
            ArDifferential::new(&alphas)
        }
    }
);

// snippets for building multivectors
const _p: &'static str = "p";
const _t: &'static str = "0";
const _h: &'static str = "123";
const _q: &'static str = "0123";
const _B: &'static str = "23 31 12";
const _T: &'static str = "023 031 012";
const _A: &'static str = "1 2 3";
const _E: &'static str = "01 02 03";

// Common MultiVectors
__default_mvec_impl!(G ALLOWED_ALPHA_STRINGS.join(" "));
__default_mvec_impl!(Fields [_B, _E].join(" "));
__default_mvec_impl!(Even_sub_algebra [_p, _B, _q, _E].join(" "));
__default_mvec_impl!(Odd_sub_algebra [_t, _T, _h, _A].join(" "));

__default_mvec_impl!(B "23 31 12");
__default_mvec_impl!(T "023 031 012");
__default_mvec_impl!(A "1 2 3");
__default_mvec_impl!(E "01 02 03");

__default_mvec_impl!(Zet_B "p 23 31 12");
__default_mvec_impl!(Zet_T "0 023 031 012");
__default_mvec_impl!(Zet_A "123 1 2 3");
__default_mvec_impl!(Zet_E "0123 01 02 03");

// Common Differential Operators
__default_differential_operator_impl!(Dmu "0 1 2 3");
__default_differential_operator_impl!(DG ALLOWED_ALPHA_STRINGS.join(" "));
