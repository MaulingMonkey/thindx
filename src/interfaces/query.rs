#![allow(dead_code)] // TODO: remove

use crate::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9)\]
/// An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
#[derive(Clone)] #[repr(transparent)]
pub struct Query(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DQuery9>);



/// [Query] management
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery)\]
    /// IDirect3DDevice9::CreateQuery
    ///
    /// Creates a status query.
    pub(crate) fn create_query(&self, type_: QueryType) -> Result<Query, MethodError> {
        let mut query = null_mut();
        let hr = unsafe { self.0.CreateQuery(type_.into(), &mut query) };
        MethodError::check("IDirect3DDevice9::CreateQuery", hr)?;
        Ok(unsafe { Query::from_raw(query) })
    }
}

// #[test] fn create_query() {} // TODO
