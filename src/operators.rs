use super::*;
use sppparse::{SparsePointer, SparseValue};

pub trait OApiOperator<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
>
{
    fn get(&self) -> Result<Vec<SparseValue<T>>, SparseError>;
    fn new(val: Vec<OperatorSelector<T>>) -> Self;
}

macro_rules! OApiOperatorImpl {
    ($struct_name:ident) => {
        impl<T> OApiOperator<T> for $struct_name<T>
        where
            T: 'static
                + Serialize
                + DeserializeOwned
                + SparsableTrait
                + Clone
                + std::fmt::Debug
                + PartialEq,
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

#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq)]
#[serde(
    bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait + Clone + std::fmt::Debug + PartialEq"
)]
pub struct AnyOfSelect<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
> {
    #[getset(get)]
    #[serde(rename = "anyOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
pub struct OneOfSelect<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
> {
    #[getset(get)]
    #[serde(rename = "oneOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq)]
#[serde(
    bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait + Clone + std::fmt::Debug + PartialEq"
)]
pub struct AllOfSelect<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
> {
    #[getset(get)]
    #[serde(rename = "allOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable, Clone, Debug, PartialEq)]
#[serde(
    bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait + Clone + std::fmt::Debug + PartialEq"
)]
pub struct NotSelect<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
> {
    #[getset(get)]
    #[serde(rename = "not")]
    root: Vec<OperatorSelector<T>>,
}

OApiOperatorImpl!(AnyOfSelect);
OApiOperatorImpl!(OneOfSelect);
OApiOperatorImpl!(AllOfSelect);
OApiOperatorImpl!(NotSelect);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(
    bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait + Clone + std::fmt::Debug + PartialEq"
)]
#[serde(untagged)]
pub enum OperatorSelector<
    T: 'static + Serialize + DeserializeOwned + SparsableTrait + Clone + std::fmt::Debug + PartialEq,
> {
    AnyOf(AnyOfSelect<T>),
    OneOf(OneOfSelect<T>),
    AllOf(AllOfSelect<T>),
    Not(NotSelect<T>),
    Val(SparseSelector<T>),
}

impl<T> std::default::Default for OperatorSelector<T>
where
    T: 'static
        + Serialize
        + DeserializeOwned
        + SparsableTrait
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Default,
{
    fn default() -> Self {
        OperatorSelector::new_from_val(T::default())
    }
}

impl<T> OperatorSelector<T>
where
    T: 'static
        + Serialize
        + DeserializeOwned
        + SparsableTrait
        + Clone
        + std::fmt::Debug
        + PartialEq,
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
    T: 'static
        + Serialize
        + DeserializeOwned
        + SparsableTrait
        + Clone
        + std::fmt::Debug
        + PartialEq,
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
