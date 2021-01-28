extern crate oapi;
extern crate sppparse;

use oapi::OApi;
use sppparse::SparseRoot;
use std::path::PathBuf;

fn main() {
    let doc: OApi = OApi::new(
        SparseRoot::new_from_file(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/documents/test_docs/openapi.yml"
        )))
        .expect("to parse the openapi"),
    );

    doc.check().expect("not to have logic errors");
    println!("{:#?}", doc);
}
