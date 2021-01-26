use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiResponse<
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
> {
    description: String,
    #[serde(default)]
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
    #[serde(default)]
    content: HashMap<
        String,
        OApiMediaType<
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
    links: HashMap<String, OApiLink<LinkExt, ServerExt, ServerVarExt>>,
    #[serde(flatten)]
    extension: ResponseExt,
}
