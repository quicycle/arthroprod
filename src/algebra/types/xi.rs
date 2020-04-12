use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::algebra::Form;

// Only leaf nodes should have values
#[derive(Hash, Eq, Debug, PartialOrd, PartialEq, Clone, Ord, Serialize, Deserialize)]
pub struct Xi {
    value: Option<String>,
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
        self.value == None && self.children.len() == 0
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
        merge_xis(xis)
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

// Called recursively to merge together multiple Xi values based on parent nodes
fn merge_xis(xis: &Vec<Xi>) -> Xi {
    let mut groups: HashMap<(Option<String>, String), Vec<Xi>> = HashMap::new();
    xis.iter().cloned().for_each(|c| {
        groups
            .entry((c.value.clone(), partial_str(&c.partials)))
            .or_insert(vec![])
            .push(c);
    });

    if groups.len() <= xis.len() {
        // If there are no common parent nodes then we are done
        let mut children: Vec<Xi> = xis.iter().filter(|x| !x.is_empty()).cloned().collect();
        children.sort();

        Xi {
            value: None,
            partials: Vec::new(),
            children: children,
        }
    } else {
        // Merge the nodes we have so far and then recurse to see if there
        // is any matching parents at the next level down
        merge_xis(
            &groups
                .drain()
                .flat_map(|(_, v)| v)
                .flat_map(|c| c.children)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        vec![Xi::new("foo"), Xi::new("bar")],
        Xi {
            value: None,
            partials: vec![],
            children: vec![Xi::new("bar"), Xi::new("foo")],
        }
    )]
    #[test_case(
        vec![Xi::new("foo"), Xi::new("baz"), Xi::new("bar")],
        Xi {
            value: None,
            partials: vec![],
            children: vec![Xi::new("bar"), Xi::new("baz"), Xi::new("foo")],
        }
    )]
    #[test_case(
        vec![Xi::new("foo"), Xi::new("foo")],
        Xi {
            value: None,
            partials: vec![],
            children: vec![Xi::new("foo"), Xi::new("foo")],
        }
    )]
    fn merge_with_no_matching_parents_works(xis: Vec<Xi>, expected: Xi) {
        assert_eq!(Xi::merge(&xis), expected);
    }
}
