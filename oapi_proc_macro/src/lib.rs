#![warn(clippy::all)]
mod check;

use check::{oapi_check_derive};
use proc_macro_error::proc_macro_error;

synstructure::decl_derive!([OApiCheck, attributes(oapi)] => #[proc_macro_error] oapi_check_derive);
