/// Algorithm taken from arpy (Absolute Relativity in Python)
/// Copyright (C) 2016-2017 Innes D. Anderson-Morrison All rights reserved.
///
/// Multiplying αs
/// ==============
/// This is based on a set of simplification rules based on allowed
/// manipulations of elements in the algebra.
/// (NOTE: In all notation, αμ.αν is simplified to αμν)
///
/// (1)   αpμ == αμp == αμ
///     'Multiplication by αp (r-point) is idempotent. (αp is the identity)'
/// (2i)  α0^2 == αp
///     'Repeated α0 indices can just be removed.'
/// (2ii) αi^2 == -αp
///     'Repeated αi indices can be removed by negating'
/// (2iii) α^2 == +-αp
///     'All elements square to either +αp or -αp'
/// (3)   αμν == -ανμ
///     'Adjacent indices can be popped by negating.'
///
/// Counting pops
/// =============
/// I am converting the current product into an array of integers in order to
/// allow for the different orderings of each final product in a flexible way.
/// Ordering is a mapping of index (0,1,2,3) to position in the final product.
/// This should be stable regardless of how we define the 16 elements of the
/// algebra.
///
/// The algorithm makes use of the fact that for any ordering we can dermine the
/// whether the total number of pops is odd or even by looking at the first
/// element alone and then recursing on the rest of the ordering as a
/// sub-problem. If the final position of the first element is even then it
/// will take an odd number of pops to correctly position it. We can then look
/// only at the remaining elements and re-label them with indices 1->(n-1) and
/// repeat the process until we are done.
use super::types::{Alpha, Component, Index, KeyVec, Sign};
use std::collections::HashMap;

pub fn combine_signs(i: &Sign, j: &Sign) -> Sign {
    if i == j { Sign::Pos } else { Sign::Neg }
}

pub fn find_prod(
    i: &Alpha,
    j: &Alpha,
    metric: &HashMap<Index, Sign>,
    targets: &HashMap<KeyVec, Component>,
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
        components.remove(second);
        components.remove(first);
    }

    // If everything cancelled then i == j and we are left with αp.
    // If we are left with a single index then there is nothing to pop.
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
    let target = match targets.get(&KeyVec::new(components.clone())) {
        Some(t) => t,
        None => panic!("Shouldn't ever get here!"),
    };
    let target_vec = target.to_vec();

    // If we are already in the correct order then we're done.
    if target_vec == components {
        return Alpha {
            index: target.clone(),
            sign,
        };
    }

    // Get the current ordering and then compute pops to correct
    let mut target_ordering = HashMap::new();
    for (i, c) in target_vec.iter().enumerate() {
        target_ordering.insert(c, i as u8 + 1);
    }
    let mut current: Vec<u8> = target_vec.iter()
                                         .map(|e| *target_ordering.get(e).expect("fail"))
                                         .collect();

    while current.len() > 1 {
        if current[0] % 2 == 0 {
            sign = combine_signs(&sign, &Sign::Neg);
        }
        current.remove(0);
        let mut new_ordering = HashMap::new();
        let mut sorted = current.clone();
        sorted.sort();
        for (i, c) in sorted.iter().enumerate() {
            new_ordering.insert(c.clone(), i as u8 + 1);
        }
        current = current.iter()
                         .map(|e| *new_ordering.get(e).expect("fail"))
                         .collect();
    }

    return Alpha {
        index: target.clone(),
        sign,
    };
}
