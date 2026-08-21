use std::{
    any::{Any, TypeId},
    ops::Deref,
};

use crate::{
    schedule::schedule::Schedule,
    world::archetypes::{Archetype, ArchetypeID, Entity},
};

pub struct World {
    pub archetypes: Vec<Archetype>,
    pub object_locations: Vec<ObjectLocation>,
    pub entities: Vec<Entity>,
    pub schedule: Schedule,
}

pub struct ObjectLocation {
    pub archetype_id: ArchetypeID,
    pub row: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Vec::new(),
            object_locations: Vec::new(),
            schedule: Schedule::new(),
            entities: Vec::new(),
        }
    }

    pub fn spawn<T: 'static>(mut self, items: Vec<T>) -> Self {
        let archetype_id = self.get_or_create_archetype_id_by_items(&items);
        let archetype = &mut self.archetypes[archetype_id.0 as usize];

        // This is one entity
        for item in items {
            let col = archetype.get_column_mut(&item);
            col.push(item);
        }

        // We store the entity data in archetype
        let entity = Entity(self.entities.len() as u32);
        archetype.entities.push(entity.clone());
        archetype.len += 1;

        // We get the location
        // Store what archetype the entity is and in what location in the entity list
        let row = (archetype.entities.len()) as u32;
        self.object_locations.push(ObjectLocation {
            archetype_id: archetype.archetype_id,
            row,
        });
        self.entities.push(entity);

        self
    }

    pub fn get_or_create_archetype_by_items<T: 'static>(&mut self, items: Vec<T>) -> &Archetype {
        let archetype_id = self.get_or_create_archetype_id_by_items(&items);
        let archetype = Self::get_archetype_by_id(self, &archetype_id);
        archetype
    }

    pub fn get_or_create_archetype_id_by_items<T: 'static>(
        &mut self,
        items: &Vec<T>,
    ) -> ArchetypeID {
        let type_ids: Vec<TypeId> = items.iter().map(|item| (item).type_id()).collect();
        for archetype in self.archetypes.iter() {
            if archetype.components.iter().eq(type_ids.deref()) {
                return archetype.archetype_id;
            }
        }
        let arch_id = ArchetypeID(self.archetypes.len() as u32);

        let archetype = Archetype::new(items, arch_id);
        self.archetypes.push(archetype);
        arch_id
    }

    pub fn get_or_create_archetype_id_by_type_id<T: 'static>(
        &mut self,
        type_ids: Vec<TypeId>,
    ) -> ArchetypeID {
        for archetype in self.archetypes.iter() {
            if archetype.components.iter().eq(&type_ids) {
                return archetype.archetype_id;
            }
        }
        let arch_id = ArchetypeID(self.archetypes.len() as u32);

        let archetype = Archetype::new(&type_ids, arch_id);
        self.archetypes.push(archetype);
        arch_id
    }

    pub fn get_archetype_by_id(&self, archetype_id: &ArchetypeID) -> &Archetype {
        &self.archetypes[archetype_id.0 as usize]
    }

    // pub fn run(mut self) -> Self {
    //     let schedule = &self.schedule;
    //
    //     self
    // }
}

// Need to do systems
