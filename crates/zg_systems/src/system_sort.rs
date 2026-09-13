use crate::systems::system_struct::SystemID;

#[derive(Debug)]
pub enum SystemSort {
    Before(SystemID),
    After(SystemID),
}
