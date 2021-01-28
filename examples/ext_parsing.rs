extern crate oapi;
extern crate sppparse;

use oapi::{OApi, OApiExtensionExtractor};
use oapi_proc_macro::OApiCheckInner;
use serde::{Deserialize, Serialize};
use sppparse::Sparsable;
use sppparse::SparseRoot;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize, Sparsable, OApiCheckInner)]
#[serde(rename_all = "camelCase")]
pub struct OApiDummyExt {
    first_name: String,
    last_name: String,
}

fn main() {
    let doc: OApi = OApi::new(
        SparseRoot::new_from_file(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/documents/test_docs/extensions.yml"
        )))
        .expect("to parse the openapi"),
    );

    doc.check().expect("not to have logic errors");
    let ext: OApiDummyExt = doc
        .root_get()
        .unwrap()
        .oapi_extract_ext(doc.doc(), "x-toto")
        .unwrap();
    println!("{:#?}", ext);
}
