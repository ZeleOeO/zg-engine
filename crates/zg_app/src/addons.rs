use crate::App;

pub trait Addon {
    fn build(&self, app: &mut App);
}
