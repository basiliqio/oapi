use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation<
    OperationExt,
    ParameterExt,
    ExampleExt,
    MediaTypeExt,
    HeaderExt,
    EncodingExt,
    LinkExt,
    RequestExt,
    ResponseExt,
    CallbackExt,
    ServerExt,
    ServerVarExt,
    ExternalDocExt,
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
> {
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation<ExternalDocExt>>,
    #[serde(default)]
    operation_id: Option<String>,
    #[serde(default)]
    parameters: Vec<
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
    request_body: Option<
        OApiRequestBody<
            RequestExt,
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
    #[serde(default)]
    callbacks: HashMap<
        String,
        OApiCallback<
            OperationExt,
            ParameterExt,
            ExampleExt,
            MediaTypeExt,
            HeaderExt,
            EncodingExt,
            LinkExt,
            RequestExt,
            ResponseExt,
            CallbackExt,
            ServerExt,
            ServerVarExt,
            ExternalDocExt,
            ObjectExt,
            ArrayExt,
            NumericExt,
            StringExt,
            DiscriminatorExt,
        >,
    >,
    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    security: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    servers: Vec<OApiServer<ServerExt, ServerVarExt>>,
    #[serde(flatten)]
    extension: OperationExt,
}
