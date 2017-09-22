use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

// A hash set that can be used as a key in a HashMap
#[derive(Eq, PartialEq, Debug, Clone)]
struct KeySet {
    set: HashSet<Index>,
}

impl Hash for KeySet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE :: Need to sort the elements first to ensure
        // the same hash.
        let mut elems: Vec<&Index> = self.set.iter().collect();
        elems.sort();
        for elem in elems.iter() {
            elem.hash(state);
        }
    }
}

impl KeySet {
    fn new(c: &Component) -> KeySet {
        let mut set = HashSet::new();
        match c {
            &Component::Vector(a) => {
                set.insert(a);
                KeySet { set }
            }
            &Component::Bivector(a, b) => {
                set.insert(a);
                set.insert(b);
                KeySet { set }
            }
            &Component::Trivector(a, b, c) => {
                set.insert(a);
                set.insert(b);
                set.insert(c);
                KeySet { set }
            }
            &Component::Quadrivector(a, b, c, d) => {
                set.insert(a);
                set.insert(b);
                set.insert(c);
                set.insert(d);
                KeySet { set }
            }
            &Component::Point => KeySet { set },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
enum Index {
    Zero,
    One,
    Two,
    Three,
}

impl Index {
    /// Try to convert a string to an index
    fn try_from_str(s: &str) -> Result<Index, &str> {
        match s {
            "0" => Ok(Index::Zero),
            "1" => Ok(Index::One),
            "2" => Ok(Index::Two),
            "3" => Ok(Index::Three),
            &_ => Err("Invalid index"),
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Index::Zero => write!(f, "0"),
            Index::One => write!(f, "1"),
            Index::Two => write!(f, "2"),
            Index::Three => write!(f, "3"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Component {
    Point,
    Vector(Index),
    Bivector(Index, Index),
    Trivector(Index, Index, Index),
    Quadrivector(Index, Index, Index, Index),
}

#[derive(Debug, Eq, PartialEq)]
enum Sign {
    Pos,
    Neg,
}

#[derive(Debug, Eq, PartialEq)]
struct Alpha {
    index: Component,
    sign: Sign,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Component::Point => write!(f, "p"),
            Component::Vector(ref i) => write!(f, "{}", i),
            Component::Bivector(ref i, ref j) => write!(f, "{}{}", i, j),
            Component::Trivector(ref i, ref j, ref k) => write!(f, "{}{}{}", i, j, k),
            Component::Quadrivector(ref i, ref j, ref k, ref l) => {
                write!(f, "{}{}{}{}", i, j, k, l)
            }
        }
    }
}

impl Component {
    // TODO :: there is probably a trait for this...
    fn to_vec(&self) -> Vec<Index> {
        match *self {
            Component::Vector(i) => vec![i],
            Component::Bivector(i, j) => vec![i, j],
            Component::Trivector(i, j, k) => vec![i, j, k],
            Component::Quadrivector(i, j, k, l) => vec![i, j, k, l],
            Component::Point => vec![],
        }
    }

    fn from_vec(v: &Vec<Index>) -> Component {
        match v.len() {
            0 => Component::Point,
            1 => Component::Vector(v[0]),
            2 => Component::Bivector(v[0], v[1]),
            3 => Component::Trivector(v[0], v[1], v[2]),
            4 => Component::Quadrivector(v[0], v[1], v[2], v[3]),
            _ => panic!("Vector is too long"),
        }
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Pos => write!(f, "α{}", self.index),
            Sign::Neg => write!(f, "-α{}", self.index),
        }
    }
}

impl Alpha {
    fn new<'a>(ix: &'a str, sign: Sign, allowed: &HashSet<Component>) -> Result<Alpha, &'a str> {
        if ix == "p" {
            return Ok(Alpha {
                index: Component::Point,
                sign,
            });
        }

        let v: Vec<&str> = ix.split("")
                             .filter(|&c| c != "")
                             .collect();

        let index = match v.len() {
            1 => {
                let i = Index::try_from_str(v[0])?;
                Component::Vector(i)
            }
            2 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                Component::Bivector(i1, i2)
            }
            3 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                Component::Trivector(i1, i2, i3)
            }
            4 => {
                let i1 = Index::try_from_str(v[0])?;
                let i2 = Index::try_from_str(v[1])?;
                let i3 = Index::try_from_str(v[2])?;
                let i4 = Index::try_from_str(v[3])?;
                Component::Quadrivector(i1, i2, i3, i4)
            }
            _ => return Err("A component has at most 4 indices."),
        };

        if !allowed.contains(&index) {
            return Err("Invalid index provided");
        }
        Ok(Alpha { index, sign })
    }

    /// Check to see if an alpha is Point
    fn is_point(&self) -> bool {
        match self.index {
            Component::Point => true,
            _ => false,
        }
    }
}


/// Comine sign information from two alphas
fn combine_signs(i: &Sign, j: &Sign) -> Sign {
    if i == j { Sign::Pos } else { Sign::Neg }
}

