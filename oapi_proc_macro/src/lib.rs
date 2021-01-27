#![warn(clippy::all)]
mod check;
mod ext;
mod utils;

use check::{oapi_check_derive_handler, oapi_check_derive_handler_inner};
use ext::{oapi_ext_derive_handler, oapi_ext_derive_handler_inner};
use proc_macro_error::proc_macro_error;

synstructure::decl_derive!([OApiCheck, attributes(oapi)] => #[proc_macro_error] oapi_check_derive_handler);
synstructure::decl_derive!([OApiCheckInner, attributes(oapi)] => #[proc_macro_error] oapi_check_derive_handler_inner);
synstructure::decl_derive!([OApiExt] => #[proc_macro_error] oapi_ext_derive_handler);
synstructure::decl_derive!([OApiExtInner] => #[proc_macro_error] oapi_ext_derive_handler_inner);
