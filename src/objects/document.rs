use super::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiDocument {
    openapi: String,
    info: OApiInfo,
    servers: Option<Vec<OApiServer>>,
    #[serde(default)]
    path: HashMap<String, OApiPathItem>,
    components: Option<OApiComponents>,
    #[serde(default)]
    security: HashMap<String, OApiSecurityScheme>,
    tags: Option<Vec<OApiTag>>,
    external_docs: Option<OApiExternalDocumentation>,
}

impl OApiDocument {
    fn check_semver(&self) -> Result<(), OApiError> {
        let version: Vec<&str> = self.openapi.split('.').collect();

        if version.len() != 3 {
            return Err(OApiError::OApiCheck(
                "/openapi".to_string(),
                "Wrong semver for OpenApi".to_string(),
            ));
        }
        if version[0] != "3" {
            return Err(OApiError::OApiCheck(
                "/openapi".to_string(),
                "Only OpenApi ^3 is supported".to_string(),
            ));
        }
        Ok(())
    }
    fn check_path_parameters_inner(
        bread_crumb: &mut Vec<String>,
        path: &str,
        parameters: Vec<&OApiParameter>,
    ) -> Result<(), OApiError> {
        for param in parameters.into_iter() {
            if let OApiParameterLocation::Path = param.in_() {
                if !path.contains(format!("{{{}}}", param.name()).as_str()) {
                    bread_crumb.push(path.to_string());
                    return Err(OApiError::OApiCheck(
                        crate::check::connect_bread_crumbs(bread_crumb),
                        format!("Parameter `{{{}}}` is not present in path", param.name()),
                    ));
                }
                if !param.required() {
                    bread_crumb.push(path.to_string());
                    return Err(OApiError::OApiCheck(
                        crate::check::connect_bread_crumbs(bread_crumb),
                        format!(
                            "Parameter `{{{}}}` requirement is mandatory, because it's in path",
                            param.name()
                        ),
                    ));
                }
            }
        }
        Ok(())
    }
    fn check_path_parameters(&self, bread_crumb: &mut Vec<String>) -> Result<(), OApiError> {
        for (path, op) in self.path().iter() {
            let mut params: Vec<&OApiParameter> = Vec::new();
            params.extend(op.parameters().iter());
            if let Some(x) = op.get() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.head() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.options() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.post() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.put() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.patch() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.delete() {
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.trace() {
                params.extend(x.parameters().iter());
            }
            OApiDocument::check_path_parameters_inner(bread_crumb, path, params)?;
        }
        Ok(())
    }

    fn check_opid<'a, 'b>(
        resp: &'b mut Vec<bool>,
        uniq: &mut HashSet<&'a String>,
        op: Option<&'a OApiOperation>,
    ) {
        if let Some(x) = op {
            if let Some(x) = x.operation_id() {
                if !uniq.insert(x) {
                    resp.push(uniq.insert(x));
                }
            }
        }
    }

    pub fn get_operation_id(&self, opid_searched: &str) -> Option<&OApiOperation> {
        let mut opid: Vec<Option<&OApiOperation>> = Vec::with_capacity(8);
        for op in self.path().values() {
            opid.push(op.get().as_ref());
            opid.push(op.head().as_ref());
            opid.push(op.post().as_ref());
            opid.push(op.put().as_ref());
            opid.push(op.patch().as_ref());
            opid.push(op.delete().as_ref());
            opid.push(op.options().as_ref());
            opid.push(op.trace().as_ref());
            for op in opid.drain(..) {
                if let Some(op) = op {
                    if let Some(opid) = op.operation_id() {
                        if opid == opid_searched {
                            return Some(op);
                        }
                    }
                }
            }
        }
        None
    }
}

impl OApiCheckTrait for OApiDocument {
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.openapi.oapi_check(root, bread_crumb)?;
        self.info.oapi_check(root, bread_crumb)?;
        self.servers.oapi_check(root, bread_crumb)?;
        self.path.oapi_check(root, bread_crumb)?;
        self.components.oapi_check(root, bread_crumb)?;
        self.security.oapi_check(root, bread_crumb)?;
        self.tags.oapi_check(root, bread_crumb)?;
        self.external_docs.oapi_check(root, bread_crumb)?;
        Ok(())
    }

    fn oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.check_semver()?;
        let mut uniq: HashSet<&String> = HashSet::new();
        bread_crumb.push("path".to_string());

        for op in self.path.values() {
            let mut resp: Vec<bool> = Vec::with_capacity(9);
            OApiDocument::check_opid(&mut resp, &mut uniq, op.get().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.trace().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.delete().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.put().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.patch().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.head().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.post().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.options().as_ref());
            OApiDocument::check_opid(&mut resp, &mut uniq, op.trace().as_ref());
            if !resp.into_iter().all(|a| a) {
                return Err(OApiError::OApiCheck(
                    crate::check::connect_bread_crumbs(bread_crumb),
                    "OperationId should be unique amongst all the path".to_string(),
                ));
            }
        }
        self.check_path_parameters(bread_crumb)?;
        bread_crumb.pop();
        self.oapi_check_inner(root, bread_crumb)
    }
}
