extern crate proc_macro;

use quote::quote;

use crate::utils::include_extern_crate;

pub fn oapi_ext_derive_handler(s: synstructure::Structure) -> proc_macro2::TokenStream {
	let oapi_include = proc_macro_crate::crate_name("oapi")
		.map(|v| include_extern_crate("oapi", v.as_str()))
		.unwrap_or_else(|_e| {
			quote! {
				use crate as oapi;
			}
		});
	oapi_ext_derive(s, oapi_include)
}

pub fn oapi_ext_derive_handler_inner(s: synstructure::Structure) -> proc_macro2::TokenStream {
	let oapi_include = quote! {};
	oapi_ext_derive(s, oapi_include)
}

fn oapi_ext_derive(mut s: synstructure::Structure, include_name: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let oapi_include = include_name;
	s.add_bounds(synstructure::AddBounds::Both);
	s.gen_impl(quote! {
		#oapi_include

		gen impl oapi::OApiExtensionExtractor for @Self {
			fn oapi_raw_ext(&self) -> &HashMap<String, Value>
			{
				self._extension()
			}
		}
	})
}
