use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    marker::PhantomData,
};

use std::collections::HashSet;
use zg_utils::TypeIdMap;

use crate::{
    QueryData,
    archetypes::{Archetype, ArchetypeID, Entity},
    bundle::Bundle,
    events::Events,
    resources::{ResMut, Resource},
    systems::Query,
};

pub struct World {
    pub archetypes: Vec<Archetype>,
    pub object_locations: Vec<ObjectLocation>,
    pub entities: Vec<Entity>,
    pub resources: TypeIdMap<RefCell<Box<dyn Resource>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectLocation {
    pub archetype_id: ArchetypeID,
    pub row: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Vec::new(),
            object_locations: Vec::new(),
            entities: Vec::new(),
            resources: TypeIdMap::default(),
        }
    }

    // Replacing the T with a trait Bundle
    pub fn spawn<T: Bundle + Debug + 'static>(&mut self, bundle: T) {
        let archetype_id = self.get_or_create_archetype_id::<T>();
        let archetype = &mut self.archetypes[archetype_id.0 as usize];
        bundle.insert_into(archetype);

        // We store the entity data in archetype
        let entity = Entity(self.entities.len() as u32);
        // Get the row it's in in the archetype
        let row = (archetype.entities.len()) as u32;
        archetype.entities.push(entity.clone());

        // We get the location
        // Store what archetype the entity is and in what location in the entity list
        self.object_locations.push(ObjectLocation {
            archetype_id: archetype.archetype_id,
            row,
        });
        self.entities.push(entity);
    }

    pub fn get_resource<R: Resource + 'static>(&self) -> Ref<R> {
        let item = self
            .resources
            .get(&TypeId::of::<R>())
            .unwrap_or_else(|| panic!("Resource not found: {}", std::any::type_name::<R>()));
        let borrowed = item.try_borrow().unwrap();
        let resource = Ref::map(borrowed, |resource| {
            resource.as_ref().as_any().downcast_ref::<R>().unwrap()
        });
        resource
    }

    pub fn get_resource_mut<R: Resource + 'static>(&self) -> RefMut<R> {
        let item = self
            .resources
            .get(&TypeId::of::<R>())
            .unwrap_or_else(|| panic!("Resource not found (mut): {}", std::any::type_name::<R>()));
        let borrow_mut = item.try_borrow_mut().unwrap();
        let resource = RefMut::map(borrow_mut, |resource| {
            resource.as_mut().as_any_mut().downcast_mut::<R>().unwrap()
        });
        resource
    }

    pub fn insert<R: Resource + 'static>(&mut self, resource: R) {
        self.resources
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn remove<R: Resource + 'static>(&mut self) -> RefCell<Box<dyn Resource>> {
        let resource = self.resources.remove(&TypeId::of::<R>()).unwrap();
        resource
    }

    pub fn resource_scope<R: Resource>(&mut self, f: impl FnOnce(&mut World, ResMut<R>)) {
        let item = self.remove::<R>();
        let borrow_mut = item.try_borrow_mut().unwrap();
        let resource = RefMut::map(borrow_mut, |resource| {
            resource.as_mut().as_any_mut().downcast_mut::<R>().unwrap()
        });
        f(self, ResMut(resource));
        self.resources.insert(TypeId::of::<R>(), item);
    }

    pub fn get_or_create_archetype_id<T: Bundle + 'static>(&mut self) -> ArchetypeID {
        let type_ids: HashSet<TypeId> = T::get_archetype();

        for archetype in self.archetypes.iter() {
            if archetype.components == type_ids {
                return archetype.archetype_id;
            }
        }

        let arch_id = ArchetypeID(self.archetypes.len() as u32);
        let archetype = Archetype::new::<T>(arch_id);
        self.archetypes.push(archetype);
        arch_id
    }

    pub fn get_archetype_id<T: Bundle + 'static>(&self) -> Option<ArchetypeID> {
        let type_ids: HashSet<TypeId> = T::get_archetype();
        for archetype in self.archetypes.iter() {
            if archetype.components == type_ids {
                return Some(archetype.archetype_id);
            }
        }
        None
    }

    pub fn get_archetype_ids<T: Bundle + 'static>(&self) -> Vec<ArchetypeID> {
        let mut archetype_ids: Vec<ArchetypeID> = Vec::new();
        let type_ids: HashSet<TypeId> = T::get_archetype();
        for archetype in self.archetypes.iter() {
            if type_ids.iter().all(|t| archetype.components.contains(t)) {
                archetype_ids.push(archetype.archetype_id);
            }
        }


        archetype_ids
    }

    pub fn get_archetype_by_id(&self, archetype_id: ArchetypeID) -> &Archetype {
        &self.archetypes[archetype_id.0 as usize]
    }

    pub fn get_mut_archetype_by_id(&mut self, archetype_id: ArchetypeID) -> &mut Archetype {
        &mut self.archetypes[archetype_id.0 as usize]
    }

    pub fn get_archetypes_by_id(&self, archetype_ids: &[ArchetypeID]) -> Vec<&Archetype> {
        self.archetypes
            .iter()
            .filter(|arch| archetype_ids.contains(&arch.archetype_id))
            .collect()
    }

    pub fn query<'w, D>(&'w self) -> Query<'w, D>
    where
        D: QueryData<'w> + 'static,
    {
        Query {
            world: self,
            _marker: PhantomData,
        }
    }

    pub fn get_entity<'w, D>(&'w self, entity: Entity) -> D::Output
    where
        D: QueryData<'w> + 'static,
    {
        self.query::<D>().get(entity.0)
    }

    pub fn get_all_entities_in_archetype<'w, D>(&'w self, archetype: &Archetype) -> Vec<D::Output>
    where
        D: QueryData<'w> + 'static,
    {
        let query = self.query::<D>();
        query.iter(archetype).collect::<Vec<D::Output>>()
    }

    pub fn get_all_entities_in_archetypes<'w, D: QueryData<'w> + 'static>(
        &'w self,
        archetypes: &Vec<&Archetype>,
    ) -> Vec<D::Output> {
        let query = self.query::<D>();
        query
            .iter_all(archetypes.as_slice())
            .collect::<Vec<D::Output>>()
    }

    pub fn add_event<E: Debug + Clone + 'static>(&mut self) {
        let events = Events::<E>::new();
        self.insert(events);
    }

    pub fn write_events<E: Debug + Clone + 'static>(&mut self, event: E) {
        let mut events = self.get_resource_mut::<Events<E>>();
        events.write(event);
    }
}
