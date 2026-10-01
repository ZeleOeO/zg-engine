use crate::World;

pub trait SystemParam {
    type Item<'w>;
    fn extract_world_context<'w>(context: &'w World) -> Self::Item<'w>;
}

macro_rules! impl_for_system_param {
    ($($param:ident),*) => {
        impl<T, $($param),*> $crate::systems::system::SystemFunction<($($param),*)> for T
        where
            T: for<'w> Fn($($param::Item<'w>),*),
            $($param: $crate::systems::system_param::SystemParam),*
        {
            fn call<'w>(&self, context: &'w $crate::World) {
                (self)($($param::extract_world_context(context)),*)
            }
        }
    };
}

// Resource for Res and ResMut

impl_for_system_param!(A);
impl_for_system_param!(A, B);
impl_for_system_param!(A, B, C);
impl_for_system_param!(A, B, C, D);
impl_for_system_param!(A, B, C, D, E);
