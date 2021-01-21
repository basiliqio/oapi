use super::*;

#[test]
fn petstore() {
	let doc: SparseRoot<oapi::OApiDocument> = SparseRoot::new_from_file(PathBuf::from(
        oapi_test_path!("tests/documents/test_docs/openapi.json"),
    ))
    .expect("to parse the openapi");

    println!("{:#?}", doc);
}

#[test]
fn petstore_bundled() {
    let doc: SparseRoot<oapi::OApiDocument> = SparseRoot::new_from_file(PathBuf::from(
        oapi_test_path!("tests/documents/test_docs/openapi_bundled.json"),
    ))
    .expect("to parse the openapi");

    println!("{:#?}", doc);
}
