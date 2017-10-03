//! Parsing for calculation files

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use super::error::ArError;


pub struct Calculation {
    contents: String,
    fname: String,
}


impl Calculation {
    pub fn new(fname: String) -> Result<Calculation, Box<Error>> {
        let mut f = File::open(fname.clone())?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        Ok(Calculation { contents, fname })
    }

    pub fn parse(&mut self) -> Result<(), ArError> {
        println!("File Name: {}\nContents:\n{}\n\n", self.fname, self.contents);
        Err(ArError::InvalidCalcFile(String::from("NOT IMPLEMENTED")))
    }
}
