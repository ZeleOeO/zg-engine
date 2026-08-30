use std::any::{Any, TypeId};

use crate::world::components::ComponentColumn;

#[derive(Clone, Copy)]
pub struct ArchetypeID(pub u32);

#[derive(Clone, Copy)]
pub struct Entity(pub u32);

pub struct Archetype {
    pub archetype_id: ArchetypeID,
    pub entities: Vec<Entity>,
    pub components: Vec<TypeId>,
    pub columns: Vec<Column>,
}

pub struct Column {
    // Can't even remember why I wanted to use this
    column_type_id: TypeId,
    data: Box<dyn ComponentColumn>,
}

impl Column {
    fn get_column<T: 'static>(&self) -> &Vec<T> {
        self.data.as_any().downcast_ref::<Vec<T>>().unwrap()
    }

    fn get_column_mut<T: 'static>(&mut self) -> &mut Vec<T> {
        self.data.as_any_mut().downcast_mut::<Vec<T>>().unwrap()
    }
}

impl Archetype {
    pub fn new<T: 'static>(components: &Vec<T>, archetype_id: ArchetypeID) -> Self {
        let type_ids = components
            .iter()
            .map(|component| (component).type_id())
            .collect();
        Self {
            archetype_id: archetype_id,
            entities: Vec::new(),
            components: type_ids,
            columns: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn get_column<T: 'static>(&self, value: &T) -> &Vec<T> {
        let column = self
            .columns
            .iter()
            .find(|col| col.column_type_id == value.type_id())
            .unwrap();
        column.get_column::<T>()
    }

    pub fn get_column_mut<T: 'static>(&mut self, value: &T) -> &mut Vec<T> {
        let column = self
            .columns
            .iter_mut()
            .find(|col| col.column_type_id == value.type_id())
            .unwrap();
        column.get_column_mut::<T>()
    }

    pub fn get_column_by_type<T: 'static>(&self) -> &Vec<T> {
        let column = self
            .columns
            .iter()
            .find(|col| col.column_type_id == TypeId::of::<T>())
            .unwrap();
        column.get_column::<T>()
    }

    pub fn get_mut_column_by_type<T: 'static>(&mut self) -> &Vec<T> {
        let column = self
            .columns
            .iter_mut()
            .find(|col| col.column_type_id == TypeId::of::<T>())
            .unwrap();
        column.get_column_mut::<T>()
    }

    pub fn get_column_by_type_id<T: 'static>(&self, value: TypeId) -> &Vec<T> {
        let column = self
            .columns
            .iter()
            .find(|col| col.column_type_id == value)
            .unwrap();
        column.get_column::<T>()
    }

    pub fn get_column_ptr_by_type<T: 'static>(&self) -> *mut Vec<T> {
        let column = self.get_column_by_type::<T>();
        column as *const Vec<T> as *mut Vec<T>
    }

    pub fn has_component<T: 'static>(&self) -> bool {
        self.components.contains(&TypeId::of::<T>())
    }
}
