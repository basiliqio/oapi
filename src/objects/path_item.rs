use super::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    summary: Option<String>,
    description: Option<String>,
    get: Option<OApiOperation>,
    put: Option<OApiOperation>,
    post: Option<OApiOperation>,
    delete: Option<OApiOperation>,
    options: Option<OApiOperation>,
    patch: Option<OApiOperation>,
    head: Option<OApiOperation>,
    trace: Option<OApiOperation>,
    #[serde(default)]
    servers: Vec<OApiServer>,
    #[serde(default)]
    parameters: Vec<OApiParameter>,
}

impl OApiCheckTrait for OApiPathItem {
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.summary.oapi_check(root, bread_crumb)?;
        self.description.oapi_check(root, bread_crumb)?;
        self.get.oapi_check(root, bread_crumb)?;
        self.put.oapi_check(root, bread_crumb)?;
        self.post.oapi_check(root, bread_crumb)?;
        self.delete.oapi_check(root, bread_crumb)?;
        self.options.oapi_check(root, bread_crumb)?;
        self.patch.oapi_check(root, bread_crumb)?;
        self.head.oapi_check(root, bread_crumb)?;
        self.trace.oapi_check(root, bread_crumb)?;
        self.servers.oapi_check(root, bread_crumb)?;
        self.parameters.oapi_check(root, bread_crumb)?;
        Ok(())
    }

    fn oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        let mut uniq: HashSet<(&String, &OApiParameterLocation)> = HashSet::new();
        if !self
            .parameters
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
