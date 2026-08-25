// use std::any::TypeId;
//
// use crate::world::{
//     archetypes::{Archetype, ArchetypeID},
//     world::World,
// };
//
// trait QueryData<'w> {
//     type Fetch;
//     type Output;
//     fn init_fetch(world: &'w World, archetype: &'w Archetype) -> Self::Fetch;
//     fn fetch(fetch: &Self::Fetch, row: usize) -> Self::Output;
//     fn matches(archetype: &Archetype) -> bool;
// }
//
// impl<'w, T: 'static> QueryData<'w> for &T {
//     type Fetch = &'w [T];
//     type Output = &'w T;
//
//     fn init_fetch(_world: &'w World, archetype: &'w Archetype) -> Self::Fetch {
//         archetype.get_column_with_type::<T>()
//     }
//     fn fetch(fetch: &Self::Fetch, row: usize) -> &'w T {
//         &fetch[row]
//     }
//     fn matches(archetype: &Archetype) -> bool {
//         archetype.has_component::<T>() // contains, not "equals"
//     }
// }
//
// pub struct Query<'w, D: QueryData<'w>> {
//     world: &'w mut World,
//     _marker: std::marker::PhantomData<D>,
// }
//
// impl<'w, D: QueryData<'w>> Query<'w, D> {
//     fn iter(&mut self) -> impl Iterator<Item = D::Output> + '_ {
//         let item = (0..&self
//             .world
//             .get_archetype_by_id(&D::get_archetype(self.world))
//             .len)
//             .map(|row| D::get(self.world, row));
//         item
//     }
// }
//
// PUTTING THIS ON HOLD CAUSE IT'S TAKING SO MUCH TIME
//
//
