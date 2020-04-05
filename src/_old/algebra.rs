//! Standard operations on AR types: Alphas and Multivectors.
//!
//! Almost all of the operations contained in `ops` are based on the find_prod
//! function which computes the Full Product of two Alphas under the algebra.
//! Override versions of some functions exist which allow the caller to specify
//! custom values for ALLOWED, TARGETS, and METRIC.
//!
//! NOTE: Using the overrides will result in a panic! in cases where there would
//! normally be an error. This is to prevent malformed calculations from being
//! coerced into a valid Alpha value.
//!
//! In almost all cases you want to use the non-override functions which take
//! their configuration from the constants defined in the `consts` module.
//!
//! Finding the product of αs
//! =========================
//! This is based on a set of simplification rules based on allowed
//! manipulations of elements in the algebra.
//! (NOTE: In all notation, αμ.αν is simplified to αμν)
//!
//! (1)   αpμ == αμp == αμ
//!     'Multiplication by αp (r-point) is idempotent. (αp is the identity)'
//!
//! (2i)  α0^2 == αp
//!     'Repeated α0 indices can just be removed.'
//!
//! (2ii) αi^2 == -αp
//!     'Repeated αi indices can be removed by negating'
//!
//! (2iii) α^2 == +-αp
//!     'All elements square to either +αp or -αp'
//!
//! (3)   αμν == -ανμ
//!     'Adjacent indices can be popped by negating.'
//!
//! Counting pops
//! =============
//! I am converting the current product into an array of integers in order to
//! allow for the different orderings of each final product in a flexible way.
//! Ordering is a mapping of index (0,1,2,3) to position in the final product.
//! This should be stable regardless of how we define the 16 elements of the
//! algebra.
//!
//! The algorithm makes use of the fact that for any ordering we can dermine the
//! whether the total number of pops is odd or even by looking at the first
//! element alone and then recursing on the rest of the ordering as a
//! sub-problem. If the final position of the first element is even then it
//! will take an odd number of pops to correctly position it. We can then look
//! only at the remaining elements and re-label them with indices 1->(n-1) and
//! repeat the process until we are done.
//!

use super::config::Allowed;
use super::types::{Alpha, Component, Index, KeyVec, Mvec, Sign};
use std::collections::HashMap;

/// Full product
pub trait ArFull<RHS = Self> {
    fn ar_prod(&self, _rhs: &RHS) -> Mvec;
}

/// Division: LHS into RHS
pub trait ArDivInto<RHS = Self> {
    fn ar_div_into(&self, _rhs: &RHS) -> Mvec;
}

/// Division: LHS by RHS
pub trait ArDivBy<RHS = Self> {
    fn ar_div_by(&self, _rhs: &RHS) -> Mvec;
}

/// Addition
pub trait ArAdd<RHS = Self> {
    fn ar_add(&self, _rhs: &RHS) -> Mvec;
}

/// Compute the Clifford Algebra Full Product between two components.
///
/// (See https://en.wikipedia.org/wiki/Clifford_algebra)
///
/// NOTE: Inside of arthroprod, the full product always returns an Mvec as,
/// strictly speaking, this operation is only valid on Multivectors. In order
/// to simplify computation, and to avoid forcing users to convert everything
/// to a Multivector before calculating, `full_product` accepts anything that
/// implements the ArOps interface:
///
///   * Mvec
///
///   * Pair
///
///   * Alpha
///
/// # Examples
/// ```
/// use arthroprod::types::{Alpha, Mvec};
/// use arthroprod::algebra::full_product;
///
/// let a1 = Alpha::new("31").unwrap();
/// let a2 = Alpha::new("01").unwrap();
///
/// let mut expected = Mvec::new();
/// expected.add_string("-03").unwrap();
///
/// assert_eq!(full_product(&a1, &a2), expected);
/// ```
pub fn full_product<T, S>(i: &T, j: &S) -> Mvec
where
    T: ArFull<S>, // T must impl ArOps with RHS == S
{
    Mvec::from(i.ar_prod(j))
}

