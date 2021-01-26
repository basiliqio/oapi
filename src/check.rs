use super::*;
use auto_impl::auto_impl;

pub fn connect_bread_crumbs(bread_crumb: &[String]) -> String {
    let mut res = String::from('/');
    res.push_str(bread_crumb.join("/").as_str());
    res
}

#[auto_impl(&, &mut, Box)]
pub trait OApiCheckTrait {
    fn oapi_check(
        &self,
        state: &Rc<RefCell<SparseState>>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError> {
        self.oapi_check_inner(state, bread_crumb)
    }

    fn oapi_check_inner(
        &self,
        state: &Rc<RefCell<SparseState>>,
        bread_crumb: &mut Vec<String>,
    ) -> Result<(), OApiError>;
}

macro_rules! impl_oapi_check_nothing {
    ($x:ident) => {
        impl OApiCheckTrait for $x {
            fn oapi_check_inner(
                &self,
                _state: &Rc<RefCell<SparseState>>,
                _bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                Ok(())
            }
        }
    };
}

macro_rules! impl_oapi_check_iter {
    ($x:ident) => {
        impl<T> OApiCheckTrait for $x<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                for i in self.iter() {
                    i.oapi_check(state, bread_crumb)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_oapi_check_sparse {
    () => {
        impl<T> OApiCheckTrait for SparseSelector<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                match self {
                    SparseSelector::Ref(x) => x.oapi_check(state, bread_crumb),
                    SparseSelector::Obj(x) => x.oapi_check(state, bread_crumb),
                    _ => Ok(()),
                }
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparseRefRaw<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                self.val().oapi_check(state, bread_crumb)
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparseRef<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                self.val().oapi_check(state, bread_crumb)
            }
        }

        impl<T> OApiCheckTrait for sppparse::SparsePointedValue<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                match self {
                    SparsePointedValue::Ref(x) => x.oapi_check(state, bread_crumb),
                    SparsePointedValue::RefRaw(x) => x.oapi_check(state, bread_crumb),
                    SparsePointedValue::Obj(x) => x.oapi_check(state, bread_crumb),
                    _ => Ok(()),
                }
            }
        }
    };
}

macro_rules! impl_oapi_check_special_types {
    () => {
        impl<'a> OApiCheckTrait for &'a [u8] {
            fn oapi_check_inner(
                &self,
                _state: &Rc<RefCell<SparseState>>,
                _bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                Ok(())
            }
        }

        impl<T> OApiCheckTrait for Option<T>
        where
            T: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                match self {
                    Some(x) => x.oapi_check(state, bread_crumb),
                    None => Ok(()),
                }
            }
        }

        impl<K, V> OApiCheckTrait for HashMap<K, V>
        where
            V: OApiCheckTrait,
        {
            fn oapi_check_inner(
                &self,
                state: &Rc<RefCell<SparseState>>,
                bread_crumb: &mut Vec<String>,
            ) -> Result<(), OApiError> {
                for i in self.values() {
                    i.oapi_check(state, bread_crumb)?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_oapi_check {
    ($t:tt) => {
        impl_oapi_check_sparse!();
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
        impl_oapi_check_nothing!(Url);
        impl_oapi_check_nothing!(Version);
        impl_oapi_check_nothing!(Value);
        impl_oapi_check_special_types!();
        impl_oapi_check_iter!(Vec);
    };
}

impl_oapi_check!(OApiDocumentCore);
