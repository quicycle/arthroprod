extern crate arthroprod;
use arthroprod::consts::{ALLOWED, METRIC, TARGETS};
use arthroprod::ops;
use arthroprod::types::{Alpha, Sign};
// use arthroprod::utils;


fn main() {
    // Make some Alphas
    let p = Alpha::new("p", Sign::Pos, &ALLOWED).unwrap();
    let ab = Alpha::new("0", Sign::Pos, &ALLOWED).unwrap();
    let ex = Alpha::new("10", Sign::Neg, &ALLOWED).unwrap();
    let ty = Alpha::new("031", Sign::Pos, &ALLOWED).unwrap();
    let bz = Alpha::new("12", Sign::Neg, &ALLOWED).unwrap();
    let q = Alpha::new("0123", Sign::Neg, &ALLOWED).unwrap();

    // Find some products
    println!("Let's see if this pile of crazy works!...");
    let ans = ops::find_prod(&bz, &p, &METRIC, &TARGETS);
    println!("{} ^ {} = {}", bz, p, ans);
    let ans = ops::find_prod(&ab, &q, &METRIC, &TARGETS);
    println!("{} ^ {} = {}", ab, q, ans);
    let ans = ops::find_prod(&ty, &ty, &METRIC, &TARGETS);
    println!("{} ^ {} = {}", ty, ty, ans);
    let ans = ops::find_prod(&q, &ex, &METRIC, &TARGETS);
    println!("{} ^ {} = {}", q, ex, ans);
    println!("Boo (as they say) yah!");
}
