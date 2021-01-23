use super::*;
use auto_impl::auto_impl;

pub fn connect_bread_crumbs(bread_crumb: &[String]) -> String {
    let mut res = String::from('/');
    res.push_str(bread_crumb.join("/").as_str());
    res
}

#[auto_impl(&mut, Box)]
pub trait OApiCheck {
    fn oapi_check(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.oapi_check_inner(root, bread_crumb)
    }

    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError>;
}

macro_rules! impl_oapi_check_nothing {
    ($x:ident) => {
        impl OApiCheck for $x {
            fn oapi_check_inner(
                &self,
                _root: &SparseRoot<OApiDocument>,
                _bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                Ok(())
            }
        }
    };
}

impl OApiCheck for Url {
    fn oapi_check_inner(
        &self,
        _root: &SparseRoot<OApiDocument>,
        _bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        Ok(())
    }
}

impl OApiCheck for Version {
    fn oapi_check_inner(
        &self,
        _root: &SparseRoot<OApiDocument>,
        _bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        Ok(())
    }
}

impl<T> OApiCheck for SparseSelector<T>
where
    T: DeserializeOwned + Serialize + SparsableTrait + OApiCheckTrait,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        match self {
            SparseSelector::Ref(x) => x.oapi_check(root, bread_crumb),
            SparseSelector::Obj(x) => x.oapi_check(root, bread_crumb),
            _ => Ok(()),
        }
    }
}

impl<T> OApiCheck for sppparse::SparseRefRaw<T>
where
    T: DeserializeOwned + Serialize + SparsableTrait + OApiCheckTrait,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.val().oapi_check(root, bread_crumb)
    }
}

impl<T> OApiCheck for sppparse::SparseRef<T>
where
    T: DeserializeOwned + Serialize + SparsableTrait + OApiCheckTrait,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.val().oapi_check(root, bread_crumb)
    }
}

impl<T> OApiCheck for sppparse::SparsePointedValue<T>
where
    T: DeserializeOwned + Serialize + SparsableTrait + OApiCheckTrait,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        match self {
            SparsePointedValue::Ref(x) => x.oapi_check(root, bread_crumb),
            SparsePointedValue::RefRaw(x) => x.oapi_check(root, bread_crumb),
            SparsePointedValue::Obj(x) => x.oapi_check(root, bread_crumb),
            _ => Ok(()),
        }
    }
}

impl OApiCheck for serde_json::Value {
    fn oapi_check_inner(
        &self,
        _root: &SparseRoot<OApiDocument>,
        _bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        Ok(())
    }
}

impl<T> OApiCheck for Option<T>
where
    T: Serialize + DeserializeOwned + OApiCheck,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        match self {
            Some(x) => x.oapi_check(root, bread_crumb),
            None => Ok(()),
        }
    }
}

impl<'a> OApiCheck for &'a str {
    fn oapi_check_inner(
        &self,
        _root: &SparseRoot<OApiDocument>,
        _bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        Ok(())
    }
}

impl<'a> OApiCheck for &'a [u8] {
    fn oapi_check_inner(
        &self,
        _root: &SparseRoot<OApiDocument>,
        _bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        Ok(())
    }
}

impl<K, V> OApiCheck for HashMap<K, V>
where
    V: OApiCheck,
{
    fn oapi_check_inner(
        &self,
        root: &SparseRoot<OApiDocument>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        for i in self.values() {
            i.oapi_check(root, bread_crumb)?;
        }
        Ok(())
    }
}

macro_rules! impl_oapi_check_iter {
    ($x:ident) => {
        impl<T> OApiCheck for $x<T>
        where
            T: OApiCheck,
        {
            fn oapi_check_inner(
                &self,
                root: &SparseRoot<OApiDocument>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                for i in self.iter() {
                    i.oapi_check(root, bread_crumb)?;
                }
                Ok(())
            }
        }
    };
}

impl_oapi_check_nothing!(bool);
impl_oapi_check_nothing!(i8);
impl_oapi_check_nothing!(i16);
impl_oapi_check_nothing!(i32);
impl_oapi_check_nothing!(i64);
impl_oapi_check_nothing!(isize);
impl_oapi_check_nothing!(u8);
impl_oapi_check_nothing!(u16);
impl_oapi_check_nothing!(u32);
impl_oapi_check_nothing!(u64);
impl_oapi_check_nothing!(i128);
impl_oapi_check_nothing!(usize);
impl_oapi_check_nothing!(f32);
impl_oapi_check_nothing!(f64);
impl_oapi_check_nothing!(char);
impl_oapi_check_nothing!(String);
impl_oapi_check_iter!(Vec);
