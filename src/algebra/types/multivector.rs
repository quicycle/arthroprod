use std::collections::HashMap;
use std::fmt;

use crate::algebra::{Component, Term, ALLOWED_ALPHA_COMPONENTS};

#[derive(Debug, PartialEq, Clone)]
pub struct MultiVector {
    components: HashMap<Component, Vec<Term>>,
}

impl fmt::Display for MultiVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = ALLOWED_ALPHA_COMPONENTS
            .iter()
            .map(|c| match &self.components.get(c) {
                Some(vec) => Some(format!("{}),", {
                    let mut vec_str = vec
                        .iter()
                        .fold(String::from(format!("  a{}: (", c)), |s, val| {
                            format!("{}{}, ", s, val.xi())
                        });
                    let desired_len = vec_str.len() - 2;
                    vec_str.split_off(desired_len);
                    vec_str
                })),
                None => None,
            })
            .filter_map(|s| s)
            .fold(String::from(""), |s, line| format!("{}\n{}", s, line));

        write!(f, "{{{}\n}}", s)
    }
}

impl MultiVector {
    pub fn new() -> MultiVector {
        let components = HashMap::new();

        MultiVector { components }
    }

    pub fn from_terms(terms: Vec<Term>) -> MultiVector {
        let mut components = HashMap::new();

        terms.iter().for_each(|t| {
            let comp = t.alpha().component();
            let current_comps = components.entry(comp).or_insert(vec![]);
            current_comps.push(t.clone());
        });

        MultiVector { components }
    }

    pub fn add_term(&mut self, term: Term) {
        let comp = term.alpha().component();
        let current_comps = self.components.entry(comp).or_insert(vec![]);
        current_comps.push(term);
    }

    pub fn get(&self, c: &Component) -> Vec<Term> {
        let default = Vec::<Term>::new();
        self.components.get(c).unwrap_or(&default).to_vec()
    }
}
