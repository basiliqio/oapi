#![warn(clippy::all)]

use getset::Getters;
use semver::{Version, VersionReq};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sppparse::{
    Sparsable, SparsableTrait, SparseError, SparsePointedValue, SparseRoot, SparseSelector,
    SparseState,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use url::Url;

mod check;
mod error;
mod extensions;
mod oapi_root;
mod objects;
mod operators;

pub use check::OApiCheckTrait;
pub use error::OApiError;
pub use oapi_proc_macro::OApiCheck;

pub use oapi_root::OApi;

pub use extensions::{
    OApiArrayDefaultExtension, OApiCallbackDefaultExtension, OApiComponentsDefaultExtension,
    OApiContactDefaultExtension, OApiDiscriminatorDefaultExtension, OApiDocumentDefaultExtension,
    OApiEncodingDefaultExtension, OApiExampleDefaultExtension, OApiExampleSelectorDefaultExtension,
    OApiExternalDocDefaultExtension, OApiExternalDocumentationDefaultExtension,
    OApiHeaderDefaultExtension, OApiInfoDefaultExtension, OApiLicenseDefaultExtension,
    OApiLinkDefaultExtension, OApiMediaTypeDefaultExtension, OApiNumericDefaultExtension,
    OApiOauthFlowAuthorizationCodeDefaultExtension, OApiOauthFlowClientCredentialsDefaultExtension,
    OApiOauthFlowDefaultExtension, OApiOauthFlowImplicitDefaultExtension,
    OApiOauthFlowPasswordDefaultExtension, OApiObjectDefaultExtension,
    OApiOperationDefaultExtension, OApiParameterDefaultExtension, OApiPathItemDefaultExtension,
    OApiRequestBodyDefaultExtension, OApiResponseDefaultExtension, OApiSchemaArrayDefaultExtension,
    OApiSchemaDefaultExtension, OApiSchemaDiscriminatorDefaultExtension,
    OApiSchemaNumericDefaultExtension, OApiSchemaObjectDefaultExtension,
    OApiSchemaStringDefaultExtension, OApiSchemaXmlDefaultExtension,
    OApiSecuritySchemeApiKeyDefaultExtension, OApiSecuritySchemeHttpDefaultExtension,
    OApiSecuritySchemeOauth2DefaultExtension,
    OApiSecuritySchemeOauth2FlowAuthorizationCodeDefaultExtension,
    OApiSecuritySchemeOauth2FlowClientCredentialsDefaultExtension,
    OApiSecuritySchemeOauth2FlowDefaultExtension,
    OApiSecuritySchemeOauth2FlowImplicitDefaultExtension,
    OApiSecuritySchemeOauth2FlowPasswordDefaultExtension,
    OApiSecuritySchemeOpenIdConnectDefaultExtension, OApiServerDefaultExtension,
    OApiServerVarDefaultExtension, OApiServerVariableDefaultExtension, OApiStringDefaultExtension,
    OApiTagDefaultExtension,
};

pub use operators::{
    AllOfSelect, AnyOfSelect, NotSelect, OApiOperator, OneOfSelect, OperatorSelector,
};

pub use objects::schema::{
    OApiNumericFormat, OApiNumericMaximum, OApiNumericMinimum, OApiSchema,
    OApiSchemaAdditionalItem, OApiSchemaArray, OApiSchemaDiscriminator, OApiSchemaNumeric,
    OApiSchemaObject, OApiSchemaString, OApiSchemaXml, OApiStringFormat,
};

pub use objects::{
    OApiApiKeyLocation, OApiCallback, OApiComponents, OApiContact, OApiDiscriminator, OApiDocument,
    OApiEncoding, OApiExample, OApiExampleSelector, OApiExternalDocumentation, OApiHeader,
    OApiInfo, OApiLicense, OApiLink, OApiMediaType, OApiOAuthFlow, OApiOAuthFlowAuthorizationCode,
    OApiOAuthFlowClientCredentials, OApiOAuthFlowImplicit, OApiOAuthFlowPassword, OApiOperation,
    OApiParameter, OApiParameterLocation, OApiParameterStyle, OApiPathItem, OApiRequestBody,
    OApiResponse, OApiSecurityScheme, OApiSecuritySchemeApiKey, OApiSecuritySchemeHttp,
    OApiSecuritySchemeOauth2, OApiSecuritySchemeOpenIdConnect, OApiServer, OApiServerVariable,
    OApiTag,
};