/// Allow the caller to specify a different metric and set of target alphas
/// when computing a product.
///
/// This implementation will panic! if the resulting Alpha is not in the
/// supplied `targets` HashMap.
///
/// # Examples
/// ```
/// use arthroprod::types::Alpha;
/// use arthroprod::algebra::find_prod_override;
/// use arthroprod::consts::{ALLOWED, METRIC};
///
/// let a1 = Alpha::new("31").unwrap();
/// let a2 = Alpha::new("01").unwrap();
///
/// assert_eq!(find_prod_override(&a1, &a2, &METRIC, &ALLOWED),
/// Alpha::new("-03").unwrap());
/// assert_eq!(find_prod_override(&a1, &a1, &METRIC, &ALLOWED),
/// Alpha::new("-p").unwrap());
/// ```
pub fn find_prod_override(
    i: &Alpha,
    j: &Alpha,
    metric: &HashMap<Index, Sign>,
    allowed: &Allowed,
) -> Alpha {
    let targets = allowed.targets();
    let mut sign = i.sign().combine_with(&j.sign());

    // Rule (1) :: Multiplication by αp is idempotent
    if i.is_point() {
        let comp = j.comp();
        return Alpha::from_comp(&comp, &sign);
    };
    if j.is_point() {
        let comp = i.comp();
        return Alpha::from_comp(&comp, &sign);
    };

    // Rule (2) :: Squaring and popping
    let i_comps = i.as_vec();
    let j_comps = j.as_vec();
    let mut intersection = vec![];

    // Find the repeated components in the combined indices
    for comp in i_comps.iter() {
        if j_comps.contains(comp) {
            intersection.push(comp);
        }
    }

    // Combine into a single vector
    let mut components = i_comps.clone();
    components.append(&mut j.as_vec());

    // Find out how far apart the repeated indices are, remove them and then adjust
    // the sign accordingly.
    for repeat in intersection.iter() {
        let mut first = 0;
        let mut second = 0;
        let mut first_index = true;
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
        sign = sign.combine_with(&pop_sign);
        // Update sign due to cancellation under the metric
        sign = sign.combine_with(&metric[repeat]);
        // Remove the repeated elements
        components.remove(second);
        components.remove(first);
    }

    // If everything cancelled then i == j and we are left with αp.
    // If we are left with a single index then there is nothing to pop.
    if components.len() == 0 {
        let comp = Component::Point;
        return Alpha::from_comp(&comp, &sign);
    } else if components.len() == 1 {
        let comp = Component::Vector(components[0]);
        return Alpha::from_comp(&comp, &sign);
    }

    // Rule (3) :: Popping to the correct order
    let comp = targets
        .get(&KeyVec::new(components.clone()))
        .expect(&format!("{:?} not in TARGETS.", components))
        .clone();
    let target_vec = comp.as_vec();

    // If we are already in the correct order then we're done.
    if target_vec == components {
        return Alpha::from_comp(&comp, &sign);
    }

    // Get the current ordering and then compute pops to correct
    let mut target_ordering = HashMap::new();
    for (i, c) in target_vec.iter().enumerate() {
        target_ordering.insert(c, i as u8 + 1);
    }
    let mut current: Vec<u8> = components
        .iter()
        .map(|e| *target_ordering.get(e).expect("fail"))
        .collect();

    while current.len() > 1 {
        if current[0] % 2 == 0 {
            sign = sign.combine_with(&Sign::Neg);
        }
        current.remove(0);
        let mut new_ordering = HashMap::new();
        let mut sorted = current.clone();
        sorted.sort();
        for (i, c) in sorted.iter().enumerate() {
            new_ordering.insert(c.clone(), i as u8 + 1);
        }
        current = current
            .iter()
            .map(|e| *new_ordering.get(e).expect("fail"))
            .collect();
    }

    // Now that the sign is correct we can return
    return Alpha::from_comp(&comp, &sign);
}

#[cfg(test)]
mod tests {
    use super::super::consts::ALPHAS;
    use super::super::consts::{ALLOWED, METRIC};
    use super::*;

    lazy_static! {
        static ref POINT: Alpha = Alpha::new("p").unwrap();
        static ref NEG_POINT: Alpha = Alpha::new("-p").unwrap();
    }

    const INDICES: [&str; 4] = ["0", "1", "2", "3"];
    const STR_SIGNS: [&str; 2] = ["", "-"];

