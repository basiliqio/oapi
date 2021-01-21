use super::*;

mod api_key_location;
mod components;
mod contact;
mod discriminator;
mod document;
mod encoding;
mod example;
mod example_selector;
mod external_documentation;
mod header;
mod info;
mod license;
mod link;
mod media_type;
mod oauth_flow;
mod oauth_flow_authorization_code;
mod oauth_flow_client_credentials;
mod oauth_flow_implicit;
mod oauth_flow_password;
mod operation;
mod parameter;
mod parameter_location;
mod parameter_style;
mod path_item;
mod request_body;
mod response;
mod security_oauth2;
mod security_open_id_connect;
mod security_scheme;
mod security_scheme_api_key;
mod security_scheme_http;
mod server;
mod server_variable;
mod tag;

pub mod schema;
pub type OApiCallback = HashMap<String, OApiPathItem>;

pub use {
    api_key_location::OApiApiKeyLocation, components::OApiComponents, contact::OApiContact,
    discriminator::OApiDiscriminator, document::OApiDocument, encoding::OApiEncoding,
    example::OApiExample, example_selector::OApiExampleSelector,
    external_documentation::OApiExternalDocumentation, header::OApiHeader, info::OApiInfo,
    license::OApiLicense, link::OApiLink, media_type::OApiMediaType, oauth_flow::OApiOAuthFlow,
    oauth_flow_authorization_code::OApiOAuthFlowAuthorizationCode,
    oauth_flow_client_credentials::OApiOAuthFlowClientCredentials,
    oauth_flow_implicit::OApiOAuthFlowImplicit, oauth_flow_password::OApiOAuthFlowPassword,
    operation::OApiOperation, parameter::OApiParameter, parameter_location::OApiParameterLocation,
    parameter_style::OApiParameterStyle, path_item::OApiPathItem, request_body::OApiRequestBody,
    response::OApiResponse, security_oauth2::OApiSecurityOauth2,
    security_open_id_connect::OApiSecurityOpenIdConnect, security_scheme::OApiSecurityScheme,
    security_scheme_api_key::OApiSecuritySchemeApiKey,
    security_scheme_http::OApiSecuritySchemeHttp, server::OApiServer,
    server_variable::OApiServerVariable, tag::OApiTag,
};
