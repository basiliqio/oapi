use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag<TagExt, ExternalDocExt> {
    name: String,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation<ExternalDocExt>>,
    #[serde(flatten)]
    extension: TagExt,
}
