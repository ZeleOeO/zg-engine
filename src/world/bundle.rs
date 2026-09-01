use std::any::TypeId;

use crate::world::archetypes::Archetype;

pub trait Bundle {
    fn insert_into(self, archetype: &mut Archetype);
    fn get_archetype() -> Vec<TypeId>;
}

macro_rules! impl_tuple_for_bundle {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl< $($T),*> $crate::world::bundle::Bundle for ($($T,)*)
        where
        $($T: 'static),*
        {
            fn insert_into(self,  archetype: &mut $crate::world::archetypes::Archetype) {
                let ($($T,)*) = self;
                ($(archetype.insert_component($T )), *);

            }
            fn get_archetype() -> Vec<TypeId> {
                vec![$(TypeId::of::<$T>()), *]
            }
        }
    };
}

impl_tuple_for_bundle!(A);
impl_tuple_for_bundle!(A, B);
impl_tuple_for_bundle!(A, B, C);
impl_tuple_for_bundle!(A, B, C, D);

// fn stuff() {
//     Archetype
// }
