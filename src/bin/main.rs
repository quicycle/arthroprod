extern crate clap;
extern crate arthroprod;
use clap::App;

use arthroprod::ops;
use arthroprod::types::Alpha;


fn main() {
    // See https://docs.rs/clap/2.26.2/clap/ for details
    let matches = App::new("ar")
        .version("0.1.3")
        .author("Innes Anderson-Morrison. <innesdmorrison@gmail.com>")
        .about(
            "Clifford Algebra based computation for the theory of Absolute Relativity.
At present this is a simple calculator for multiplying two alpha values.",
        )
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
            <ALPHA1>             'First alpha'
            <ALPHA2>             'Second alpha'",
        )
        .get_matches();

    // We're safe to use unwrap here because these are required arguments.
    // Otherwise we need to use `if let` or `unwrap_or`
    let a1 = matches.value_of("ALPHA1").unwrap();
    let a2 = matches.value_of("ALPHA2").unwrap();


    let a1 = Alpha::new(a1);
    let a2 = Alpha::new(a2);
    println!("{} ^ {} = {}", a1, a2, ops::find_prod(&a1, &a2));
}
