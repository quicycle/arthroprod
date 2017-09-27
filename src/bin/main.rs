extern crate arthroprod;
extern crate getopts;

use arthroprod::config;
use arthroprod::ops;
use arthroprod::types::Alpha;
use getopts::Options;
use std::env;
use std::io::{self, Write};
use std::process;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run(cfg: config::Config) -> Result<(), &'static str> {
    loop {
        print!("\n>>> ");
        io::stdout().flush().unwrap();

        // Read the user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let alphas: Vec<&str> = input.split_whitespace().collect();
        if alphas.len() != 2 {
            println!("\nMust provide two alpha indices");
            continue;
        }

        let a1 = match Alpha::new(alphas[0]) {
            Ok(a) => a,
            Err(e) => {
                println!("\n{}", e);
                continue;
            }
        };
        let a2 = match Alpha::new(alphas[1]) {
            Ok(a) => a,
            Err(e) => {
                println!("\n{}", e);
                continue;
            }
        };
        let res = ops::find_prod_override(&a1, &a2, &cfg.metric, &cfg.allowed);
        println!("{} ^ {} = {}", a1, a2, res);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "config", "provide a custom config to use.", "CONFIG");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Handle help text
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Handle config override
    let cfg = match matches.opt_str("c") {
        Some(c) => {
            config::Config::new(c).unwrap_or_else(|err| {
                eprintln!("Problem parsing args: {}", err);
                process::exit(1);
            })
        }
        None => config::Config::new_default(),
    };

    if let Err(e) = run(cfg) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
