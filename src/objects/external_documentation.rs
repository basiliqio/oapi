use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiExternalDocumentation<ExternalDocExt> {
    url: Url,
    #[serde(default)]
    description: Option<String>,
    #[serde(flatten)]
    extension: ExternalDocExt,
}
