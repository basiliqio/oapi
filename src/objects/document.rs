use super::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[oapi(handler = "self._oapi_check")]
pub struct OApiDocument {
    openapi: Version,
    info: OApiInfo,
    #[serde(default)]
    servers: Option<Vec<OApiServer>>,
    #[serde(default)]
    paths: HashMap<String, OApiPathItem>,
    #[serde(default)]
    components: Option<OApiComponents>,
    #[serde(default)]
    security: HashMap<String, OApiSecurityScheme>,
    #[serde(default)]
    tags: Option<Vec<OApiTag>>,
    #[serde(default)]
    external_docs: Option<OApiExternalDocumentation>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}

impl OApiDocument {
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

    fn check_path_parameters_inner(
        bread_crumb: &mut Vec<String>,
        path: &str,
        parameters: Vec<&OApiParameter>,
    ) -> Result<(), OApiError> {
        let mut uniq: HashSet<(&String, &OApiParameterLocation)> = HashSet::new();
        for param in parameters.into_iter() {
            if !uniq.insert((param.name(), param.in_())) {
                return Err(OApiError::OApiCheck(
                    crate::check::connect_bread_crumbs(bread_crumb),
                    "Parameters should be unique by name and location".to_string(),
                ));
            }
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
    fn check_path_parameters(&self, bread_crumb: &mut Vec<String>) -> Result<(), OApiError> {
        for (path, op) in self.paths().iter() {
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
            Self::check_path_parameters_inner(bread_crumb, path, params)?;
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

// impl<
// 		DocumentExt,
// 		PathItemExt,
// 		OperationExt,
// 		RequestExt,
// 		CallbackExt,
// 		TagExt,
// 		InfoExt,
// 		LicenseExt,
// 		ContactExt,
// 		ComponentsExt,
// 		LinkExt,
// 		ServerExt,
// 		ServerVarExt,
// 		ResponseExt,
// 		ParameterExt,
// 		ExampleExt,
// 		RequestBodyExt,
// 		HeaderExt,
// 		EncodingExt,
// 		MediaTypeExt,
// 		SecuritySchemeApiKeyExt,
// 		SecuritySchemeHttpExt,
// 		SecuritySchemeOauth2Ext,
// 		SecuritySchemeOpenIdConnectExt,
// 		SecuritySchemeOauth2FlowExt,
// 		SecuritySchemeOauth2FlowImplicitExt,
// 		SecuritySchemeOauth2FlowPasswordExt,
// 		SecuritySchemeOauth2FlowClientCredentialsExt,
// 		SecuritySchemeOauth2FlowAuthorizationCodeExt,
// 		ObjectExt,
// 		ArrayExt,
// 		StringExt,
// 		NumericExt,
// 		DiscriminatorExt,
// 		ExternalDocExt,
// 	> OApiCheckTrait
// 	for OApiDocument<
// 		DocumentExt,
// 		PathItemExt,
// 		OperationExt,
// 		RequestExt,
// 		CallbackExt,
// 		TagExt,
// 		InfoExt,
// 		LicenseExt,
// 		ContactExt,
// 		ComponentsExt,
// 		LinkExt,
// 		ServerExt,
// 		ServerVarExt,
// 		ResponseExt,
// 		ParameterExt,
// 		ExampleExt,
// 		RequestBodyExt,
// 		HeaderExt,
// 		EncodingExt,
// 		MediaTypeExt,
// 		SecuritySchemeApiKeyExt,
// 		SecuritySchemeHttpExt,
// 		SecuritySchemeOauth2Ext,
// 		SecuritySchemeOpenIdConnectExt,
// 		SecuritySchemeOauth2FlowExt,
// 		SecuritySchemeOauth2FlowImplicitExt,
// 		SecuritySchemeOauth2FlowPasswordExt,
// 		SecuritySchemeOauth2FlowClientCredentialsExt,
// 		SecuritySchemeOauth2FlowAuthorizationCodeExt,
// 		ObjectExt,
// 		ArrayExt,
// 		StringExt,
// 		NumericExt,
// 		DiscriminatorExt,
// 		ExternalDocExt,
// 	> where
// 	DocumentExt: OApiCheckTrait + SparsableTrait,
// 	PathItemExt: OApiCheckTrait + SparsableTrait,
// 	OperationExt: OApiCheckTrait + SparsableTrait,
// 	RequestExt: OApiCheckTrait + SparsableTrait,
// 	CallbackExt: OApiCheckTrait + SparsableTrait,
// 	TagExt: OApiCheckTrait + SparsableTrait,
// 	InfoExt: OApiCheckTrait + SparsableTrait,
// 	LicenseExt: OApiCheckTrait + SparsableTrait,
// 	ContactExt: OApiCheckTrait + SparsableTrait,
// 	ComponentsExt: OApiCheckTrait + SparsableTrait,
// 	LinkExt: OApiCheckTrait + SparsableTrait,
// 	ServerExt: OApiCheckTrait + SparsableTrait,
// 	ServerVarExt: OApiCheckTrait + SparsableTrait,
// 	ResponseExt: OApiCheckTrait + SparsableTrait,
// 	ParameterExt: OApiCheckTrait + SparsableTrait,
// 	ExampleExt: OApiCheckTrait + SparsableTrait,
// 	RequestBodyExt: OApiCheckTrait + SparsableTrait,
// 	HeaderExt: OApiCheckTrait + SparsableTrait,
// 	EncodingExt: OApiCheckTrait + SparsableTrait,
// 	MediaTypeExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeApiKeyExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeHttpExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2Ext: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOpenIdConnectExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2FlowExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2FlowImplicitExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2FlowPasswordExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2FlowClientCredentialsExt: OApiCheckTrait + SparsableTrait,
// 	SecuritySchemeOauth2FlowAuthorizationCodeExt: OApiCheckTrait + SparsableTrait,
// 	ObjectExt: OApiCheckTrait + SparsableTrait,
// 	ArrayExt: OApiCheckTrait + SparsableTrait,
// 	StringExt: OApiCheckTrait + SparsableTrait,
// 	NumericExt: OApiCheckTrait + SparsableTrait,
// 	DiscriminatorExt: OApiCheckTrait + SparsableTrait,
// 	ExternalDocExt: OApiCheckTrait + SparsableTrait,

// {
// 	fn oapi_check_inner(
// 		&self,
// 		root: &Rc<RefCell<SparseState>>,
// 		bread_crumb: &mut Vec<String>,
// 	) -> Result<(), OApiError> {
// 		self.openapi.oapi_check(root, bread_crumb)?;
// 		self.info.oapi_check(root, bread_crumb)?;
// 		self.servers.oapi_check(root, bread_crumb)?;
// 		self.paths.oapi_check(root, bread_crumb)?;
// 		// self.components.oapi_check(root, bread_crumb)?;
// 		self.security.oapi_check(root, bread_crumb)?;
// 		self.tags.oapi_check(root, bread_crumb)?;
// 		self.external_docs.oapi_check(root, bread_crumb)?;
// 		Ok(())
// 	}

// fn oapi_check(
// 	&self,
// 	root: &Rc<RefCell<SparseState>>,
// 	bread_crumb: &mut Vec<String>,
// ) -> Result<(), OApiError> {
// 	self.check_semver()?;
// 	let mut uniq: HashSet<&String> = HashSet::new();
// 	bread_crumb.push("path".to_string());

// 	for op in self.paths.values() {
// 		let mut resp: Vec<bool> = Vec::with_capacity(8);
// 		Self::check_opid(&mut resp, &mut uniq, op.get().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.delete().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.put().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.patch().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.head().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.post().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.options().as_ref());
// 		Self::check_opid(&mut resp, &mut uniq, op.trace().as_ref());
// 		if !resp.into_iter().all(|a| a) {
// 			return Err(OApiError::OApiCheck(
// 				crate::check::connect_bread_crumbs(bread_crumb),
// 				"OperationId should be unique amongst all the paths".to_string(),
// 			));
// 		}
// 	}
// 	self.check_path_parameters(bread_crumb)?;
// 	bread_crumb.pop();
// 	self.oapi_check_inner(root, bread_crumb)
// }
// }
