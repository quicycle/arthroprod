#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::{full, Form, MultiVector, AR};
use arthroprod::prelude::*;

// negate everything but the bivectors
fn double_dagger<T: AR>(m: &T) -> MultiVector {
    MultiVector::from_terms(
        m.as_terms()
            .iter()
            .map(|t| match t.alpha().form() {
                Form::Bivector(_, _) => t.clone(),
                _ => -t.clone(),
            })
            .collect(),
    )
}

fn main() {
    let m = mvec![term!(0 1 2 3), term!(2 3), term!(3 1), term!(1 2)];
    let mut mres: MultiVector = full(&m, &double_dagger(&m));
    mres.simplify();

    println!("m = {}", m);
    println!("m ^ m_ddagger = {}\n", mres);

    println!("Dmu G = {}", Dmu().left_apply(G()));
}
