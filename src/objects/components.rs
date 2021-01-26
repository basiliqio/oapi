use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, Default)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OApiComponents<
    ComponentsExt,
    LinkExt,
    ServerExt,
    ServerVarExt,
    ResponseExt,
    ParameterExt,
    ExampleExt,
    RequestBodyExt,
    HeaderExt,
    EncodingExt,
    MediaTypeExt,
    SecuritySchemeApiKeyExt,
    SecuritySchemeHttpExt,
    SecuritySchemeOauth2Ext,
    SecuritySchemeOpenIdConnectExt,
    SecuritySchemeOauth2FlowExt,
    SecuritySchemeOauth2FlowImplicitExt,
    SecuritySchemeOauth2FlowPasswordExt,
    SecuritySchemeOauth2FlowClientCredentialsExt,
    SecuritySchemeOauth2FlowAuthorizationCodeExt,
    ObjectExt,
    ArrayExt,
    StringExt,
    NumericExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    #[serde(default)]
    links: HashMap<String, OApiLink<LinkExt, ServerExt, ServerVarExt>>,
    schemas: HashMap<
        String,
        OperatorSelector<
            OApiSchema<
                ObjectExt,
                ArrayExt,
                NumericExt,
                StringExt,
                DiscriminatorExt,
                ExternalDocExt,
            >,
        >,
    >,
    #[serde(default)]
    responses: HashMap<
        String,
        OApiResponse<
            ResponseExt,
            HeaderExt,
            MediaTypeExt,
            LinkExt,
            ExampleExt,
            EncodingExt,
            ServerExt,
            ServerVarExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    parameters: HashMap<
        String,
        OApiParameter<
            ParameterExt,
            ExampleExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    #[serde(default)]
    examples: HashMap<String, OApiExampleSelector<ExampleExt>>,
    request_bodies: HashMap<
        String,
        OApiRequestBody<
            RequestBodyExt,
            MediaTypeExt,
            ExampleExt,
            EncodingExt,
            HeaderExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    headers: HashMap<
        String,
        OApiHeader<
            HeaderExt,
            ExampleExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
            ExternalDocExt,
        >,
    >,
    security_schemes: HashMap<
        String,
        OApiSecurityScheme<
            SecuritySchemeApiKeyExt,
            SecuritySchemeHttpExt,
            SecuritySchemeOauth2Ext,
            SecuritySchemeOpenIdConnectExt,
            SecuritySchemeOauth2FlowExt,
            SecuritySchemeOauth2FlowImplicitExt,
            SecuritySchemeOauth2FlowPasswordExt,
            SecuritySchemeOauth2FlowClientCredentialsExt,
            SecuritySchemeOauth2FlowAuthorizationCodeExt,
        >,
    >,
    // callbacks: HashMap<String, OApiCallback>,
    #[serde(flatten)]
    extension: ComponentsExt,
}
