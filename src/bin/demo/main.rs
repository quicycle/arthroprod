#[macro_use]
extern crate arthroprod;

use arthroprod::algebra::*;

fn main() {
    let a1 = alpha!(0 2 3);
    let a2 = -alpha!(0 3);

    println!("{:?} -> {}", a1, a1);
    println!("{:?} -> {}", a2, a2);

    let res = full(&a1, &a2);
    println!("{:?}", res);
    println!("result = {}", res);

    println!("{}", -term!(1 2 3));
}
