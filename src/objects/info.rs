use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiInfo<InfoExt, LicenseExt, ContactExt> {
    title: String,
    description: Option<String>,
    terms_of_services: Option<String>,
    contact: Option<OApiContact<ContactExt>>,
    license: Option<OApiLicense<LicenseExt>>,
    version: String,
    #[serde(flatten)]
    extension: InfoExt,
}
