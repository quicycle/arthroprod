use std::collections::HashMap;
use std::fmt;

use crate::algebra::{Component, Term, ALLOWED_ALPHA_COMPONENTS, AR};

#[derive(Debug, PartialEq, Clone)]
pub struct MultiVector {
    terms: Vec<Term>,
}

impl AR for MultiVector {
    fn as_terms(&self) -> Vec<Term> {
        self.terms.clone()
    }
}

impl fmt::Display for MultiVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = ALLOWED_ALPHA_COMPONENTS
            .iter()
            .map(|c| {
                let for_comp = self.get(c);
                if for_comp.len() > 0 {
                    Some(format!("{}),", {
                        let mut vec_str = for_comp
                            .iter()
                            .fold(String::from(format!("  a{}: (", c)), |s, val| {
                                format!("{}{}, ", s, val.xi())
                            });
                        let desired_len = vec_str.len() - 2;
                        vec_str.split_off(desired_len);
                        vec_str
                    }))
                } else {
                    None
                }
            })
            .filter_map(|s| s)
            .fold(String::from(""), |s, line| format!("{}\n{}", s, line));

        write!(f, "{{{}\n}}", s)
    }
}

impl MultiVector {
    pub fn new() -> MultiVector {
        MultiVector { terms: vec![] }
    }

    pub fn from_terms(terms: Vec<Term>) -> MultiVector {
        MultiVector { terms }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    pub fn get(&self, c: &Component) -> Vec<Term> {
        self.terms
            .iter()
            .filter(|t| &t.alpha().component() == c)
            .map(|t| t.clone())
            .collect()
    }

    pub fn cancel_terms(&mut self) {
        let mut seen: HashMap<Term, Vec<Term>> = HashMap::new();

        self.terms.sort();

        self.terms.iter().for_each(|t| {
            let neg = -(t.clone());
            if let Some(current) = seen.get_mut(&neg) {
                if current.len() == 1 {
                    seen.remove(&t);
                }
            } else {
                if let Some(current) = seen.get_mut(&t) {
                    current.push(t.clone());
                } else {
                    seen.insert(t.clone(), vec![t.clone()]);
                }
            };
        });

        self.terms = seen.drain().map(|(_, v)| v).flatten().collect()
    }
}
