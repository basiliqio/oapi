use super::*;
use oapi::{OApiError, OApiExtensionExtractor};
use oapi_proc_macro::OApiCheckInner;
use sppparse::Sparsable;

#[derive(Debug, PartialEq, Serialize, Deserialize, Sparsable, OApiCheckInner)]
#[serde(rename_all = "camelCase")]
pub struct OApiDummyExt {
    first_name: String,
    last_name: String,
}

#[test]
fn ok() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/extensions.yml"
        )))
        .expect("to parse the openapi"),
    );

    let ext: OApiDummyExt = doc
        .root_get()
        .unwrap()
        .oapi_extract_ext(doc.doc(), "x-toto")
        .unwrap();
    assert_eq!(ext.first_name.as_str(), "hello", "first name mismatch");
    assert_eq!(ext.last_name.as_str(), "world", "last name mismatch");
}

#[test]
fn bad_format() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/extensions.yml"
        )))
        .expect("to parse the openapi"),
    );

    let ext: OApiError = doc
        .root_get()
        .unwrap()
        .info()
        .oapi_extract_ext::<OApiDummyExt>(doc.doc(), "x-toto")
        .expect_err("Shoulve failed validation");
    match ext {
        OApiError::SppparseError(sppparse::SparseError::SerdeJson(_)) => (),
        _ => panic!("Wrong error type, was expecting serde json error"),
    };
}

#[test]
fn bad_name() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/extensions.yml"
        )))
        .expect("to parse the openapi"),
    );

    let ext: OApiError = doc
        .root_get()
        .unwrap()
        .oapi_extract_ext::<OApiDummyExt>(doc.doc(), "x-hello")
        .expect_err("Shoulve failed validation");
    match ext {
        OApiError::NoExtKey(s) => assert_eq!(s.as_str(), "x-hello", "key mismatch"),
        _ => panic!("Wrong error type, was expecting serde json error"),
    };
}
