use quote::quote;

pub fn include_extern_crate(name: &str, new_name: &str) -> proc_macro2::TokenStream {
    let new_crate_name = syn::Ident::new(new_name, proc_macro2::Span::call_site());
    let crate_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    quote! {
        extern crate #new_crate_name as #crate_name;
    }
}
