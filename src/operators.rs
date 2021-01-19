use super::*;
use sppparse::{SparsePointer, SparseValue};

pub trait OApiOperator<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    fn get<'a>(&'a self) -> Result<Vec<SparseValue<'a, T>>, SparseError>;
}

macro_rules! OApiOperatorImpl {
    ($struct_name:ident) => {
        impl<T> OApiOperator<T> for $struct_name<T>
        where
            T: 'static + Serialize + DeserializeOwned + SparsableTrait,
        {
            fn get<'a>(&'a self) -> Result<Vec<SparseValue<'a, T>>, SparseError> {
                let mut res: Vec<SparseValue<T>> = Vec::new();

                for v in self.root().iter() {
                    res.append(&mut v.get()?);
                }
                Ok(res)
            }
        }
    };
}

#[derive(Serialize, Deserialize, Getters, Sparsable)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
pub struct AnyOfSelect<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    #[getset(get)]
    #[serde(rename = "anyOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
pub struct OneOfSelect<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    #[getset(get)]
    #[serde(rename = "oneOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
pub struct AllOfSelect<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    #[getset(get)]
    #[serde(rename = "allOf")]
    root: Vec<OperatorSelector<T>>,
}

#[derive(Serialize, Deserialize, Getters, Sparsable)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
pub struct NotSelect<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    #[getset(get)]
    #[serde(rename = "not")]
    root: Vec<OperatorSelector<T>>,
}

OApiOperatorImpl!(AnyOfSelect);
OApiOperatorImpl!(OneOfSelect);
OApiOperatorImpl!(AllOfSelect);
OApiOperatorImpl!(NotSelect);

#[derive(Serialize, Deserialize, Sparsable)]
#[serde(bound = "T: 'static + DeserializeOwned + Serialize + SparsableTrait")]
#[serde(untagged)]
pub enum OperatorSelector<T: 'static + Serialize + DeserializeOwned + SparsableTrait> {
    AnyOf(AnyOfSelect<T>),
    OneOf(OneOfSelect<T>),
    AllOf(AllOfSelect<T>),
    Not(NotSelect<T>),
    Val(SparseSelector<T>),
}

impl<T> OApiOperator<T> for OperatorSelector<T>
where
    T: 'static + Serialize + DeserializeOwned + SparsableTrait,
{
    fn get<'a>(&'a self) -> Result<Vec<SparseValue<'a, T>>, SparseError> {
        match self {
            OperatorSelector::AnyOf(x) => x.get(),
            OperatorSelector::OneOf(x) => x.get(),
            OperatorSelector::AllOf(x) => x.get(),
            OperatorSelector::Not(x) => x.get(),
            OperatorSelector::Val(x) => Ok(vec![x.get()?]),
        }
    }
}
