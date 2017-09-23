use super::types::{Component, Index, KeyVec, Sign};
use std::collections::{HashMap, HashSet};


pub fn get_allowed() -> HashSet<Component> {
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
    return allowed;
}

pub fn get_targets(allowed: &HashSet<Component>) -> HashMap<KeyVec, Component> {
    let mut targets = HashMap::new();
    for s in allowed.iter() {
        if *s != Component::Point {
            targets.insert(KeyVec::new(s.to_vec()), s.clone());
        }
    }
    return targets;
}

pub fn get_metric() -> HashMap<Index, Sign> {
    let mut metric = HashMap::new();
    metric.insert(Index::Zero, Sign::Pos);
    metric.insert(Index::One, Sign::Neg);
    metric.insert(Index::Two, Sign::Neg);
    metric.insert(Index::Three, Sign::Neg);
    return metric;
}
