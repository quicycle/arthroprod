use crate::algebra::{ar_product, invert_alpha, Alpha, MultiVector, Term, AR};

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Serialize, Deserialize)]
pub struct ArDifferential {
    // stored in their inverted from so that we can operate from either the left or right
    // by simply using ar_product
    wrt: Vec<Alpha>,
}

impl ArDifferential {
    pub fn new(wrt: &[Alpha]) -> ArDifferential {
        ArDifferential {
            wrt: wrt.iter().map(|w| invert_alpha(w)).collect(),
        }
    }

    pub fn apply_left(&self, mvec: MultiVector) -> MultiVector {
        self.apply(mvec, ApplyFrom::Left)
    }

    pub fn apply_right(&self, mvec: MultiVector) -> MultiVector {
        self.apply(mvec, ApplyFrom::Right)
    }

    fn apply(&self, mvec: MultiVector, side: ApplyFrom) -> MultiVector {
        MultiVector::from_terms(
            mvec.as_terms()
                .iter()
                .flat_map(|t| {
                    self.wrt
                        .as_alphas()
                        .iter()
                        .map(|w| term_partial(t, w, side))
                        .collect::<Vec<Term>>()
                })
                .collect(),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum ApplyFrom {
    Left,
    Right,
}

fn term_partial(term: &Term, wrt: &Alpha, side: ApplyFrom) -> Term {
    let a: Alpha = match side {
        ApplyFrom::Left => ar_product(wrt, &term.alpha()),
        ApplyFrom::Right => ar_product(&term.alpha(), wrt),
    };

    let mut t = term.clone();
    t.add_partial(wrt);
    t.set_alpha(a);

    return t;
}
