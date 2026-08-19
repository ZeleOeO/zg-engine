use std::any::{Any, TypeId, type_name};

use crate::{
    managers::{material::MaterialComponent, mesh::MeshComponent, transform::Transform},
    resources::mesh::Mesh,
};

#[derive(Clone, Copy)]
pub struct ArchetypeID(u32);

pub struct Archetype {
    pub archetype_id: ArchetypeID,
    pub row: Vec<u32>,
    pub components: Vec<TypeId>,
    pub column: Vec<Column>,
}

pub enum Column {
    Transform(Vec<Transform>),
    Mesh(Vec<MeshComponent>),
    Material(Vec<MaterialComponent>),
    // Light(Vec<LightComponent>),
}


pub trait Components {
    type Output;
    fn get_column(column: &Column) -> Option<&Self::Output>;
}

impl Components for Transform {
    type Output = Vec<Transform>;
    fn get_column(column: &Column) -> Option<&Vec<Transform>> {
        match column {
            Column::Transform(item) => Some(item),
            _ => None,
        }
    }
}

impl Components for Mesh {
    type Output = Vec<MeshComponent>;
    fn get_column(column: &Column) -> Option<&Self::Output> {
        match column {
            Column::Mesh(mesh) => Some(mesh),
            _ => None,
        }
    }
}

impl Components for MaterialComponent {
    type Output = Vec<MaterialComponent>;
    fn get_column(column: &Column) -> Option<&Self::Output> {
        match column {
            Column::Material(material) => Some(material),
            _ => None,
        }
    }
}

impl Archetype {
    pub fn new<T: Components>(&self, components: Vec<T>, archetype_id: ArchetypeID) -> Self {
        let type_ids = components
            .iter()
            .map(|component| (component).type_id())
            .collect();
        Self {
            archetype_id: archetype_id,
            row: Vec::new(),
            components: type_ids,
            column: ,
        }
    }

    pub fn get_column<T: Components>(&self) -> &T::Output {
        // I have a vector
        // I want to get the actual column
        T::get_column(&self.column).unwrap()
    }
}
