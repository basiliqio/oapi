extern crate proc_macro;

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;
use synstructure::BindStyle;

fn include_extern_crate(name: &str, new_name: &str) -> proc_macro2::TokenStream {
	let new_crate_name = syn::Ident::new(new_name, proc_macro2::Span::call_site());
	let crate_name = syn::Ident::new(name, proc_macro2::Span::call_site());
	quote! {
		extern crate #new_crate_name as #crate_name;
	}
}

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

pub fn oapi_check_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
	let attribute = find_attr(&s.ast().attrs);
	let handler = attribute.map(|attribute| {
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
	});

	let body = s.bind_with(|_bi| BindStyle::Ref).each(|bi| {
		let bi_string = match &bi.ast().ident {
			Some(x) => x.to_string(),
			None => "<unamed>".to_string(),
		};
		let fnc_call = match handler.clone() {
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
				}
			}
			None => {
				quote! {
					#bi.oapi_check_inner(root, bread_crumb)?;
				}
			}
		};
		let res = quote! {
			bread_crumb.push(#bi_string.to_string());
			#fnc_call
			bread_crumb.pop();
		};
		res
	});

	let oapi_include = proc_macro_crate::crate_name("oapi")
		.map(|v| include_extern_crate("oapi", v.as_str()))
		.unwrap_or_else(|_e| {
			quote! {
				use crate as oapi;
			}
		});
	let sppparse_include = proc_macro_crate::crate_name("sppparse")
		.map(|v| include_extern_crate("sppparse", v.as_str()))
		.unwrap_or_else(|_e| {
			quote! {
				use crate as sppparse;
			}
		});
	s.add_bounds(synstructure::AddBounds::Both);
	s.gen_impl(quote! {
		#oapi_include
		#sppparse_include

		gen impl oapi::OApiCheckTrait for @Self {
			fn oapi_check_inner(&self, root: &::std::rc::Rc<::std::cell::RefCell<sppparse::SparseState>>, bread_crumb: &mut Vec<String>) -> Result<(), oapi::OApiError>
			{
				match *self { #body };
				Ok(())
			}
		}
	})
}
