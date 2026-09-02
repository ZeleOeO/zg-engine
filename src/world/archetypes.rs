use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use crate::world::{bundle::Bundle, components::ComponentColumn};

#[derive(Clone, Copy, Debug)]
pub struct ArchetypeID(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct Entity(pub u32);

#[derive(Debug)]
pub struct Archetype {
    pub archetype_id: ArchetypeID,
    pub entities: Vec<Entity>,
    pub components: Vec<TypeId>,
    pub columns: Vec<Column>,
}

#[derive(Debug)]
pub struct Column {
    // Can't even remember why I wanted to use this
    pub column_type_id: TypeId,
    pub data: Box<dyn ComponentColumn>,
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
    pub fn new<T: Bundle + 'static>(archetype_id: ArchetypeID) -> Self {
        let type_ids = T::get_archetype();
        let items = T::empty_columns();
        Self {
            archetype_id: archetype_id,
            entities: Vec::new(),
            components: type_ids,
            columns: items,
        }
    }

    pub fn new_with_type_ids(type_ids: Vec<TypeId>, archetype_id: ArchetypeID) -> Self {
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

    pub fn insert_component<T: 'static>(&mut self, component: T) {
        let col = self.get_column_mut(&component);
        col.push(component);
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

    pub fn get_column_mut_by_type_id<T: 'static>(&mut self, value: TypeId) -> &mut Vec<T> {
        let column = self
            .columns
            .iter_mut()
            .find(|col| col.column_type_id == value)
            .unwrap();
        column.get_column_mut::<T>()
    }

    pub fn get_column_ptr_by_type<T: 'static>(&self) -> *mut Vec<T> {
        let column = self.get_column_by_type::<T>();
        column as *const Vec<T> as *mut Vec<T>
    }

    pub fn has_component<T: 'static>(&self) -> bool {
        self.components.contains(&TypeId::of::<T>())
    }
}
