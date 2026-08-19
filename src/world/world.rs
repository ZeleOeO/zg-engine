use std::any::TypeId;

use crate::world::archetypes::{Archetype, ArchetypeID};

pub struct World {
    // TODO add archetypes
    archetypes: Vec<Archetype>,
    object_locations: Vec<ObjectLocation>,
}

pub struct ObjectLocation {
    pub archetype_id: ArchetypeID,
    pub row: u32,
}

impl World {
    // TODO Change camera aspect ratio
    pub fn new() -> Self {
        Self {
            archetypes: Vec::new(),
            object_locations: Vec::new(),
        }
    }

    pub fn spawn(mut self) -> Self {
        // TODO add a bunch of stuff up top

        self
    }

    //
    pub fn get_or_create_archetype(&mut self, components: &Vec<TypeId>) -> ArchetypeID {
        for archetype in self.archetypes.iter() {
            if archetype.components.iter().eq(components) {
                return archetype.archetype_id;
            }
        }
    }
}

// Need to do systems
