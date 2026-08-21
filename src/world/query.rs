use crate::world::{archetypes::ArchetypeID, world::World};

pub trait QueryData<'w> {
    type Output;
    fn get_archetype(world: &'w mut World) -> ArchetypeID;
    fn get(world: &'w mut World, row: usize) -> Self::Output;
}
#[macro_export]
macro_rules! impl_query_data_for_tuple {
    ($($T:ty),*) => {
        impl<'w > $crate::world::query::QueryData<'w> for ($($T,)*)
        where
        $($T: 'static),*
        {
            type Output = ($(&'w $T,)*);
            fn get(world: &'w mut World, row: usize) -> Self::Output {
                let archetype_id = world.get_or_create_archetype_id_by_type_id(
                    vec![$(::std::any::TypeId::of::<$T>()),*]
                );
                let archetype = &world.archetypes[archetype_id.0 as usize];
                ($(archetype.get_column_with_type::<$T>()[row],)*)
            }
        fn get_archetype(world: &'w mut World) -> $crate::world::archetypes::ArchetypeID {
            world.get_or_create_archetype_id_by_type_id(
                vec![$(::std::any::TypeId::of::<$T>()),*]
            )
        }

        }
    };
}

// In case the macro doesn't work
// impl<'w, T: 'static> QueryData<'w> for &T {
//     type Output = &'w Vec<T>;
//     fn get(world: &'w mut World) -> Self::Output {
//         let archetype_id =
//             world.get_or_create_archetype_id_by_type_id::<T>(vec![TypeId::of::<T>()]);
//         let archetype = &world.archetypes[archetype_id.0 as usize];
//         // This gets the column
//         archetype.get_column_with_type::<T>()
//     }
// }
//
// impl<'w, A: 'static, B: 'static> QueryData<'w> for (&A, &B) {
//     type Output = (&'w Vec<A>, &'w Vec<B>);
//     fn get(world: &'w mut World) -> Self::Output {
//         let archetype_id =
//             world.get_or_create_archetype_id_by_type_id::<A>(vec![TypeId::of::<A>()]);
//         let archetype = &world.archetypes[archetype_id.0 as usize];
//         // This gets the column
//         (
//             archetype.get_column_with_type::<A>(),
//             archetype.get_column_with_type::<B>(),
//         )
//     }
// }

pub struct Query<'w, D: QueryData<'w>> {
    world: &'w mut World,
    _marker: std::marker::PhantomData<D>,
}

impl<'w, D: QueryData<'w>> Query<'w, D> {
    fn iter(&self) -> impl Iterator<Item = D::Output> + '_ {
        let item = (0..self
            .world
            .get_archetype_by_id(&D::get_archetype(self.world))
            .len)
            .map(|row| D::get(self.world, row));
        item
    }
}
