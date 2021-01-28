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
pub use oapi_proc_macro::OApiCheck;

pub use check::OApiCheckTrait;
pub use error::OApiError;
use oapi_proc_macro::OApiExt;

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
