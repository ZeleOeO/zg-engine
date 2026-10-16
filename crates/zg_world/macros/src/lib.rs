use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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
pub fn derive_system_set(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_g, ty_g, where_c) = ast.generics.split_for_impl();

    let variant_body = match &ast.data {
        Data::Enum(e) => {
            let arms = e.variants.iter().enumerate().map(|(i, v)| {
                let vname = &v.ident;
                let pat = match &v.fields {
                    Fields::Unit => quote!(),
                    Fields::Unnamed(_) => quote!((..)),
                    Fields::Named(_) => quote!({ .. }),
                };
                quote!(Self::#vname #pat => #i)
            });
            quote!(match self { #(#arms,)* })
        }
        _ => quote!(0usize),
    };

    quote! {
        impl #impl_g ::zg_world::SystemSet for #name #ty_g #where_c {
            fn variant(&self) -> usize {
                #variant_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_g, ty_g, where_c) = ast.generics.split_for_impl();

    quote! {
        impl #impl_g ::zg_world::Component for #name #ty_g #where_c {
        }
    }
    .into()
}
