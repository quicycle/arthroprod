use std::collections::HashMap;
use std::fmt;

use super::component::*;
use super::pair::*;
use super::super::config::Allowed;
use super::super::consts::{ALLOWED, ALPHAS};
use {ArError, Result};

#[derive(Debug, PartialEq, Clone)]
/// A Multivector containing elements of mixed dimensionality.
///
/// Multivectors are the primary API for working with arthroprod.
///
/// TODO: from Vec<&str>, from String, Iterator
pub struct Mvec {
    components: HashMap<Component, Vec<Pair>>,
    allowed: Allowed,
    order: Vec<Component>,
}


impl fmt::Display for Mvec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comps = &self.components;

        let s = self.order
            .iter()
            .map(|c| match comps.get(c) {
                Some(vec) => {
                    Some(format!("{}),", {
                        let mut vec_str = vec.iter().fold(
                            String::from(format!("  α{}: (", c)),
                            |s, val| format!("{}{}, ", s, val.xi()),
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


impl Mvec {
    /// Create a new MultiVector with the default configuration.
    pub fn new() -> Mvec {
        let components = HashMap::new();
        let order = ALPHAS
            .iter()
            .map(|a| Component::unsafe_new(a).unwrap())
            .collect();
        let allowed = ALLOWED.clone();
        Mvec {
            components,
            allowed,
            order,
        }
    }

    /// Add an element to the multivector
    fn add_element(&mut self, comp: Component, pair: Pair) {
        let current_comps = self.components.entry(comp).or_insert(vec![]);
        current_comps.push(pair);

    }

    /// Add a symbolic element to the multivector.
    pub fn add_string(&mut self, s: &str) -> Result<()> {
        let raw_ix = s.trim_matches('-');
        let comp = Component::new_override(raw_ix, self.allowed.indices())?;
        let pair = Pair::sym(s)?;
        self.add_element(comp, pair);
        Ok(())
    }

    /// Add an existing pair to the multivector.
    pub fn add_pair(&mut self, pair: Pair) -> Result<()> {
        let comp = pair.alpha().comp();

        if !self.allowed.indices().contains(&comp) {
            return Err(ArError::ComponentNotAllowed(String::from("Invalid component for Multivector")));
        }

        self.add_element(comp.clone(), pair.clone());
        Ok(())
    }

    /// get the values from the multivector that correspond to this component
    pub fn get(&self, c: &Component) -> Option<&Vec<Pair>> {
        self.components.get(c)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_string() {
        let mut m = Mvec::new();
        m.add_string("01").unwrap();
        println!("\nmvec = {}\n", m);

        let expected = vec![Pair::sym("01").unwrap()];
        let key = Component::new("01").unwrap();
        assert_eq!(m.get(&key), Some(&expected));
    }
}
