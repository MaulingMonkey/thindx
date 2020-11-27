#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::shared::winerror::*;

use std::convert::TryInto;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9)\]
/// An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
///
/// ### See Also
///
/// *   [Device::create_query]
#[derive(Clone)] #[repr(transparent)]
pub struct Query(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DQuery9>);



/// # Queries
/// Create/Check Occlusion and other [Query]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery)\]
    /// IDirect3DDevice9::CreateQuery
    ///
    /// Creates a status query.
    pub fn create_query(&self, type_: QueryType) -> Result<Query, MethodError> {
        let mut query = null_mut();
        let hr = unsafe { self.0.CreateQuery(type_.into(), &mut query) };
        MethodError::check("IDirect3DDevice9::CreateQuery", hr)?;
        Ok(unsafe { Query::from_raw(query) })
    }
}

// #[test] fn create_query() {} // TODO



impl Query {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdata)\]
    /// IDirect3DQuery9::GetData
    ///
    /// Polls a queried resource to get the query state or a query result.
    /// For more information about queries, see [Queries (Direct3D9)].
    ///
    /// [Queries (Direct3D9)]:          https://docs.microsoft.com/en-us/windows/desktop/direct3d9/queries
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::DEVICELOST]    The device was lost
    /// *   [D3DERR::INVALIDCALL]   If `data.as_mut().len()` > `u32::MAX`
    /// *   Ok(`true`)              The query data is available
    /// *   Ok(`false`)             The query data is not available
    pub fn get_data_inplace(&self, mut data: impl AsMut<[u8]>, get_data_flags: impl Into<GetData>) -> Result<bool, MethodError> {
        let data = data.as_mut();
        let flags = get_data_flags.into().into();
        let len = data.len().try_into().map_err(|_| MethodError("Query::get_data_inplace", D3DERR::INVALIDCALL))?;

        let hr = unsafe { self.0.GetData(data.as_mut_ptr().cast(), len, flags) };
        MethodError::check("IDirect3DQuery9::GetData", hr)?;
        Ok(hr != S_FALSE)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdatasize)\]
    /// IDirect3DQuery9::GetDataSize
    ///
    /// Gets the number of bytes in the query data.
    pub fn get_data_size(&self) -> u32 {
        unsafe { self.0.GetDataSize() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdevice)\]
    /// IDirect3DQuery9::GetDevice
    ///
    /// Gets the device that is being queried.
    pub fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.0.GetDevice(&mut device) };
        MethodError::check("IDirect3DQuery9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-gettype)\]
    /// IDirect3DQuery9::GetType
    ///
    /// Gets the query type.
    pub fn get_type(&self) -> QueryType {
        QueryType::from_unchecked(unsafe { self.0.GetType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-issue)\]
    /// IDirect3DQuery9::Issue
    ///
    /// Issue a query.
    pub fn issue(&self, issue_flags: impl Into<Issue>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.Issue(issue_flags.into().into()) };
        MethodError::check("IDirect3DQuery9::Issue", hr)
    }
}
