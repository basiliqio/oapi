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
    fn oapi_check(
        &self,
        _root: &SparseRoot<OApiDocument>,
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
        Ok(())
    }
}
