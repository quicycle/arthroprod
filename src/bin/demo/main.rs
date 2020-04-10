#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::*;

fn main() {
    let a1 = alpha!(0 2 3);
    let a2 = -alpha!(0 3);

    println!("{:?} -> {}", a1, a1);
    println!("{:?} -> {}", a2, a2);

    println!("\n[products]\n");
    let res1: Alpha = full(&a1, &a2);
    println!("{:?}", res1);
    println!("result = {}\n", res1);

    let res2: MultiVector = full(&a1, &a2);
    println!("{:?}", res2);
    println!("result = {}", res2);

    println!("{}", -term!(1 2 3));
}
