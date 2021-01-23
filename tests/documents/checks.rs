use super::*;

#[test]
fn check_semver_too_low() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_semver_too_low.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the semver");
    println!("{}", err);
}

#[test]
fn check_semver_too_high() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_semver_too_high.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the semver");
    println!("{}", err);
}

#[test]
fn check_semver_bad_formatted() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_semver_bad_formatted.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the semver");
    println!("{}", err);
}
