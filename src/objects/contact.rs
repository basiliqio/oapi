use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiContact<ContactExt> {
    name: Option<String>,
    url: Option<Url>,
    email: Option<String>,
    #[serde(flatten)]
    extension: ContactExt,
}
