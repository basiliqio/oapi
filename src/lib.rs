#![warn(clippy::all)]

use getset::Getters;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sppparse::{Sparsable, SparsableTrait, SparseError, SparsePointedValue, SparseSelector};
use std::collections::HashMap;
use url::Url;

mod json_schemas;
mod operators;
mod schemas;

pub use operators::{
    AllOfSelect, AnyOfSelect, NotSelect, OApiOperator, OneOfSelect, OperatorSelector,
};

pub use json_schemas::{
    OApiNumericMaximum, OApiNumericMinimum, OApiSchema, OApiSchemaAdditionalItem, OApiSchemaArray,
    OApiSchemaNumeric, OApiSchemaObject, OApiSchemaString,
};
