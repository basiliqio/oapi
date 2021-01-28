use super::*;
use sppparse::{SparsePointer, SparseValue};

/// ## OApi Operator trait
///
/// This trait is implemented by the operators.
/// The operators allows to aggregate multiple part of a schema to form one.
pub trait OApiOperator<T> {
    /// Get the values pointed by the operator
    fn get(&self) -> Result<Vec<SparseValue<T>>, SparseError>;
    /// Create a operator providing values
    fn new(val: Vec<OperatorSelector<T>>) -> Self;
}

macro_rules! OApiOperatorImpl {
    ($struct_name:ident) => {
        impl<T> OApiOperator<T> for $struct_name<T>
        where
            T: 'static + Serialize + DeserializeOwned + SparsableTrait,
        {
            fn get(&self) -> Result<Vec<SparseValue<T>>, SparseError> {
                let mut res: Vec<SparseValue<T>> = Vec::new();

                for v in self.root().iter() {
                    res.append(&mut v.get()?);
                }
                Ok(res)
            }

            fn new(val: Vec<OperatorSelector<T>>) -> Self {
                $struct_name { root: val }
            }
        }
    };
}

/// ## `anyOf` operator
#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq, OApiCheck)]
pub struct AnyOfSelect<T> {
    #[getset(get)]
    #[serde(rename = "anyOf")]
    root: Vec<OperatorSelector<T>>,
}

/// ## `oneOf` operator
#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq, OApiCheck)]
pub struct OneOfSelect<T> {
    #[getset(get)]
    #[serde(rename = "oneOf")]
    root: Vec<OperatorSelector<T>>,
}

/// ## `allOf` operator
#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq, OApiCheck)]
pub struct AllOfSelect<T> {
    #[getset(get)]
    #[serde(rename = "allOf")]
    root: Vec<OperatorSelector<T>>,
}

/// ## `not` operator
#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq, OApiCheck)]
pub struct NotSelect<T> {
    #[getset(get)]
    #[serde(rename = "not")]
    root: Vec<OperatorSelector<T>>,
}

OApiOperatorImpl!(AnyOfSelect);
OApiOperatorImpl!(OneOfSelect);
OApiOperatorImpl!(AllOfSelect);
OApiOperatorImpl!(NotSelect);

/// ## A selector between operator or value
///
/// This selector either resolve to an operator, to a reference to the value or to the value.
/// It's recursive and allows nested objects, operators and references.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum OperatorSelector<T> {
    AnyOf(AnyOfSelect<T>),
    OneOf(OneOfSelect<T>),
    AllOf(AllOfSelect<T>),
    Not(NotSelect<T>),
    Val(SparseSelector<T>),
}

impl<T> std::default::Default for OperatorSelector<T>
where
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Default,
{
    fn default() -> Self {
        OperatorSelector::new_from_val(T::default())
    }
}

impl<T> OperatorSelector<T>
where
    T: 'static + Serialize + DeserializeOwned + SparsableTrait,
{
    pub fn get(&self) -> Result<Vec<SparseValue<T>>, SparseError> {
        match self {
            OperatorSelector::AnyOf(x) => x.get(),
            OperatorSelector::OneOf(x) => x.get(),
            OperatorSelector::AllOf(x) => x.get(),
            OperatorSelector::Not(x) => x.get(),
            OperatorSelector::Val(x) => Ok(vec![x.get()?]),
        }
    }

    pub fn new_from_val(val: T) -> Self {
        OperatorSelector::Val(SparseSelector::Obj(SparsePointedValue::Obj(val)))
    }
}

impl<T> SparsableTrait for OperatorSelector<T>
where
    T: 'static + Serialize + DeserializeOwned + SparsableTrait,
{
    fn sparse_init(
        &mut self,
        state: &mut sppparse::SparseState,
        metadata: &sppparse::SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        let ndepth = depth + 1;
        match self {
            OperatorSelector::AnyOf(x) => x.sparse_init(state, metadata, ndepth),
            OperatorSelector::OneOf(x) => x.sparse_init(state, metadata, ndepth),
            OperatorSelector::AllOf(x) => x.sparse_init(state, metadata, ndepth),
            OperatorSelector::Not(x) => x.sparse_init(state, metadata, ndepth),
            OperatorSelector::Val(x) => x.sparse_init(state, metadata, ndepth),
        }
    }
}

impl<T> OApiCheckTrait for OperatorSelector<T>
where
    T: OApiCheckTrait,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        match self {
            OperatorSelector::AnyOf(x) => x.oapi_check(root, bread_crumb),
            OperatorSelector::OneOf(x) => x.oapi_check(root, bread_crumb),
            OperatorSelector::AllOf(x) => x.oapi_check(root, bread_crumb),
            OperatorSelector::Not(x) => x.oapi_check(root, bread_crumb),
            OperatorSelector::Val(x) => x.oapi_check(root, bread_crumb),
        }
    }

    fn oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.oapi_check_inner(root, bread_crumb)
    }
}
