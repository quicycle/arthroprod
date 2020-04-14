//! Finding the product of αs
//! =========================
//! This is based on a set of simplification rules based on allowed
//! manipulations of elements in the algebra.
//! (NOTE: In all notation, αμ.αν is simplified to αμν)
//!
//! (1)   αpμ == αμp == αμ
//!     'Multiplication by αp (point) is idempotent. (αp is the identity)'
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

use crate::algebra::{Alpha, Form, Index, Sign};

/// Compute the full product of i and j under the +--- metric and form ordering
/// conventions given in ALLOWED_ALPHA_formS.
/// This function will panic if invalid forms are somehow provided in order to
/// prevent malformed calculations from running.
pub fn ar_product(i: &Alpha, j: &Alpha) -> Alpha {
    let mut sign = i.sign().combine(&j.sign());
    let i_form = i.form();
    let j_form = j.form();

    // Multiplication by ap is idempotent on the form but does affect sign
    match (i.form(), j.form()) {
        (Form::Point, _) => return Alpha::new(sign, j_form).unwrap(),
        (_, Form::Point) => return Alpha::new(sign, i_form).unwrap(),
        _ => (),
    };

    let (pop_sign, axes) = pop_and_cancel_repeated_indices(i_form, j_form);
    sign = sign.combine(&pop_sign);

    // For ap and vectors we don't have an ordering to worry about
    match axes.len() {
        0 => return Alpha::new(sign, Form::Point).unwrap(),
        1 => return Alpha::new(sign, Form::Vector(axes[0])).unwrap(),
        _ => (),
    };

    let (ordering_sign, target) = pop_to_correct_ordering(&axes);
    sign = sign.combine(&ordering_sign);

    let comp = Form::try_from_indices(&target).unwrap();
    return Alpha::new(sign, comp).unwrap();
}

// NOTE: This is where we are hard coding the +--- metric along with assuming
//       that we are using conventional sign rules for combining +/-
fn apply_metric(s: Sign, a: &Index) -> Sign {
    match a {
        Index::Zero => s,
        _ => s.combine(&Sign::Neg),
    }
}

// See test case below that ensures this is correct with the current Allowed config
fn get_target_ordering(axes: &Vec<Index>) -> Vec<Index> {
    let mut sorted = axes.clone();
    sorted.sort();

    match sorted[..] {
        // B
        [Index::Two, Index::Three] => vec![Index::Two, Index::Three],
        [Index::One, Index::Three] => vec![Index::Three, Index::One],
        [Index::One, Index::Two] => vec![Index::One, Index::Two],
        // E
        [Index::Zero, Index::One] => vec![Index::Zero, Index::One],
        [Index::Zero, Index::Two] => vec![Index::Zero, Index::Two],
        [Index::Zero, Index::Three] => vec![Index::Zero, Index::Three],
        // T
        [Index::Zero, Index::Two, Index::Three] => vec![Index::Zero, Index::Two, Index::Three],
        [Index::Zero, Index::One, Index::Three] => vec![Index::Zero, Index::Three, Index::One],
        [Index::Zero, Index::One, Index::Two] => vec![Index::Zero, Index::One, Index::Two],
        // h, q
        [Index::One, Index::Two, Index::Three] => vec![Index::One, Index::Two, Index::Three],
        [Index::Zero, Index::One, Index::Two, Index::Three] => {
            vec![Index::Zero, Index::One, Index::Two, Index::Three]
        }
        // p, t & A have no ordering
        _ => axes.clone(),
    }
}

// This makes use of apply_metric above to determine sign changes when cancelling repeated
// axes and starts from a positive sign. The return value of this function needs to be
// combined with any accumulated sign changes to obtain the true sign.
fn pop_and_cancel_repeated_indices(i_form: Form, j_form: Form) -> (Sign, Vec<Index>) {
    let i_axes = i_form.as_vec();
    let j_axes = j_form.as_vec();
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

fn pop_to_correct_ordering(axes: &Vec<Index>) -> (Sign, Vec<Index>) {
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
    use crate::algebra::{Alpha, Form, Index, ALLOWED_ALPHA_FORMS, AR};

    #[test]
    fn target_ordering_is_always_correct_for_allowed() {
        for c in ALLOWED_ALPHA_FORMS.iter() {
            let axes = c.as_vec();
            assert_eq!(get_target_ordering(&axes), axes);
        }
    }

    #[test]
    fn maching_forms_cancel_completely() {
        for c in ALLOWED_ALPHA_FORMS.iter() {
            let (_, axes) = pop_and_cancel_repeated_indices(c.clone(), *c);
            assert_eq!(axes, vec![]);
        }
    }

    #[test]
    fn cancelling_repreats_never_leaves_duplicate_axes() {
        for c1 in ALLOWED_ALPHA_FORMS.iter() {
            for c2 in ALLOWED_ALPHA_FORMS.iter() {
                let (_, mut axes) = pop_and_cancel_repeated_indices(*c1, *c2);
                axes.sort();

                let mut deduped = axes.clone();
                deduped.dedup();
                assert_eq!(axes, deduped);
            }
        }
    }

    #[test]
    fn swapping_axes_negates_when_not_squaring() {
        let axes = vec![Index::Zero, Index::One, Index::Two, Index::Three];
        for i in axes.clone().iter() {
            for j in axes.iter() {
                if i == j {
                    continue;
                };

                let a1 = Alpha::new(Sign::Pos, Form::Vector(*i)).unwrap();
                let a2 = Alpha::new(Sign::Pos, Form::Vector(*j)).unwrap();

                let res1 = ar_product(&a1, &a2);
                let res2 = ar_product(&a2, &a1);

                assert_eq!(res1.form(), res2.form());
                assert_ne!(res1.sign(), res2.sign());
            }
        }
    }

    #[test]
    fn alphas_invert_through_ap() {
        let ap = Alpha::new(Sign::Pos, Form::Point).unwrap();

        for c in ALLOWED_ALPHA_FORMS.iter() {
            let alpha = Alpha::new(Sign::Pos, *c).unwrap();

            assert_eq!(ar_product(&alpha, &alpha.inverse()), ap);
        }
    }
}
