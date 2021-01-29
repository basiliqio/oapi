use super::*;

/// ## The OpenApi [path items](https://swagger.io/specification/#path-item-object)
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Default, Sparsable, OApiCheck, OApiExt,
)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiPathItem {
    /// A summary of this path item
    summary: Option<String>,
    /// A description of this path item
    description: Option<String>,
    /// The `GET` operation of this path, if any
    #[serde(default)]
    get: Option<OApiOperation>,
    /// The `PUT` operation of this path, if any
    #[serde(default)]
    put: Option<OApiOperation>,
    /// The `POST` operation of this path, if any
    #[serde(default)]
    post: Option<OApiOperation>,
    /// The `DELETE` operation of this path, if any
    #[serde(default)]
    delete: Option<OApiOperation>,
    /// The `OPTIONS` operation of this path, if any
    #[serde(default)]
    options: Option<OApiOperation>,
    /// The `PATCH` operation of this path, if any
    #[serde(default)]
    patch: Option<OApiOperation>,
    /// The `HEAD` operation of this path, if any
    #[serde(default)]
    head: Option<OApiOperation>,
    /// The `TRACE` operation of this path, if any
    #[serde(default)]
    trace: Option<OApiOperation>,
    /// The servers this applies to
    #[serde(default)]
    servers: Vec<OApiServer>,
    /// The parameters to use
    #[serde(default)]
    parameters: Vec<SparseSelector<OApiParameter>>,
    /// Extensions, if any
    #[serde(flatten)]
    #[getset(get)]
    _extension: HashMap<String, Value>,
}
