mod input;
mod keyboard;
mod mouse;
mod system;

pub use input::{Input, InputFocus};
pub use mouse::MouseMotion;
pub use system::{CloseSet, InputAddon, InputSet};
