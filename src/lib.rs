#![warn(clippy::all)]

use getset::Getters;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sppparse::{Sparsable, SparsableTrait, SparseError, SparsePointedValue, SparseSelector};
use std::collections::HashMap;
use url::Url;

mod objects;
mod operators;

pub use operators::{
    AllOfSelect, AnyOfSelect, NotSelect, OApiOperator, OneOfSelect, OperatorSelector,
};

pub use objects::schema::{
    OApiNumericFormat, OApiNumericMaximum, OApiNumericMinimum, OApiSchema,
    OApiSchemaAdditionalItem, OApiSchemaArray, OApiSchemaCommon, OApiSchemaDiscriminator,
    OApiSchemaNumeric, OApiSchemaObject, OApiSchemaString, OApiSchemaXml, OApiStringFormat,
};

pub use objects::{
    OApiApiKeyLocation, OApiCallback, OApiComponents, OApiContact, OApiDiscriminator, OApiDocument,
    OApiEncoding, OApiExample, OApiExampleSelector, OApiExternalDocumentation, OApiHeader,
    OApiInfo, OApiLicense, OApiLink, OApiMediaType, OApiOAuthFlow, OApiOAuthFlowAuthorizationCode,
    OApiOAuthFlowClientCredentials, OApiOAuthFlowImplicit, OApiOAuthFlowPassword, OApiOperation,
    OApiParameter, OApiParameterLocation, OApiParameterStyle, OApiPathItem, OApiRequestBody,
    OApiResponse, OApiSecurityOauth2, OApiSecurityOpenIdConnect, OApiSecurityScheme,
    OApiSecuritySchemeApiKey, OApiSecuritySchemeHttp, OApiServer, OApiServerVariable, OApiTag,
};
