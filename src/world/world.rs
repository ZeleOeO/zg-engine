use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::Deref,
    sync::Arc,
};

use winit::window::Window;

use crate::{
    camera::{camera::Camera, camera_controller::CameraController},
    graphics::gpu::InternalGraphics,
    managers::Manager,
    render::render_queue::RenderQueue,
    utils::storage_util::TypeIdMap,
    world::{
        archetypes::{Archetype, ArchetypeID, Entity},
        query::{Query, QueryData},
        resources::{Resource, ResourceMut, ResourceRef},
    },
};

pub struct World {
    pub archetypes: Vec<Archetype>,
    pub object_locations: Vec<ObjectLocation>,
    pub entities: Vec<Entity>,
    pub resources: TypeIdMap<RefCell<Box<dyn Resource>>>,
    pub default_camera: Option<Entity>,
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
            entities: Vec::new(),
            resources: TypeIdMap::default(),
            default_camera: None,
        }
    }

    pub fn spawn<T: 'static>(&mut self, items: Vec<T>) -> Entity {
        let archetype_id = self.get_or_create_archetype_id_by_items(&items);
        let archetype = &mut self.archetypes[archetype_id.0 as usize];

        // This is one entity
        for item in items {
            let col = archetype.get_column_mut(&item);
            col.push(item);
        }

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

        entity
    }

    pub fn get<R: Resource + 'static>(&self) -> ResourceRef<R> {
        let item = self.resources.get(&TypeId::of::<R>()).unwrap();
        let borrowed = item.try_borrow().unwrap();
        let resource = Ref::map(borrowed, |resource| {
            resource.as_ref().as_any().downcast_ref::<R>().unwrap()
        });
        ResourceRef(resource)
    }

    pub fn get_mut<R: Resource + 'static>(&self) -> ResourceMut<R> {
        let item = self.resources.get(&TypeId::of::<R>()).unwrap();
        let borrow_mut = item.try_borrow_mut().unwrap();
        let resource = RefMut::map(borrow_mut, |resource| {
            resource.as_mut().as_any_mut().downcast_mut::<R>().unwrap()
        });
        ResourceMut(resource)
    }

    pub fn insert<R: Resource + 'static>(&mut self, resource: R) {
        self.resources
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn insert_default_resources(&mut self, window: Arc<Window>) {
        let window_size = window.inner_size();
        let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();

        let manager = Manager::new();
        let camera_entity = self.spawn(vec![Camera::new(
            (window_size.width as f32) / window_size.height as f32,
        )]);
        self.default_camera = Some(camera_entity);

        let camera_controller = CameraController::new(0.1, 2.0);

        let render_queue = RenderQueue::default();

        self.insert(window);
        self.insert(internal_graphics);
        self.insert(manager);
        self.insert(camera_controller);
        self.insert(render_queue);
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

    pub fn get_or_create_archetype_id_by_type_ids(&mut self, type_ids: Vec<TypeId>) -> ArchetypeID {
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

    pub fn get_archetype_by_type_ids(&self, type_ids: Vec<TypeId>) -> Option<&Archetype> {
        for archetype in self.archetypes.iter() {
            if archetype.components.iter().eq(&type_ids) {
                return Some(self.get_archetype_by_id(&archetype.archetype_id));
            }
        }
        None
    }

    fn query<'w, D: QueryData<'w>>(&'w self) -> Query<'w, D> {
        Query {
            world: self,
            _marker: PhantomData,
        }
    }

    pub fn get_entity<'w, D: QueryData<'w>>(&'w self, entity: Entity) -> D::Output {
        let location = &self.object_locations[entity.0 as usize];
        self.query::<D>().get(location.row)
    }

    pub fn get_all_entities_in_archetype<'w, D: QueryData<'w> + 'w>(
        &'w self,
        archetype: &Archetype,
    ) -> Vec<D::Output> {
        let query = self.query::<D>();
        query.iter(archetype).collect::<Vec<_>>()
    }
}
