use crate::world::{archetypes::Archetype, world::World};

// I need to get all components in an archetype
// We'll get the archetype
// We'll then have a vec of columns
// if we iterate through them line by line
// Then we can get the components line by line
//
// I also need to make it mut

pub trait QueryData<'w> {
    type Output;
    fn get(world: &'w World, row: usize) -> Self::Output;
}
macro_rules! impl_query_for_tuples {
    ($($T:ident),*) => {
        impl<'w, $($T),*> $crate::world::query::QueryData<'w> for ($($T,)*)
        where
        $($T: 'static),*
        {
            type Output = ($(&'w mut $T,)*);
            fn get(world: &'w  $crate::world::world::World, row: usize) -> Self::Output {
                let location = &world.object_locations[row];
                let archetype = world.get_archetype_by_id(location.archetype_id);
                println!("Macro: {:#?}", archetype);

                // Takes pointer so I can mutate without worrying about multiple mutation borrows
                unsafe {
                   ($( (&mut *archetype.get_column_ptr_by_type::<$T>()).get_mut(row).unwrap(),)*)
                }
            }

        }
    };
}

impl_query_for_tuples!(A);
impl_query_for_tuples!(A, B);
impl_query_for_tuples!(A, B, C);
impl_query_for_tuples!(A, B, C, D);
impl_query_for_tuples!(A, B, C, D, E);

pub struct Query<'w, D: QueryData<'w>> {
    pub world: &'w World,
    pub _marker: std::marker::PhantomData<D>,
}
//
impl<'w, D: QueryData<'w>> Query<'w, D> {
    // fn iter(&self) -> impl Iterator<Item = D::Output> + '_ {}

    pub fn get(&self, row: u32) -> D::Output {
        D::get(self.world, row as usize)
    }

    pub fn iter(&self, archetype: &Archetype) -> impl Iterator<Item = D::Output> + '_ {
        (0..archetype.len()).map(|row| D::get(self.world, row))
    }
}
