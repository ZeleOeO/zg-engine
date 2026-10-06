use crate::{SystemParam, archetypes::Archetype, bundle::Bundle, world::World};

pub trait QueryData<'w> {
    type Output;
    type Bundle: Bundle;
    fn get(world: &'w World, row: usize) -> Self::Output;
}
pub struct Query<'w, D: QueryData<'w>> {
    pub world: &'w World,
    pub _marker: std::marker::PhantomData<D>,
}
impl<'w, D> Query<'w, D>
where
    D: QueryData<'w> + 'static,
{
    pub fn get(&self, row: u32) -> D::Output {
        D::get(self.world, row as usize)
    }

    pub fn iter(&self, archetype: &Archetype) -> impl Iterator<Item = D::Output> + '_ {
        let entities = archetype.entities.clone();
        let item = entities
            .into_iter()
            .map(move |entity| D::get(self.world, entity.0 as usize));
        item
    }

    pub fn iter_all<'a>(
        &'a self,
        archetypes: &'a [&Archetype],
    ) -> impl Iterator<Item = D::Output> + 'a {
        archetypes.iter().flat_map(move |arch| self.iter(arch))
    }

    pub fn get_all_entities(&self) -> Vec<D::Output> {
        let archetype_id = self.world.get_archetype_ids::<D::Bundle>();
        let archetype = self.world.get_archetypes_by_id(archetype_id.as_slice());
        self.world.get_all_entities_in_archetypes::<D>(&archetype)
    }
}

impl<D> SystemParam for Query<'_, D>
where
    for<'w> D: QueryData<'w> + 'static,
{
    type Param<'w> = Query<'w, D>;
    type State = ();

    fn extract_params<'w>(world: &'w World, _state: &'w mut Self::State) -> Self::Param<'w> {
        world.query()
    }

    fn init_state() -> Self::State {
        ()
    }

    fn reset(_world: &mut crate::World, _state: &mut Self::State) {}
}

macro_rules! impl_query_for_tuples {
    ($($T:ident),*) => {
        impl<'w, $($T),*> $crate::query::QueryData<'w> for ($($T,)*)
        where
        $($T: 'static + std::fmt::Debug ),*
        {
            type Output = ($(&'w mut $T,)*);
            type Bundle = ($($T, ) *);
            fn get(world: &'w  $crate::world::World, row: usize) -> Self::Output {
                let location = &world.object_locations[row];
                let archetype = world.get_archetype_by_id(location.archetype_id);

                unsafe {
                   ($( (&mut *archetype.get_column_ptr_by_type::<$T>()).get_mut(location.row as usize).unwrap(),)*)
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
impl_query_for_tuples!(A, B, C, D, E, F);
