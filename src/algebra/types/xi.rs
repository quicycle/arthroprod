use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::algebra::types::alpha::ALLOWED_ALPHA_STRINGS;
use crate::algebra::Form;

#[derive(Hash, Eq, Debug, PartialEq, Clone, Serialize, Deserialize)]
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
        let mut children = vec![];
        for x in xis.iter() {
            if x.is_empty_parent() {
                children.extend(x.children.clone());
            } else {
                children.push(x.clone());
            }
        }
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

        let power_notation = |children: &Vec<Xi>| -> String {
            let exp_str = |(xi, count): (Xi, usize)| -> String {
                if count == 1 {
                    xi.dotted_string()
                } else {
                    format!("{}^{}", xi.dotted_string(), count)
                }
            };

            let mut groups: HashMap<Xi, Vec<Xi>> = HashMap::new();
            children.iter().cloned().for_each(|c| {
                groups.entry(c.clone()).or_insert(vec![]).push(c);
            });

            let mut powers: Vec<(Xi, usize)> = groups.drain().map(|(k, v)| (k, v.len())).collect();
            powers.sort_by(|a, b| a.0.cmp(&b.0));
            powers[1..powers.len()]
                .iter()
                .fold(exp_str(powers[0].clone()), |acc, pair| {
                    format!("{}.{}", acc, exp_str(pair.clone()))
                })
        };

        match self.value.clone() {
            Some(val) => format!("{}ξ{}", partials, val),
            None => match self.children.len() {
                0 => panic!("Empty Xi"),
                1 => with_partials(self.children[0].dotted_string()),
                _ => with_partials(power_notation(&self.children)),
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

impl cmp::Ord for Xi {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        fn cmp_ix(l: &String, r: &String) -> cmp::Ordering {
            let opt_i1 = ALLOWED_ALPHA_STRINGS.iter().position(|f| f == &l);
            let opt_i2 = ALLOWED_ALPHA_STRINGS.iter().position(|f| f == &r);

            // Compare indices if both are present, otherwise place indexed
            // first and non-indexed in alphanumeric ordering after
            match (opt_i1, opt_i2) {
                (Some(i1), Some(i2)) => i1.cmp(&i2),
                (Some(_), None) => cmp::Ordering::Less,
                (None, Some(_)) => cmp::Ordering::Greater,
                (None, None) => l.cmp(&r),
            }
        }

        match (self.children.len(), other.children.len()) {
            // Should be leaf nodes with a value
            (0, 0) => match (&self.value, &other.value) {
                (Some(l), Some(r)) => cmp_ix(l, r).then(self.partials.cmp(&other.partials)),
                _ => self.partials.cmp(&other.partials),
            },
            _ => self.children.cmp(&other.children),
        }
    }
}

impl cmp::PartialOrd for Xi {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
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

    #[test_case(
        vec![
            Xi { value: Some("foo".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
            Xi { value: Some("bar".to_string()), partials: vec![], children: vec![] },
        ],
        Xi {
            value: None,
            partials: vec![],
            children: vec![
                Xi { value: Some("bar".to_string()), partials: vec![], children: vec![] },
                Xi { value: Some("foo".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
            ]
        }
    )]
    #[test_case(
        vec![
            Xi { value: Some("foo".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
            Xi { value: Some("baz".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
            Xi { value: Some("bar".to_string()), partials: vec![], children: vec![] },
        ],
        Xi {
            value: None,
            partials: vec![],
            children: vec![
                Xi { value: Some("bar".to_string()), partials: vec![], children: vec![] },
                Xi { value: Some("baz".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
                Xi { value: Some("foo".to_string()), partials: vec![alpha!(0).form()], children: vec![] },
            ]
        }
    )]
    fn merge_with_top_level_partials_works(xis: Vec<Xi>, expected: Xi) {
        assert_eq!(Xi::merge(&xis), expected);
    }

    #[test_case(
        vec![
            Xi { value: None, partials: vec![], children: vec![Xi::new("foo")] },
            Xi {
                value: None,
                partials: vec![],
                children: vec![
                    Xi {
                        value: Some("bar".to_string()),
                        partials: vec![alpha!(0).form()],
                        children: vec![]
                    }
                ]
            },
        ],
        Xi {
            value: None,
            partials: vec![],
            children: vec![
                Xi {
                    value: Some("bar".to_string()),
                    partials: vec![alpha!(0).form()],
                    children: vec![]
                },
                Xi::new("foo"),
            ]
        }
    )]
    fn merge_with_partials_on_children_works(xis: Vec<Xi>, expected: Xi) {
        assert_eq!(Xi::merge(&xis), expected);
    }
}
