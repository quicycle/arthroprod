use crate::algebra::{Alpha, Axis, Component, Sign};

// Compute the full product of i and j under the +--- metric and component ordering
// conventions given in ALLOWED_ALPHA_COMPONENTS.
// This function will panic if invalid components are somehow provided in order to
// prevent malformed calculations from running.
pub fn full_product(i: &Alpha, j: &Alpha) -> Alpha {
    let mut sign = i.sign().combine(&j.sign());
    let i_component = i.component();
    let j_component = j.component();

    // Multiplication by ap is idempotent on the component but does affect sign
    match (i.is_point(), j.is_point()) {
        (true, _) => return Alpha::new(sign, j_component).unwrap(),
        (_, true) => return Alpha::new(sign, i_component).unwrap(),
        (false, false) => (),
    };

    let (pop_sign, axes) = pop_and_cancel_repeated_axes(i_component, j_component);
    sign = sign.combine(&pop_sign);

    // For ap and vectors we don't have an ordering to worry about
    match axes.len() {
        0 => return Alpha::new(sign, Component::Point).unwrap(),
        1 => return Alpha::new(sign, Component::Vector(axes[0])).unwrap(),
        _ => (),
    };

    let (ordering_sign, target) = pop_to_correct_ordering(&axes);
    sign = sign.combine(&ordering_sign);

    let comp = Component::try_from_axes(&target).unwrap();
    return Alpha::new(sign, comp).unwrap();
}

// NOTE: This is where we are hard coding the +--- metric along with assuming
//       that we are using conventional sign rules for combining +/-
fn apply_metric(s: Sign, a: &Axis) -> Sign {
    match a {
        Axis::T => s,
        _ => s.combine(&Sign::Neg),
    }
}

// See test case below that ensures this is correct with the current Allowed config
fn get_target_ordering(axes: &Vec<Axis>) -> Vec<Axis> {
    let mut sorted = axes.clone();
    sorted.sort();

    match sorted[..] {
        // B
        [Axis::Y, Axis::Z] => vec![Axis::Y, Axis::Z],
        [Axis::X, Axis::Z] => vec![Axis::Z, Axis::X],
        [Axis::X, Axis::Y] => vec![Axis::X, Axis::Y],
        // E
        [Axis::T, Axis::X] => vec![Axis::T, Axis::X],
        [Axis::T, Axis::Y] => vec![Axis::T, Axis::Y],
        [Axis::T, Axis::Z] => vec![Axis::T, Axis::Z],
        // T
        [Axis::T, Axis::Y, Axis::Z] => vec![Axis::T, Axis::Y, Axis::Z],
        [Axis::T, Axis::X, Axis::Z] => vec![Axis::T, Axis::Z, Axis::X],
        [Axis::T, Axis::X, Axis::Y] => vec![Axis::T, Axis::X, Axis::Y],
        // h, q
        [Axis::X, Axis::Y, Axis::Z] => vec![Axis::X, Axis::Y, Axis::Z],
        [Axis::T, Axis::X, Axis::Y, Axis::Z] => vec![Axis::T, Axis::X, Axis::Y, Axis::Z],
        // p, t & A have no ordering
        _ => axes.clone(),
    }
}

// This makes use of apply_metric above to determine sign changes when cancelling repeated
// axes and starts from a positive sign. The return value of this function needs to be
// combined with any accumulated sign changes to obtain the true sign.
fn pop_and_cancel_repeated_axes(
    i_component: Component,
    j_component: Component,
) -> (Sign, Vec<Axis>) {
    let i_axes = i_component.as_vec();
    let j_axes = j_component.as_vec();
    let mut sign = Sign::Pos;

    let mut axes = i_axes.clone();
    axes.append(&mut j_axes.clone());

    let mut repeated = Vec::new();
    for a in i_axes.iter() {
        if j_axes.contains(a) {
            repeated.push(a.clone());
        }
    }

    for r in repeated.iter() {
        sign = apply_metric(sign, r);

        let (mut i1, mut i2) = (-1, -1);
        for (pos, a) in axes.iter().enumerate() {
            if a == r {
                if i1 == -1 {
                    i1 = pos as i8;
                } else {
                    i2 = pos as i8;
                    break;
                }
            }
        }
        let n_pops = i2 - i1 - 1;
        if n_pops % 2 == 1 {
            sign = sign.combine(&Sign::Neg);
        }

        // Remove elements in reverse order to avoid invalidating the i2
        axes.remove(i2 as usize);
        axes.remove(i1 as usize);
    }

    return (sign, axes);
}

fn pop_to_correct_ordering(axes: &Vec<Axis>) -> (Sign, Vec<Axis>) {
    let target = get_target_ordering(&axes);
    let mut sign = Sign::Pos;

    if &target == axes {
        return (sign, target);
    }

    let mut remaining = permuted_indices(axes, &target);
    while remaining.len() > 1 {
        if remaining[0] % 2 == 1 {
            sign = sign.combine(&Sign::Neg);
        }
        remaining.remove(0);

        let mut sorted = remaining.clone();
        sorted.sort();

        remaining = permuted_indices(&remaining, &sorted);
    }

    return (sign, target);
}

// s1 is assumed to be a permutation of s2 and this will panic if it is not.
// Furthermore, s1 and s2 are required to have no repeated elements.
// The return value is a Vec of the positions of each element of s1 in s2.
fn permuted_indices<T: Ord>(s1: &[T], s2: &[T]) -> Vec<u8> {
    s1.iter()
        .map(|target| s2.iter().position(|elem| elem == target).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{Axis, ALLOWED_ALPHA_COMPONENTS};

    #[test]
    fn target_ordering_is_always_correct_for_allowed() {
        for c in ALLOWED_ALPHA_COMPONENTS.iter() {
            let axes = c.as_vec();
            assert_eq!(get_target_ordering(&axes), axes);
        }
    }

    #[test]
    fn maching_components_cancel_completely() {
        for c in ALLOWED_ALPHA_COMPONENTS.iter() {
            let (_, axes) = pop_and_cancel_repeated_axes(c.clone(), *c);
            assert_eq!(axes, vec![]);
        }
    }

    #[test]
    fn cancelling_repreats_never_leaves_duplicate_axes() {
        for c1 in ALLOWED_ALPHA_COMPONENTS.iter() {
            for c2 in ALLOWED_ALPHA_COMPONENTS.iter() {
                let (_, mut axes) = pop_and_cancel_repeated_axes(*c1, *c2);
                axes.sort();

                let mut deduped = axes.clone();
                deduped.dedup();
                assert_eq!(axes, deduped);
            }
        }
    }

    #[test]
    fn swapping_axes_negates_when_not_squaring() {
        let axes = vec![Axis::T, Axis::X, Axis::Y, Axis::Z];
        for i in axes.clone().iter() {
            for j in axes.iter() {
                if i == j {
                    continue;
                };

                let a1 = Alpha::new(Sign::Pos, Component::Vector(*i)).unwrap();
                let a2 = Alpha::new(Sign::Pos, Component::Vector(*j)).unwrap();

                let res1 = full_product(&a1, &a2);
                let res2 = full_product(&a2, &a1);

                assert_eq!(res1.component(), res2.component());
                assert_ne!(res1.sign(), res2.sign());
            }
        }
    }
}
