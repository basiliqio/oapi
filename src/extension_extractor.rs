use super::*;

pub trait OApiExtensionExtractor {
    fn oapi_raw_ext(&self) -> &HashMap<String, Value>;
    fn oapi_extract_ext<S>(
        &self,
        root: &SparseRoot<OApiDocument>,
        key: &str,
    ) -> Result<S, OApiError>
    where
        S: DeserializeOwned + SparsableTrait + Serialize + OApiCheckTrait,
    {
        let raw: &HashMap<String, Value> = self.oapi_raw_ext();

        let mut res: S = serde_json::from_value(
            raw.get(key)
                .cloned()
                .ok_or_else(|| OApiError::NoExtKey(key.to_string()))?,
        )
        .map_err(|err| OApiError::SppparseError(SparseError::SerdeJson(err)))?;
        let mut state = root
            .state()
            .try_borrow_mut()
            .map_err(|_err| OApiError::SppparseError(SparseError::StateAlreadyBorrowed))?;
        let metadata = root.metadata();
        res.sparse_init(&mut state, metadata, 0)?;
        Ok(res)
    }
}
