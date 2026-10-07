use std::{collections::HashSet, fmt::Debug, hash::Hash};

use zg_world::Resource;

#[derive(Resource, Debug)]
pub struct Input<I: Hash + Debug + Clone + Eq + 'static> {
    held: HashSet<I>,
    just_pressed: HashSet<I>,
    just_released: HashSet<I>,
}

impl<I: Hash + Debug + Clone + Eq + 'static> Input<I> {
    pub fn press(&mut self, input: I) {
        if self.held.insert(input.clone()) {
            self.just_pressed.insert(input);
        }
    }

    pub fn release(&mut self, input: I) {
        if self.held.remove(&input) {
            self.just_released.insert(input);
        }
    }

    pub fn is_just_pressed(&self, input: I) -> bool {
        self.just_pressed.contains(&input)
    }
    pub fn is_just_released(&self, input: I) -> bool {
        self.just_released.contains(&input)
    }
    pub fn is_pressed(&self, input: I) -> bool {
        self.held.contains(&input)
    }

    pub fn new() -> Input<I> {
        Input {
            held: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}
