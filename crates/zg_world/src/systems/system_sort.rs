use crate::systems::system::SystemID;

#[derive(Debug)]
pub enum SystemSort {
    Before(SystemID),
    After(SystemID),
}
