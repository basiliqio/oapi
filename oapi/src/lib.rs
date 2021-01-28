//! # Simple parsing
//!
//! A library to easily parse an OpenApi Document according to specifications.
//! It allows for distant references and support the OpenApi operators.
//!
//! For example, parsing a file in your code might you like the following :
//!
//! ```rust
//! extern crate oapi;
//! extern crate sppparse;
//!
//! use oapi::OApi;
//! use sppparse::SparseRoot;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let doc: OApi = OApi::new(
//!         SparseRoot::new_from_file(PathBuf::from(concat!(
//!             env!("CARGO_MANIFEST_DIR"),
//!             "/tests/documents/test_docs/openapi.yml"
//!         )))
//!         .expect("to parse the openapi"),
//!     );
//!
//!     doc.check().expect("not to have logic errors");
//!     println!("{:#?}", doc);
//! }
//! ```
//!
//! # Extension support
//!
//! You can use extension of the OpenApi Document, as specified in the specifications.
//!
//! For example, parsing a file with extension in your code might you like the following :
//!
//!
//! ```rust
//! extern crate oapi;
//! extern crate sppparse;
//!
//! use oapi::{OApi, OApiExtensionExtractor};
//! use sppparse::SparseRoot;
//! use std::path::PathBuf;
//! use serde::{Serialize, Deserialize};
//! use sppparse::Sparsable;
//! use oapi_derive::OApiCheckInner;
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize, Sparsable, OApiCheckInner)]
//! #[serde(rename_all = "camelCase")]
//! pub struct OApiDummyExt {
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! fn main() {
//!     let doc: OApi = OApi::new(
//!         SparseRoot::new_from_file(PathBuf::from(concat!(
//!             env!("CARGO_MANIFEST_DIR"),
//!             "/tests/documents/test_docs/extensions.yml"
//!         )))
//!         .expect("to parse the openapi"),
//!     );
//!
//!     doc.check().expect("not to have logic errors");
//!     let ext: OApiDummyExt = doc
//!         .root_get()
//!         .unwrap()
//!         .oapi_extract_ext(doc.doc(), "x-toto")
//!         .unwrap();
//!     println!("{:#?}", ext);
//! }
//! ```

#![warn(clippy::all)]

use getset::Getters;
use semver::{Version, VersionReq};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sppparse::{
    Sparsable, SparsableTrait, SparseError, SparsePointedValue, SparseRefRawInline, SparseRoot,
    SparseSelector,
};
use std::collections::HashMap;

mod check;
mod error;
mod extension_extractor;
mod oapi_root;
mod objects;
mod operators;

/// ## Derive for OApiCheckTrait
///
/// This will apply the OApiCheckTrait for this object and call `oapi_check` on all its attribute.
///
/// It can be customized using the `#[oapi(handler = "a_function")]` attribute to call a custom checker.
///
/// The default checker returns `Ok(())` by default
pub use oapi_derive::OApiCheck;

pub use check::OApiCheckTrait;
pub use error::OApiError;
use oapi_derive::OApiExt;

pub use extension_extractor::OApiExtensionExtractor;

pub use oapi_root::OApi;

pub use operators::{
    AllOfSelect, AnyOfSelect, NotSelect, OApiOperator, OneOfSelect, OperatorSelector,
};

pub use objects::schema::{
    OApiNumericFormat, OApiNumericMaximum, OApiNumericMinimum, OApiSchema,
    OApiSchemaAdditionalItem, OApiSchemaArray, OApiSchemaDiscriminator, OApiSchemaNumeric,
    OApiSchemaObject, OApiSchemaString, OApiSchemaXml, OApiStringFormat,
};

pub use objects::{
    OApiApiKeyLocation, OApiCallback, OApiComponents, OApiContact, OApiDocument, OApiEncoding,
    OApiExample, OApiExampleSelector, OApiExternalDocumentation, OApiHeader, OApiInfo, OApiLicense,
    OApiLink, OApiMediaType, OApiOAuthFlow, OApiOAuthFlowAuthorizationCode,
    OApiOAuthFlowClientCredentials, OApiOAuthFlowImplicit, OApiOAuthFlowPassword, OApiOperation,
    OApiParameter, OApiParameterLocation, OApiParameterStyle, OApiPathItem, OApiRequestBody,
    OApiResponse, OApiSecurityScheme, OApiSecuritySchemeApiKey, OApiSecuritySchemeHttp,
    OApiSecuritySchemeOauth2, OApiSecuritySchemeOpenIdConnect, OApiServer, OApiServerVariable,
    OApiTag,
};
