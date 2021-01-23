use super::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiOperation {
    #[serde(default)]
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    operation_id: Option<String>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
    request_body: Option<OApiRequestBody>,
    #[serde(default)]
    responses: HashMap<String, OApiResponse>,
    #[serde(default)]
    callbacks: HashMap<String, OApiCallback>,
    deprecated: bool,
    #[serde(default)]
    security: HashMap<String, Vec<String>>,
    servers: Vec<OApiServer>,
}

impl OApiCheckTrait for OApiOperation {
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.tags.oapi_check(root, bread_crumb)?;
        self.summary.oapi_check(root, bread_crumb)?;
        self.description.oapi_check(root, bread_crumb)?;
        self.external_docs.oapi_check(root, bread_crumb)?;
        self.operation_id.oapi_check(root, bread_crumb)?;
        self.parameters.oapi_check(root, bread_crumb)?;
        self.request_body.oapi_check(root, bread_crumb)?;
        self.responses.oapi_check(root, bread_crumb)?;
        self.callbacks.oapi_check(root, bread_crumb)?;
        self.deprecated.oapi_check(root, bread_crumb)?;
        self.security.oapi_check(root, bread_crumb)?;
        self.servers.oapi_check(root, bread_crumb)?;
        Ok(())
    }

    fn oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        let mut uniq: HashSet<(&String, &OApiParameterLocation)> = HashSet::new();
        if !self
            .parameters()
            .iter()
            .all(|x| uniq.insert((x.name(), x.in_())))
        {
            bread_crumb.push("parameters".to_string());
            return Err(OApiError::OApiCheck(
                crate::check::connect_bread_crumbs(bread_crumb),
                "Parameters should be unique by name and location".to_string(),
            ));
        }
        self.oapi_check_inner(root, bread_crumb)
    }
}
