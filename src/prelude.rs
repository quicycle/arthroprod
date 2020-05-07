//! Common data structures and operators used in the algebra
#![allow(non_snake_case, non_upper_case_globals)]

use crate::algebra::types::ALLOWED_ALPHA_STRINGS;
use crate::algebra::ArDifferential;
use crate::algebra::{Alpha, Form, Index, MultiVector, Sign, Term, AR};

macro_rules! __default_mvec_impls(
    {
        $(
            $(#[$outer:meta])*
            $name:ident => $term_str:expr
        ),+
    } => {
        $(
            $(#[$outer])*
            pub fn $name() -> MultiVector {
                let mut terms = vec![];
                for s in $term_str.split_whitespace() {
                    if s == "p" {
                        terms.push(
                            Term::new(None, Alpha::new(Sign::Pos, Form::Point).unwrap())
                        );
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
        )+
    }
);

// helper for defining common operators
macro_rules! __default_differential_operator_impls(
    {
        $(
            $(#[$outer:meta])*
            $name:ident => $alpha_str:expr
        ),+
    } => {

        $(
            $(#[$outer])*
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
        )+
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

__default_mvec_impls! {
    /// The general multivector containing all 16 elements of the algebra
    G => ALLOWED_ALPHA_STRINGS.join(" "),

    /// The magnetic and electric fields
    Fields => [_B, _E].join(" "),

    /// The even sub algebra: bivector fields plus root-mass terms ap and a0123
    Even_sub_algebra => [_p, _B, _q, _E].join(" "),

    /// The odd sub algebra: vectors and trivectors
    Odd_sub_algebra => [_t, _T, _h, _A].join(" "),

    /// The magnetic field: space-space bivectors
    B => "23 31 12",

    /// The spin elements: time-space-space trivectors
    T => "023 031 012",

    /// The space basis vectors
    A => "1 2 3",

    /// The electric field: time-space bivectors
    E => "01 02 03",

    /// ζB : pivot plus magnetic
    Zet_B => "p 23 31 12",

    /// ζT : time plus spin
    Zet_T => "0 023 031 012",

    /// ζA : hedgehog plus space
    Zet_A => "123 1 2 3",

    /// ζE : dual-pivot plus electric
    Zet_E => "0123 01 02 03"
}

__default_differential_operator_impls! {
    /// Dμ : differentiate with respect to space and time
    Dmu => "0 1 2 3",

    /// DG : differentiate with respect to all 16 elements of the algebra
    DG => ALLOWED_ALPHA_STRINGS.join(" ")
}
