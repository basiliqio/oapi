use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiEncoding<
    EncodingExt,
    HeaderExt,
    ExampleExt,
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    content_type: Option<String>,
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
    style: Option<String>,
    #[serde(default)]
    explode: bool,
    #[serde(default)]
    allow_reserved: bool,
    #[serde(flatten)]
    extension: EncodingExt,
}
