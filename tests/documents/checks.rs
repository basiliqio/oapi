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
fn check_parameter_dup_path() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_parameter_dup_path.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the duplicate parameter");
    println!("{}", err);
}

#[test]
fn check_parameter_required_in_path() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_parameter_required_in_path.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the required parameter");
    println!("{}", err);
}

#[test]
fn check_parameter_absent_in_path() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_parameter_absent_path.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the required parameter");
    println!("{}", err);
}

#[test]
fn check_duplicate_operation_id() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_unique_operation_id.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the duplicate operation id");
    println!("{}", err);
}

#[test]
fn check_links_type() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_links_type.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the links");
    println!("{}", err);
}

#[test]
fn check_links_operation_id_exists() {
    let doc: oapi::OApi = oapi::OApi::new(
        SparseRoot::new_from_file(PathBuf::from(oapi_test_path!(
            "tests/documents/test_docs/check_links_operation_id_exists.yml"
        )))
        .expect("to parse the openapi"),
    );

    let err: oapi::OApiError = doc
        .check()
        .expect_err("Expect an error concerning the links");
    println!("{}", err);
}
