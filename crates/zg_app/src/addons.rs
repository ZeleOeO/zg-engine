use std::sync::Arc;

use winit::window::Window;
use zg_world::Resource;

use crate::App;

pub trait Addon {
    fn build(&self, app: &mut App);
}

#[derive(Debug, Resource)]
pub struct WindowHandle(pub Arc<Window>);
