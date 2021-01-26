use super::*;
use auto_impl::auto_impl;

pub fn connect_bread_crumbs(bread_crumb: &[String]) -> String {
    let mut res = String::from('/');
    res.push_str(bread_crumb.join("/").as_str());
    res
}

#[auto_impl(&mut, Box)]
pub trait OApiCheckTrait {
    type Doc;

    fn oapi_check(
        &self,
        root: &SparseRoot<Self::Doc>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.oapi_check_inner(root, bread_crumb)
    }

    fn oapi_check_inner(
        &self,
        root: &SparseRoot<Self::Doc>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError>;
}

macro_rules! impl_oapi_check_nothing {
    ($t:tt, $x:ident) => {
        impl OApiCheckTrait for $x {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                _root: &SparseRoot<Self::Doc>,
                _bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                Ok(())
            }
        }
    };
}

macro_rules! impl_oapi_check_iter {
    ($t:tt, $x:ident) => {
        impl<T> OApiCheckTrait for $x<T>
        where
            T: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
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

macro_rules! impl_oapi_check_sparse {
    ($t:tt) => {
        impl<T> OApiCheckTrait for SparseSelector<T>
        where
            T: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                match self {
                    SparseSelector::Ref(x) => x.oapi_check(root, bread_crumb),
                    SparseSelector::Obj(x) => x.oapi_check(root, bread_crumb),
                    _ => Ok(()),
                }
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparseRefRaw<T>
        where
            T: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                self.val().oapi_check(root, bread_crumb)
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparseRef<T>
        where
            T: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                self.val().oapi_check(root, bread_crumb)
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparsePointedValue<T>
        where
            T: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;

            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
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
    };
}

macro_rules! impl_oapi_check_special_types {
    ($t:tt) => {
        impl<'a> OApiCheckTrait for &'a [u8] {
            type Doc = $t;
            fn oapi_check_inner(
                &self,
                _root: &SparseRoot<Self::Doc>,
                _bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                Ok(())
            }
        }
        impl<K, V> OApiCheckTrait for HashMap<K, V>
        where
            V: SparsableTrait + OApiCheckTrait<Doc = $t>,
        {
            type Doc = $t;
            fn oapi_check_inner(
                &self,
                root: &SparseRoot<Self::Doc>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                for i in self.values() {
                    i.oapi_check(root, bread_crumb)?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_oapi_check {
    ($t:tt) => {
        impl_oapi_check_sparse!($t);
        impl_oapi_check_special_types!($t);
        impl_oapi_check_nothing!($t, bool);
        impl_oapi_check_nothing!($t, i8);
        impl_oapi_check_nothing!($t, i16);
        impl_oapi_check_nothing!($t, i32);
        impl_oapi_check_nothing!($t, i64);
        impl_oapi_check_nothing!($t, isize);
        impl_oapi_check_nothing!($t, u8);
        impl_oapi_check_nothing!($t, u16);
        impl_oapi_check_nothing!($t, u32);
        impl_oapi_check_nothing!($t, u64);
        impl_oapi_check_nothing!($t, i128);
        impl_oapi_check_nothing!($t, usize);
        impl_oapi_check_nothing!($t, f32);
        impl_oapi_check_nothing!($t, f64);
        impl_oapi_check_nothing!($t, char);
        impl_oapi_check_nothing!($t, String);
        impl_oapi_check_nothing!($t, Url);
        impl_oapi_check_nothing!($t, Version);
        impl_oapi_check_nothing!($t, Value);
        impl_oapi_check_iter!($t, Vec);
    };
}

impl_oapi_check!(OApiDocumentCore);
