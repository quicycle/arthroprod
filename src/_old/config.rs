//! Configuration data structures used in the rest of arthroprod.

use super::consts::{ALPHAS, DEFAULT_METRIC_SIGNATURE};
use super::types::{Alpha, Component, Index, KeyVec, Sign};
use {ArError, Result};
use std::collections::{HashMap, HashSet};


/// A configuration to run calculations under.
pub struct Config {
    pub allowed: Allowed,
    pub metric: HashMap<Index, Sign>,
}


impl Config {
    /// Create a new default config.
    pub fn new_default() -> Config {
        let allowed = Allowed::from_vec(ALPHAS.to_vec()).expect("!!!");
        let metric = metric_from_string(DEFAULT_METRIC_SIGNATURE).expect("!!!");
        Config { allowed, metric }
    }

    /// Create a new config override.
    ///
    /// NOT CURRENTLY IMPLEMENTED!!! (This will just return the default config)
    pub fn new(fname: String) -> Result<Config> {
        eprintln!("Not loading config from {}: custom config is not implemented yet!", fname);
        Ok(Config::new_default())
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
/// The base elements of the algebra.
///
/// A valid Allowed instance contains 16 Alpha elements that form the algebra
/// used to compute within Absolute Relativity. More specifically there are:
///
///   * One zero dimensional scalar.
///
///   * Four vectors (one of time and three of space)
///
///   * Six bivectors (three space-space and three space-time)
///
///   * Four trivectors (one space-space-space and three time-space-space)
///
///   * One quadrivector (all four of the vector elements)
///
/// Any custom Allowed instance used in calculations must be passed to the
/// `override` functions and methods in order to maintain consistency in
/// calculations. The program will panic if inconsistent Alphas are found.
pub struct Allowed {
    elems: HashSet<Component>,
    targets: HashMap<KeyVec, Component>,
}


impl Allowed {
    /// Create a new copy of the default Allowed instance.
    pub fn default() -> Allowed {
        Allowed::from_vec(ALPHAS.to_vec()).expect(
            "The default ALPHAS are not valid!",
        )
    }

    /// Parse a vector of string indices into a new ALLOWED instance.
    ///
    /// The vector of strings provided will be checked for some (but not all)
    /// possible user mistakes in defining a set of Alpha values to work with.
    /// Some flexibility is left because the very nature of this feature is to
    /// allow the user to tinker with how the alagebra works.
    ///
    /// # Examples
    /// ```
    /// use arthroprod::config::Allowed;
    ///
    /// let indices = vec!["p", "23", "31", "12", "0", "023", "031", "012",
    /// "123", "1", "2", "3", "0123", "01", "02", "03"];
    ///
    /// match Allowed::from_vec(indices) {
    ///     Ok(a) => a,
    ///     Err(_) => panic!("Oops!"),
    /// };
    /// ```
    pub fn from_vec(indices: Vec<&str>) -> Result<Allowed> {
        // Validate that this looks like a possible config value for ALLOWED in terms
        // of number of indices and correct number of components for each order.
        let mut point = 0;
        let mut vectors = 0;
        let mut bivectors = 0;
        let mut trivectors = 0;
        let mut quadrivector = 0;

        for i in indices.iter() {
            match i.len() {
                1 => {
                    if i == &"p" {
                        point = point + 1;
                    } else {
                        vectors = vectors + 1
                    }
                }
                2 => bivectors = bivectors + 1,
                3 => trivectors = trivectors + 1,
                4 => quadrivector = quadrivector + 1,
                _ => return Err(ArError::InvalidConfig(String::from("Invalid index in ALLOWED"))),
            }
        }

        let expected = [
            ("αp instances", point, 1),
            ("vectors", vectors, 4),
            ("bivectors", bivectors, 6),
            ("trivectors", trivectors, 4),
            ("quadrivectors", quadrivector, 1),
        ];

        for case in expected.iter() {
            let (name, have, want) = *case;
            if have != want {
                return Err(ArError::InvalidConfig(String::from(
                    format!("ALLOWED contained wrong number of {}: {} != {}", name, have, want),
                )));
            }
        }

        let mut elems = HashSet::new();
        for i in indices {
            let comp = Component::unsafe_new(i)?;
            elems.insert(comp);
        }

        let mut targets = HashMap::new();
        for e in elems.iter() {
            if *e != Component::Point {
                targets.insert(KeyVec::new(e.as_vec()), e.clone());
            }
        }
        Ok(Allowed { elems, targets })
    }

    /// Get the set of elements that make up the algebra.
    pub fn indices(&self) -> &HashSet<Component> {
        &self.elems
    }

    /// Get a map that allows for determining the correct ordering of a given
    /// set of indices under the current Allowed instance.
    pub fn targets(&self) -> &HashMap<KeyVec, Component> {
        &self.targets
    }

    pub fn alphas(&self) -> Vec<Alpha> {
        self.elems
            .iter()
            .map(|c| Alpha::from_comp(c, &Sign::Pos))
            .collect()
    }
}

/// Construct a new metric from a sign string.
///
/// Sign strings are four characters long and are made of +/- only.
///
/// # Examples
/// ```
/// use arthroprod::config::metric_from_string;
///
/// match metric_from_string("-+++") {
///     Ok(m) => m,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn metric_from_string(s: &str) -> Result<HashMap<Index, Sign>> {
    if s.len() != 4 {
        return Err(ArError::InvalidConfig(String::from("Metric must contain 4 values")));
    }
    let mut m = HashMap::new();

    for (ix, sgn) in s.chars().enumerate() {
        let index = Index::try_from_str(ix.to_string().as_str())?;
        match sgn {
            '+' => m.insert(index, Sign::Pos),
            '-' => m.insert(index, Sign::Neg),
            _ => return Err(ArError::InvalidConfig(String::from("Metric string must only be +/-"))),
        };
    }
    Ok(m)
}
