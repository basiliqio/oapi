extern crate proc_macro;

use quote::quote;
use synstructure::BindStyle;

fn include_extern_crate(name: &str, new_name: &str) -> proc_macro2::TokenStream {
	let new_crate_name = syn::Ident::new(new_name, proc_macro2::Span::call_site());
	let crate_name = syn::Ident::new(name, proc_macro2::Span::call_site());
	quote! {
		extern crate #new_crate_name as #crate_name;
	}
}

pub fn oapi_check_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
	let body = s.bind_with(|_bi| BindStyle::Ref).each(|bi| {
		let bi_string = match &bi.ast().ident
		{
			Some(x) => x.to_string(),
			None => "<unamed>".to_string()
		};
		quote! {
			bread_crumb.push(#bi_string.to_string());
			#bi.oapi_check(root, bread_crumb)?;
			bread_crumb.pop();
		}
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
	s.gen_impl(quote! {
		#oapi_include
		#sppparse_include

		gen impl oapi::OApiCheckTrait for @Self {
			fn oapi_check(&self, root: &sppparse::SparseRoot<oapi::OApiDocument>, bread_crumb: &mut Vec<String>) -> Result<(), oapi::OApiError>
			{
				match *self { #body };
				Ok(())
			}
		}
	})
}
