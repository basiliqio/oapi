use super::*;
use sppparse::{SparsePointer, SparseValue};
use std::collections::HashSet;

/// ## The OpenApi document [root object](https://swagger.io/specification/#openapi-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiDocument {
    /// The OAPI version used. Needs to be 3^
    openapi: Version,
    /// Info about the current document
    info: OApiInfo,
    /// List of servers used in this document
    #[serde(default)]
    servers: Option<Vec<OApiServer>>,
    /// List of paths this documents defines.
    #[serde(default)]
    paths: HashMap<String, OApiPathItem>,
    /// An optional object containing components re-used across the document
    #[serde(default)]
    components: Option<OApiComponents>,
    /// The security used for this API
    #[serde(default)]
    security: HashMap<String, SparseSelector<OApiSecurityScheme>>,
    /// Tags used to categorize the operations of this document
    #[serde(default)]
    tags: Option<Vec<OApiTag>>,
    /// Link to some external documentation, if any
    #[serde(default)]
    external_docs: Option<OApiExternalDocumentation>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}

impl OApiDocument {
    /// Checks that the current version of the OpenApi document is ^3.0.0
    fn check_semver(&self) -> Result<(), OApiError> {
        let version_req: VersionReq = VersionReq::parse("^3.0.0").unwrap();
        if !version_req.matches(&self.openapi) {
            return Err(OApiError::OApiCheck(
                "/openapi".to_string(),
                "Only OpenApi ^3 is supported".to_string(),
            ));
        }
        Ok(())
    }

    /// Check if the path parameter are duplicated in a list
    fn check_path_parameter_dup(
        bread_crumb: &mut Vec<String>,
        parameters: &[SparseSelector<OApiParameter>],
    ) -> Result<(), OApiError> {
        let mut uniq: HashSet<(&String, &OApiParameterLocation)> = HashSet::new();
        let mut tmp_store: Vec<SparseValue<'_, OApiParameter>> = Vec::new();
        for param in parameters.iter() {
            tmp_store.push(param.get()?);
        }
        for param in tmp_store.iter() {
            if !uniq.insert((param.name(), param.in_())) {
                return Err(OApiError::OApiCheck(
                    crate::check::connect_bread_crumbs(bread_crumb),
                    format!("Parameters should be unique by name and location. Param `{}` in `{:#?}` is duplicated", param.name(), param.in_()),
                ));
            }
        }
        Ok(())
    }

    /// Checks the path parameters variables used in this document.
    ///
    /// Check if they're defined, they should be present in the path and vice-versa
    fn check_path_parameters_inner(
        bread_crumb: &mut Vec<String>,
        path: &str,
        parameters: Vec<&SparseSelector<OApiParameter>>,
    ) -> Result<(), OApiError> {
        for param in parameters.into_iter() {
            let param = param.get()?;
            if let OApiParameterLocation::Path = param.in_() {
                if !path.contains(format!("{{{}}}", param.name()).as_str()) {
                    bread_crumb.push(path.to_string());
                    return Err(OApiError::OApiCheck(
                        crate::check::connect_bread_crumbs(bread_crumb),
                        format!("Parameter `{{{}}}` is not present in path", param.name()),
                    ));
                }
                if let Some(required) = param.required() {
                    if !*required {
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
        }
        Ok(())
    }

    /// Gather all the path parameters at path and operation level
    /// and check them.
    fn check_path_parameters(&self, bread_crumb: &mut Vec<String>) -> Result<(), OApiError> {
        for (path, op) in self.paths().iter() {
            let mut params: Vec<&SparseSelector<OApiParameter>> = Vec::new();
            Self::check_path_parameter_dup(bread_crumb, op.parameters().as_slice())?;
            params.extend(op.parameters().iter());
            if let Some(x) = op.get() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.head() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.options() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.post() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.put() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.patch() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.delete() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            if let Some(x) = op.trace() {
                Self::check_path_parameter_dup(bread_crumb, x.parameters())?;
                params.extend(x.parameters().iter());
            }
            Self::check_path_parameters_inner(bread_crumb, path, params)?;
        }
        Ok(())
    }

    /// Routine function used to check that an Operation Id is not defined multiple
    /// times. The `op` operation is added to the `uniq` set and the result of that
    /// operation is stored in the `resp` vec. In the end, the `resp` will be evaluated.
    /// Every field needs to be `true`, guaranting that every Operation Id are unique.
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

    /// Get an operation providing its Id
    pub fn get_operation_id(&self, opid_searched: &str) -> Option<&OApiOperation> {
        let mut opid: Vec<Option<&OApiOperation>> = Vec::with_capacity(8);
        for op in self.paths().values() {
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

    /// Perform internal checks of the document, like :
    ///
    /// - Uniqueness of the Operations Id
    /// - Path parameters correctness
    fn _oapi_check(
        &self,
        _root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.check_semver()?;
        let mut uniq: HashSet<&String> = HashSet::new();
        bread_crumb.push("path".to_string());

        for op in self.paths.values() {
            let mut resp: Vec<bool> = Vec::with_capacity(8);
            Self::check_opid(&mut resp, &mut uniq, op.get().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.delete().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.put().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.patch().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.head().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.post().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.options().as_ref());
            Self::check_opid(&mut resp, &mut uniq, op.trace().as_ref());
            if !resp.into_iter().all(|a| a) {
                return Err(OApiError::OApiCheck(
                    crate::check::connect_bread_crumbs(bread_crumb),
                    "OperationId should be unique amongst all the paths".to_string(),
                ));
            }
        }
        self.check_path_parameters(bread_crumb)?;
        bread_crumb.pop();
        Ok(())
    }
}
