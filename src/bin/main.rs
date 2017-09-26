extern crate arthroprod;
use arthroprod::ops;
use arthroprod::types::Alpha;


fn main() {
    let a1 = Alpha::new("31");
    let a2 = Alpha::new("01");
    println!("{} ^ {} = {}", a1, a2, ops::find_prod(&a1, &a2));

    let a3 = Alpha::new("1");
    let a4 = Alpha::new("2");
    println!("{} ^ {} = {}", a3, a4, ops::find_prod(&a3, &a4));
    println!("{} ^ {} = {}", a4, a3, ops::find_prod(&a4, &a3));
}
