use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_g, ty_g, where_c) = ast.generics.split_for_impl();

    quote! {
        impl #impl_g ::zg_world::Resource for #name #ty_g #where_c {
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(SystemSet)]
pub fn derive_addon(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_g, ty_g, where_c) = ast.generics.split_for_impl();

    quote! {
        impl #impl_g ::zg_world::SystemSet for #name #ty_g #where_c {}
    }
    .into()
}
