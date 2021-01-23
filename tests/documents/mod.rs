extern crate oapi;

#[macro_export]
macro_rules! oapi_test_path {
    ($e:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $e)
    };
}

mod checks;
mod parse_valid;

use sppparse::SparseRoot;
use std::path::PathBuf;
