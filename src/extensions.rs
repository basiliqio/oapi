use super::*;

pub trait OApiExtensionRequirements:
    'static
    + Clone
    + PartialEq
    + std::fmt::Debug
    + SparsableTrait
    + Clone
    + Default
    + Serialize
    + DeserializeOwned // + OApiCheckTrait
{
}
pub type OApiEncodingDefaultExtension = HashMap<String, Value>;
pub type OApiOauthFlowDefaultExtension = HashMap<String, Value>;
pub type OApiDocumentDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2DefaultExtension = HashMap<String, Value>;
pub type OApiLicenseDefaultExtension = HashMap<String, Value>;
pub type OApiInfoDefaultExtension = HashMap<String, Value>;
pub type OApiParameterDefaultExtension = HashMap<String, Value>;
pub type OApiComponentsDefaultExtension = HashMap<String, Value>;
pub type OApiOauthFlowAuthorizationCodeDefaultExtension = HashMap<String, Value>;
pub type OApiPathItemDefaultExtension = HashMap<String, Value>;
pub type OApiExternalDocumentationDefaultExtension = HashMap<String, Value>;
pub type OApiResponseDefaultExtension = HashMap<String, Value>;
pub type OApiMediaTypeDefaultExtension = HashMap<String, Value>;
pub type OApiDiscriminatorDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaObjectDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaArrayDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaXmlDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaDiscriminatorDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaStringDefaultExtension = HashMap<String, Value>;
pub type OApiSchemaNumericDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOpenIdConnectDefaultExtension = HashMap<String, Value>;
pub type OApiExampleSelectorDefaultExtension = HashMap<String, Value>;
pub type OApiRequestBodyDefaultExtension = HashMap<String, Value>;
pub type OApiLinkDefaultExtension = HashMap<String, Value>;
pub type OApiServerVariableDefaultExtension = HashMap<String, Value>;
pub type OApiOperationDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeHttpDefaultExtension = HashMap<String, Value>;
pub type OApiContactDefaultExtension = HashMap<String, Value>;
pub type OApiOauthFlowImplicitDefaultExtension = HashMap<String, Value>;
pub type OApiServerDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeApiKeyDefaultExtension = HashMap<String, Value>;
pub type OApiOauthFlowClientCredentialsDefaultExtension = HashMap<String, Value>;
pub type OApiTagDefaultExtension = HashMap<String, Value>;
pub type OApiExampleDefaultExtension = HashMap<String, Value>;
pub type OApiHeaderDefaultExtension = HashMap<String, Value>;
pub type OApiOauthFlowPasswordDefaultExtension = HashMap<String, Value>;
pub type OApiCallbackDefaultExtension = HashMap<String, Value>;
pub type OApiServerVarDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2FlowDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2FlowImplicitDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2FlowPasswordDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2FlowClientCredentialsDefaultExtension = HashMap<String, Value>;
pub type OApiSecuritySchemeOauth2FlowAuthorizationCodeDefaultExtension = HashMap<String, Value>;
pub type OApiObjectDefaultExtension = HashMap<String, Value>;
pub type OApiArrayDefaultExtension = HashMap<String, Value>;
pub type OApiStringDefaultExtension = HashMap<String, Value>;
pub type OApiNumericDefaultExtension = HashMap<String, Value>;
pub type OApiExternalDocDefaultExtension = HashMap<String, Value>;
