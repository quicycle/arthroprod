#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::{full, Alpha, Axis, Form, MultiVector, Sign, Term, AR};

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
    let a1 = alpha!(0 2 3);
    let a2 = -alpha!(0 3);
    let res1: Alpha = full(&a1, &a2);

    println!("a1 = {}", a1);
    println!("a2 = {}", a2);
    println!("a1 ^ a2 = {}\n", res1);

    println!("-term!(1 2 3) -> {}", -term!(1 2 3));
    println!("term!(\"foo\", 1 2 3) -> {}\n", term!("foo", 1 2 3));

    let m = mvec![term!(0 1 2 3), term!(2 3), term!(3 1), term!(1 2)];
    let mut mres: MultiVector = full(&m, &double_dagger(&m));
    mres.simplify();

    println!("m = {}", m);
    println!("m ^ m! = {}", mres);
}