    /// Helper to avoid having to pass METRIC and ALLOWED each time
    fn find_prod(i: &Alpha, j: &Alpha) -> Alpha {
        find_prod_override(i, j, &METRIC, &ALLOWED)
    }

    proptest! {
        #[test]
        /// The product of an alpha with itself is always +/-αp
        fn squaring_is_always_ap(ref ix in 0..16, ref s in 0..2) {
            let ix = ALPHAS[*ix as usize];
            let s = STR_SIGNS[*s as usize];
            let i = Alpha::new(&format!("{}{}", s, ix)).unwrap();
            let res = find_prod(&i, &i);

            prop_assert_eq!(res.comp(), &Component::Point);
        }

        #[test]
        /// Multiplication by αp should always be idempotent
        fn ap_is_idempotent(ref ix in 0..16, ref s in 0..2) {
            let ix = ALPHAS[*ix as usize];
            let s = STR_SIGNS[*s as usize];
            let i = Alpha::new(&format!("{}{}", s, ix)).unwrap();
            let res = find_prod(&i, &POINT);

            prop_assert_eq!(res, i);
        }

        #[test]
        /// Multiplication by αp should always be idempotent
        fn neg_ap_negates(ref ix in 0..16, ref s in 0..2) {
            let ix = ALPHAS[*ix as usize];
            let s = STR_SIGNS[*s as usize];
            let i = Alpha::new(&format!("{}{}", s, ix)).unwrap();
            let res = find_prod(&i, &NEG_POINT);

            prop_assert_eq!(res.comp(), i.comp());
            prop_assert_ne!(res.sign(), i.sign());
        }

        #[test]
        /// Swapping two adjacent indices negates the product when not squaring
        fn swap_ij(ref i in 0..4, ref j in 0..4, ref si in 0..2, ref sj in 0..2) {
            // Squaring doesn't flip sign as both elements are equal
            prop_assume!(i != j);

            let i = INDICES[*i as usize];
            let si = STR_SIGNS[*si as usize];
            let i = Alpha::new(&format!("{}{}", si, i)).unwrap();

            let j = INDICES[*j as usize];
            let sj = STR_SIGNS[*sj as usize];
            let j = Alpha::new(&format!("{}{}", sj, j)).unwrap();

            let first = find_prod(&i, &j);
            let second = find_prod(&j, &i);

            prop_assert_eq!(first.comp(), second.comp());
            prop_assert_ne!(first.sign(), second.sign());
        }

        #[test]
        /// Swapping two adjacent indices negates the product when not squaring
        fn swap_ijk(ref i in 0..4, ref j in 0..4, ref k in 0..4,
                    ref si in 0..2, ref sj in 0..2, ref sk in 0..2) {
            // Squaring doesn't flip sign as both elements are equal
            prop_assume!(i != j && i != k && j != k);

            let i = INDICES[*i as usize];
            let si = STR_SIGNS[*si as usize];
            let i = Alpha::new(&format!("{}{}", si, i)).unwrap();

            let j = INDICES[*j as usize];
            let sj = STR_SIGNS[*sj as usize];
            let j = Alpha::new(&format!("{}{}", sj, j)).unwrap();

            let k = INDICES[*k as usize];
            let sk = STR_SIGNS[*sk as usize];
            let k = Alpha::new(&format!("{}{}", sk, k)).unwrap();

            // Should be equal to one another
            let ijk = find_prod(&find_prod(&i, &j), &k);
            let jki = find_prod(&find_prod(&j, &k), &i);
            let kij = find_prod(&find_prod(&k, &i), &j);
            prop_assert_eq!(ijk.clone(), jki.clone());
            prop_assert_eq!(ijk.clone(), kij.clone());

            // should be equal to one another
            let ikj = find_prod(&find_prod(&i, &k), &j);
            let jik = find_prod(&find_prod(&j, &i), &k);
            let kji = find_prod(&find_prod(&k, &j), &i);
            prop_assert_eq!(ikj.clone(), jik.clone());
            prop_assert_eq!(ikj.clone(), kji.clone());

            // Both sets should have the same index but opposite signs.
            prop_assert_eq!(ijk.comp(), ikj.comp());
            prop_assert_ne!(ijk.sign(), ikj.sign());
        }
    }
}
