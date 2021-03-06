extern crate proc_macro;

use crate::utils::include_extern_crate;
use proc_macro2::{Group, Span, TokenStream, TokenTree};
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;
use synstructure::BindStyle;

// Inspired by https://gitlab.com/torkleyy/err-derive/-/blob/0ee9570a094e0aebc1d62000909213f7609adf31/src/lib.rs#L298
fn find_attr(attrs: &[syn::Attribute]) -> Option<syn::MetaList> {
    let mut handler = None;
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if meta
                .path()
                .get_ident()
                .map_or(false, |ident| ident == "oapi")
            {
                let span = attr.span();
                if handler.is_some() {
                    abort!(span, "Cannot have two display attributes")
                } else if let syn::Meta::List(list) = meta {
                    handler = Some(list);
                } else {
                    abort!(span, "error attribute must take a list in parentheses")
                }
            }
        }
    }
    handler
}

// Borrowed from `serde` : https://github.com/serde-rs/serde/blob/b054ea41053ea4047882cc33970d2257cdfe04ac/serde_derive/src/internals/respan.rs
fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}

fn find_handler(attribute: std::option::Option<syn::MetaList>) -> Option<syn::Lit> {
    attribute.map(|attribute| {
        let handler: syn::Lit = match attribute.nested[0] {
            syn::NestedMeta::Meta(syn::Meta::NameValue(ref nv))
                if nv
                    .path
                    .get_ident()
                    .map_or(false, |ident| ident == "handler") =>
            {
                nv.lit.clone()
            }
            _ => abort!(
                attribute.nested.span(),
                "Error attribute must begin `handler = <fnc>` to control the handler function."
            ),
        };
        handler
    })
}

fn create_check_body(handler: Option<syn::Lit>) -> TokenStream {
    match handler {
        Some(handler) => {
            let ident: TokenStream = match handler
			{
				syn::Lit::Str(s) => syn::parse2(
					respan(
						syn::parse_str(&s.value())
						.expect("to parse the function name"), s.span()))
						.expect("the function name to be valid"),
				_ => abort!(handler.span(), "The handler function should be a string\ni.e. #[oapi(handler = handler_function)]")
			};
            quote! {
                #ident(root, bread_crumb)?;
                self.oapi_check_inner(root, bread_crumb)
            }
        }
        None => {
            quote! {
                self.oapi_check_inner(root, bread_crumb)
            }
        }
    }
}

pub fn oapi_check_derive_handler(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let oapi_include = proc_macro_crate::crate_name("oapi")
        .map(|v| include_extern_crate("oapi", v.as_str()))
        .unwrap_or_else(|_e| {
            quote! {
                use crate as oapi;
            }
        });
    oapi_check_derive(s, oapi_include)
}

pub fn oapi_check_derive_handler_inner(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let oapi_include = quote! {};
    oapi_check_derive(s, oapi_include)
}

fn oapi_check_derive(
    mut s: synstructure::Structure,
    include_name: TokenStream,
) -> proc_macro2::TokenStream {
    let attribute = find_attr(&s.ast().attrs);
    let handler = find_handler(attribute);
    let check_body = create_check_body(handler);
    let body = s.bind_with(|_bi| BindStyle::Ref).each(|bi| {
        let bi_string = match &bi.ast().ident {
            Some(x) => x.to_string(),
            None => "<unamed>".to_string(),
        };
        let res = quote! {
            bread_crumb.push(#bi_string.to_string());
            #bi.oapi_check(root, bread_crumb)?;
            bread_crumb.pop();
        };
        res
    });

    let oapi_include = include_name;
    let sppparse_include = proc_macro_crate::crate_name("sppparse")
        .map(|v| include_extern_crate("sppparse", v.as_str()))
        .unwrap_or_else(|_e| {
            quote! {
                extern crate sppparse;
            }
        });
    s.add_bounds(synstructure::AddBounds::Both);
    s.gen_impl(quote! {
		#oapi_include
		#sppparse_include

		gen impl oapi::OApiCheckTrait for @Self {
			fn oapi_check(&self, root: &sppparse::SparseRoot<oapi::OApiDocument>, bread_crumb: &mut Vec<String>) -> Result<(), oapi::OApiError>
			{
				#check_body
			}
			fn oapi_check_inner(&self, root: &sppparse::SparseRoot<oapi::OApiDocument>, bread_crumb: &mut Vec<String>) -> Result<(), oapi::OApiError>
			{
				match *self { #body };
				Ok(())
			}
		}
	})
}
