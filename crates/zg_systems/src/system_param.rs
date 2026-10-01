use crate::system::Context;

pub trait SystemParam {
    fn extract_world_context(context: &Context) -> Self;
}

macro_rules! impl_for_system_param {
    ($($param:ident),*) => {
        impl<T, $($param),*> $crate::system::SystemFunction<($($param),*)> for T
        where
            T: Fn($($param),*),
            $($param: $crate::system_param::SystemParam),*
        {
            fn call(&self, context: &$crate::system::Context) {
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
