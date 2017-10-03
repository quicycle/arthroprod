extern crate arthroprod;
extern crate getopts;

use std::env;
use std::io::{self, Write};
use std::process;

use getopts::Options;

use arthroprod::algebra;
use arthroprod::calcfile;
use arthroprod::types::*;



fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn repl() -> Result<(), &'static str> {
    loop {
        print!("\n>>> ");
        io::stdout().flush().unwrap();

        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(
            "Failed to read input",
        );

        let alphas: Vec<&str> = input.split_whitespace().collect();
        if alphas.len() != 2 {
            println!("\nMust provide two alpha indices: e.g. 'a12 a023'");
            continue;
        }

        let a1 = match Alpha::new(&alphas[0][1..]) {
            Ok(a) => a,
            Err(e) => {
                println!("\n{}", e);
                continue;
            }
        };
        let a2 = match Alpha::new(&alphas[1][1..]) {
            Ok(a) => a,
            Err(e) => {
                println!("\n{}", e);
                continue;
            }
        };
        let res = algebra::full_product(&a1, &a2);
        println!("{} ^ {} = {}", a1, a2, res);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        "provide a calculation file to run.",
        "CALC-FILE",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    // Handle help text
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // See if there is a calculation file to run otherwise start the repl.
    if matches.opt_present("f") {
        let f = matches.opt_str("f").expect("unreachable");
        let mut calc_file = calcfile::Calculation::new(f).unwrap_or_else(|err| {
            eprintln!("Problem parsing calculation file: {}", err);
            process::exit(1);
        });
        if let Err(e) = calc_file.parse() {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = repl() {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
