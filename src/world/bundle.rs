use std::any::TypeId;

use crate::world::archetypes::{Archetype, Column};

pub trait Bundle {
    fn insert_into(self, archetype: &mut Archetype);
    fn get_archetype() -> Vec<TypeId>;
    fn empty_columns() -> Vec<Column>;
}

macro_rules! impl_tuple_for_bundle {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl< $($T),*> $crate::world::bundle::Bundle for ($($T,)*)
        where
        $($T: 'static + std::fmt::Debug ),*
        {
            fn insert_into(self,  archetype: &mut $crate::world::archetypes::Archetype) {
                let ($($T,)*) = self;
                ($(archetype.insert_component($T )), *);

            }
            fn get_archetype() -> Vec<TypeId> {
                vec![$(TypeId::of::<$T>()), *]
            }

            fn empty_columns() -> Vec<Column> {
                vec![$(
                    Column {
                        column_type_id: TypeId::of::<$T>(),
                        data: Box::new(Vec::<$T>::new()),
                    }
                ), *
                ]
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
