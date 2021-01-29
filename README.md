# OApi

The OpenApi documents parser

<a href="https://gitlab.com/basiliq/oapi/-/pipelines" alt="Gitlab pipeline status">
  <img src="https://img.shields.io/gitlab/pipeline/basiliq/oapi/master">
</a>
<a href="https://codecov.io/gl/basiliq/oapi" alt="Codecov">
  <img src="https://img.shields.io/codecov/c/gitlab/basiliq/oapi?token=20Hr7vtRk4">
</a>
<a href="https://crates.io/crates/oapi" alt="Crates.io version">
  <img src="https://img.shields.io/crates/v/oapi">
</a>
<a href="https://crates.io/crates/oapi" alt="Crates.io license">
  <img src="https://img.shields.io/crates/l/oapi?label=license">
</a>
<a href="https://docs.rs/oapi" alt="Docs.rs">
  <img src="https://docs.rs/oapi/badge.svg">
</a>

- [OApi](#oapi)
	- [Introduction](#introduction)
	- [Example](#example)



## Introduction

This librairy aims to help deserialize and work with OpenApi documents.

Built on top of [serde](https://serde.rs/) and [sppparse](https://crates.io/crates/sppparse), it facilitate parsing
OpenApi document, even with `$ref` pointers and user defined extensions.


## Example

```rust
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
```
