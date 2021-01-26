use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiLicense<LicenseExt> {
    name: String,
    url: Option<Url>,
    #[serde(flatten)]
    extension: LicenseExt,
}