/// find_prod computes the product of two alpha values under the algebra.
fn find_prod(
    i: &Alpha,
    j: &Alpha,
    metric: &HashMap<Index, Sign>,
    targets: &HashMap<KeySet, Component>,
    allowed: &HashSet<Component>,
) -> Alpha {
    let mut sign = combine_signs(&i.sign, &j.sign);

    // Rule (1) :: Multiplication by αp is idempotent
    if i.is_point() {
        return Alpha {
            index: j.index.clone(),
            sign,
        };
    };
    if j.is_point() {
        return Alpha {
            index: i.index.clone(),
            sign,
        };
    };

    // Rule (2) :: Squaring and popping
    let i_comps = i.index.to_vec();
    let j_comps = j.index.to_vec();
    let mut intersection = vec![];

    // Find the repeated components in the combined indices
    for comp in i_comps.iter() {
        if j_comps.contains(comp) {
            intersection.push(comp);
        }
    }

    // Combine into a single vector
    let mut components = i_comps.clone();
    components.append(&mut j.index.to_vec());

    // Find out how far apart the repeated indices are, remove them and then adjust
    // the sign accordingly.
    for repeat in intersection.iter() {
        let mut first = 0;
        let mut second = 0;
        let mut first_index = false;
        for (i, comp) in components.iter().enumerate() {
            if comp == *repeat {
                if first_index {
                    first = i;
                    first_index = false;
                } else {
                    second = i;
                }
            }
        }
        let n_pops = second - first - 1;
        let pop_sign = if n_pops % 2 == 1 {
            Sign::Neg
        } else {
            Sign::Pos
        };
        // Update sign due to pops
        sign = combine_signs(&sign, &pop_sign);
        // Update sign due to cancellation under the metric
        sign = combine_signs(&sign, &metric[repeat]);
        // Remove the repeated elements
        // NOTE:: Remove moves all elements after the index to the left so
        // we need to do it in this order.
        components.remove(second);
        components.remove(first);
    }

    // If everything cancelled then i == j and we are left with αp (r-point)
    if components.len() == 0 {
        return Alpha {
            index: Component::Point,
            sign,
        };
    } else if components.len() == 1 {
        return Alpha {
            index: Component::Vector(components[0]),
            sign,
        };
    }

    // Rule (3) :: Popping to the correct order
    let mut target = HashSet::new();
    for c in components.iter() {
        target.insert(c.clone());
    }
    let target = match targets.get(&KeySet { set: target }) {
        Some(t) => t,
        None => panic!("Shouldn't ever get here!"),
    };
    let target_vec = target.to_vec();

    // If we are already in the correct order then return
    if target_vec == components {
        return Alpha {
            index: target.clone(),
            sign,
        };
    }

    Alpha {
        index: target.clone(),
        sign,
    }
}

fn main() {
    // Initialise the allowed values
    let mut allowed = HashSet::new();
    allowed.insert(Component::Point);
    allowed.insert(Component::Vector(Index::Zero));
    allowed.insert(Component::Vector(Index::One));
    allowed.insert(Component::Vector(Index::Two));
    allowed.insert(Component::Vector(Index::Three));
    allowed.insert(Component::Bivector(Index::One, Index::Zero));
    allowed.insert(Component::Bivector(Index::Two, Index::Zero));
    allowed.insert(Component::Bivector(Index::Three, Index::Zero));
    allowed.insert(Component::Bivector(Index::Two, Index::Three));
    allowed.insert(Component::Bivector(Index::Three, Index::One));
    allowed.insert(Component::Bivector(Index::One, Index::Two));
    allowed.insert(Component::Trivector(Index::Zero, Index::Two, Index::Three));
    allowed.insert(Component::Trivector(Index::Zero, Index::Three, Index::One));
    allowed.insert(Component::Trivector(Index::Zero, Index::One, Index::Two));
    allowed.insert(Component::Trivector(Index::One, Index::Two, Index::Three));
    allowed.insert(Component::Quadrivector(
        Index::Zero,
        Index::One,
        Index::Two,
        Index::Three,
    ));

    // Make targets a set
    let mut targets = HashMap::new();
    for s in allowed.iter() {
        if *s != Component::Point {
            targets.insert(KeySet::new(s), s.clone());
        }
    }

    // Initialise the chosen metric
    let mut metric = HashMap::new();
    metric.insert(Index::Zero, Sign::Pos);
    metric.insert(Index::One, Sign::Neg);
    metric.insert(Index::Two, Sign::Neg);
    metric.insert(Index::Three, Sign::Neg);


    // Make some Alphas
    let p = Alpha::new("p", Sign::Pos, &allowed).unwrap();
    println!("{}", p);
    let v = Alpha::new("2", Sign::Pos, &allowed).unwrap();
    println!("{}", v);
    let b = Alpha::new("10", Sign::Neg, &allowed).unwrap();
    println!("{}", b);
    let t = Alpha::new("031", Sign::Pos, &allowed).unwrap();
    println!("{}", t);
    let q = Alpha::new("0123", Sign::Neg, &allowed).unwrap();
    println!("{}", q);

    // Find a product
    let ans = find_prod(&v, &b, &metric, &targets, &allowed);
    println!("{} ^ {} = {}", v, b, ans);
}
