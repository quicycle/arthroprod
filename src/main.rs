extern crate arthroprod;
use arthroprod::ops;
use arthroprod::types::{Alpha, Sign};
use arthroprod::utils;


fn main() {
    let allowed = utils::get_allowed();
    let metric = utils::get_metric();
    let targets = utils::get_targets(&allowed);

    // Make some Alphas
    let p = Alpha::new("p", Sign::Pos, &allowed).unwrap();
    let ab = Alpha::new("0", Sign::Pos, &allowed).unwrap();
    let ex = Alpha::new("10", Sign::Neg, &allowed).unwrap();
    let ty = Alpha::new("031", Sign::Pos, &allowed).unwrap();
    let bz = Alpha::new("12", Sign::Neg, &allowed).unwrap();
    let q = Alpha::new("0123", Sign::Neg, &allowed).unwrap();

    // Find some products
    println!("Let's see if this pile of crazy works!...");
    let ans = ops::find_prod(&bz, &p, &metric, &targets);
    println!("{} ^ {} = {}", bz, p, ans);
    let ans = ops::find_prod(&ab, &q, &metric, &targets);
    println!("{} ^ {} = {}", ab, q, ans);
    let ans = ops::find_prod(&ty, &ty, &metric, &targets);
    println!("{} ^ {} = {}", ty, ty, ans);
    let ans = ops::find_prod(&q, &ex, &metric, &targets);
    println!("{} ^ {} = {}", q, ex, ans);
    println!("Boo (as they say) yah!");
}
