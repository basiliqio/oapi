extern crate oapi;
extern crate oapi_proc_macro;

#[macro_export]
macro_rules! oapi_test_path {
    ($e:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $e)
    };
}

mod checks;
mod extensions;
mod parse_valid;

use serde::{Deserialize, Serialize};
use sppparse::SparseRoot;
use std::path::PathBuf;
