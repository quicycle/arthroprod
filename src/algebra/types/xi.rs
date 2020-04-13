use std::fmt;
use std::ops;

use crate::algebra::Form;

#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
pub struct Xi {
    value: Option<String>, // None for non-leaf nodes
    partials: Vec<Form>,
    children: Vec<Xi>,
}

impl Xi {
    /// Construct a new symbolic Xi value
    pub fn new(value: &str) -> Xi {
        Xi {
            value: Some(value.to_string()),
            partials: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Xi {
        Xi {
            value: None,
            partials: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value == None && self.children.len() == 0 && self.partials.len() == 0
    }

    /// Add a single partial derivative to this Xi
    pub fn add_partial(&mut self, wrt: &Form) {
        self.partials.push(*wrt);
        self.partials.sort();
    }

    /// Replace the current set of partial derivatives
    pub fn set_partials(&mut self, partials: Vec<Form>) {
        self.partials = partials;
        self.partials.sort();
    }

    /// Construct a new Xi by forming the product of existing Xis
    pub fn merge(xis: &Vec<Xi>) -> Xi {
        // pull up children from empty_parents so that we don't stack empty
        // nodes on top of one another
        let mut children: Vec<Xi> = xis
            .iter()
            .flat_map(|x| {
                if x.is_empty_parent() {
                    x.children.clone()
                } else {
                    vec![x.clone()]
                }
            })
            .collect();
        children.sort();

        Xi {
            value: None,
            partials: Vec::new(),
            children: children,
        }
    }

    /// Represent this Xi as a dotted string of terms
    pub fn dotted_string(&self) -> String {
        let partials = partial_str(&self.partials);
        let with_partials = |s: String| -> String {
            match partials.len() {
                0 => s,
                _ => format!("{}({})", partials, s),
            }
        };

        match self.value.clone() {
            Some(val) => format!("{}ξ{}", partials, val),
            None => match self.children.len() {
                0 => panic!("Empty Xi"),
                1 => with_partials(self.children[0].dotted_string()),
                _ => {
                    let s = self.children[1..self.children.len()]
                        .iter()
                        .fold(format!("{}", self.children[0].dotted_string()), |acc, x| {
                            format!("{}.{}", acc, x.dotted_string())
                        });
                    with_partials(s)
                }
            },
        }
    }

    fn is_empty_parent(&self) -> bool {
        self.value == None && self.partials.len() == 0
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dotted_string())
    }
}

impl ops::Mul for Xi {
    type Output = Self;

    fn mul(self, rhs: Xi) -> Self::Output {
        Xi::merge(&vec![self, rhs])
    }
}

// Concatenate the forms represeneting the partial derivatives applied to this Xi
pub(super) fn partial_str(partials: &Vec<Form>) -> String {
    partials
        .iter()
        .fold(String::new(), |acc, p| acc + &format!("∂{}", p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        vec![Xi::new("foo"), Xi::new("bar")],
        Xi { value: None, partials: vec![], children: vec![Xi::new("bar"), Xi::new("foo")] }
    )]
    #[test_case(
        vec![Xi::new("foo"), Xi::new("baz"), Xi::new("bar")],
        Xi { value: None, partials: vec![], children: vec![Xi::new("bar"), Xi::new("baz"), Xi::new("foo")] }
    )]
    #[test_case(
        vec![Xi::new("foo"), Xi::new("foo")],
        Xi { value: None, partials: vec![], children: vec![Xi::new("foo"), Xi::new("foo")] }
    )]
    #[test_case(vec![Xi::empty(), Xi::empty()], Xi::empty())]
    fn merge_with_no_top_level_empty_works(xis: Vec<Xi>, expected: Xi) {
        assert_eq!(Xi::merge(&xis), expected);
    }

    #[test_case(
        vec![
            Xi { value: None, partials: vec![], children: vec![Xi::new("foo")] },
            Xi { value: None, partials: vec![], children: vec![Xi::new("bar")] },
        ],
        Xi { value: None, partials: vec![], children: vec![Xi::new("bar"), Xi::new("foo")] }
    )]
    #[test_case(
        vec![
            Xi { value: None, partials: vec![], children: vec![Xi::new("foo")] },
            Xi::merge(&vec![Xi::new("bar"), Xi::new("baz")]),
        ],
        Xi { value: None, partials: vec![], children: vec![Xi::new("bar"), Xi::new("baz"), Xi::new("foo")] }
    )]
    fn merge_with_empty_works(xis: Vec<Xi>, expected: Xi) {
        assert_eq!(Xi::merge(&xis), expected);
    }
}
