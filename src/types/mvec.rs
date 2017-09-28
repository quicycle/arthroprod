use std::collections::HashMap;
use std::fmt;

use super::component::*;
use super::pair::*;
use super::super::config::Allowed;
use super::super::consts::{ALLOWED, ALPHAS};
use super::xi::*;
use {ArError, Result};

#[derive(Debug, PartialEq, Clone)]
/// A Multivector containing elements of mixed dimensionality.
///
/// Multivectors are the primary API for working with arthroprod.
pub struct Mvec<'a> {
    components: HashMap<Component, Vec<Xi>>,
    allowed: &'a Allowed,
    order: Vec<Component>,
}


impl<'a> fmt::Display for Mvec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comps = &self.components;

        let s = self.order
            .iter()
            .map(|c| match comps.get(c) {
                Some(vec) => {
                    Some(format!("{}),", {
                        let mut vec_str = vec.iter().fold(
                            String::from(format!("  α{}: (", c)),
                            |s, val| format!("{}{}, ", s, val),
                        );
                        let desired_len = vec_str.len() - 2;
                        vec_str.split_off(desired_len);
                        vec_str
                    }))
                }
                None => None,
            })
            .filter_map(|s| s)
            .fold(String::from(""), |s, line| format!("{}\n{}", s, line));
        write!(f, "{{{}\n}}", s)
    }
}


impl<'a> Mvec<'a> {
    /// Create a new MultiVector with the default configuration.
    pub fn new() -> Mvec<'a> {
        let components = HashMap::new();
        let order = ALPHAS
            .iter()
            .map(|a| Component::unsafe_new(a).unwrap())
            .collect();
        let allowed = &ALLOWED;
        Mvec {
            components,
            allowed,
            order,
        }
    }

    /// Add an element to the multivector
    fn add_element(&mut self, comp: Component, xi: Xi) {
        let current_comps = self.components.entry(comp).or_insert(vec![]);
        current_comps.push(xi);

    }

    /// Add a symbolic element to the multivector.
    pub fn add_string(&mut self, s: &str) -> Result<()> {
        let comp = Component::new(s, self.allowed.indices())?;
        let xi = Xi::Symbolic(String::from(s));
        self.add_element(comp, xi);
        Ok(())
    }

    /// Add an existing pair to the multivector.
    pub fn add_pair(&mut self, p: Pair) -> Result<()> {
        let xi = p.xi();
        let comp = p.alpha().index();

        if !self.allowed.indices().contains(&comp) {
            return Err(ArError::ComponentNotAllowed(
                String::from("Invalid component for Multivector"),
            ));
        }

        self.add_element(comp.clone(), xi.clone());
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_string() {
        let mut m = Mvec::new();
        m.add_string("123").unwrap();
        m.add_string("01").unwrap();
        println!("\nmvec = {}\n", m);
    }
}
