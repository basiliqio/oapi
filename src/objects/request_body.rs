use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiRequestBody<
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
> {
    #[serde(default)]
    description: Option<String>,
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
    >, // TODO Check that key is a valid media type
    #[serde(default)]
    required: bool,
    #[serde(flatten)]
    extension: RequestExt,
}
