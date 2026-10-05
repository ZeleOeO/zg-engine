use std::{collections::HashSet, hash::Hash};

pub struct Input<I: Hash + Clone + Eq + 'static> {
    hel: HashSet<I>,
    just_pressed: HashSet<I>,
    just_release: HashSet<I>
}
