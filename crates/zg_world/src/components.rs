use std::{any::Any, fmt::Debug};

use crate::Component;
mod camera;
mod render_component;

pub use camera::*;
pub use render_component::*;

impl<T: Debug + Component + 'static> ComponentColumn for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_at_row(&self, row: usize) -> &dyn Any {
        &self[row]
    }
}

pub trait ComponentColumn: Any + Debug {
    fn len(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_at_row(&self, row: usize) -> &dyn Any;
}
