#![allow(dead_code)] // TODO: remove

use crate::*;
use crate::d3d9::*;

use winapi::shared::d3d9::IDirect3DQuery9;
use winapi::shared::winerror::*;
use winapi::um::unknwnbase::IUnknown;

use std::convert::TryInto;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9)\]
/// An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
///
/// ### See Also
///
/// *   [IDirect3DDevice9Ext::create_query]
#[derive(Clone)] #[repr(transparent)]
pub struct Query(pub(crate) mcom::Rc<IDirect3DQuery9>);

unsafe impl AsSafe<IUnknown         > for Query { fn as_safe(&self) -> &IUnknown        { &**self.0 } }
unsafe impl AsSafe<IDirect3DQuery9  > for Query { fn as_safe(&self) -> &IDirect3DQuery9 { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9)\]
/// IDirect3DQuery9 extension methods
///
/// ### Methods
///
/// | thindx                                    | docs.microsoft.com    | Description |
/// | ----------------------------------------- | --------------------- | ----------- |
/// | [get_data](Self::get_data_inplace)        | [GetData]             | Polls a queried resource to get the query state or a query result. For more information about queries, see [Queries (Direct3D 9)].
/// | [get_data_size](Self::get_data_size)      | [GetDataSize]         | Gets the number of bytes in the query data.
/// | [get_device](Self::get_device)            | [GetDevice]           | Gets the device that is being queried.
/// | [get_type](Self::get_type)                | [GetType]             | Gets the query type.
/// | [issue](Self::issue)                      | [Issue]               | Issue a query.
///
/// [GetData]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdata
/// [GetDataSize]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdatasize
/// [GetDevice]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdevice
/// [GetType]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-gettype
/// [Issue]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-issue
///
/// [Queries (Direct3D 9)]: https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries
///
pub trait IDirect3DQuery9Ext : AsSafe<IDirect3DQuery9> {
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
    fn get_data_inplace(&self, mut data: impl AsMut<[u8]>, get_data_flags: impl Into<GetData>) -> Result<bool, MethodError> {
        let data = data.as_mut();
        let flags = get_data_flags.into().into();
        let len = data.len().try_into().map_err(|_| MethodError("Query::get_data_inplace", D3DERR::INVALIDCALL))?;

        let hr = unsafe { self.as_winapi().GetData(data.as_mut_ptr().cast(), len, flags) };
        MethodError::check("IDirect3DQuery9::GetData", hr)?;
        Ok(hr != S_FALSE)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdatasize)\]
    /// IDirect3DQuery9::GetDataSize
    ///
    /// Gets the number of bytes in the query data.
    fn get_data_size(&self) -> u32 {
        unsafe { self.as_winapi().GetDataSize() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-getdevice)\]
    /// IDirect3DQuery9::GetDevice
    ///
    /// Gets the device that is being queried.
    fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.as_winapi().GetDevice(&mut device) };
        MethodError::check("IDirect3DQuery9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-gettype)\]
    /// IDirect3DQuery9::GetType
    ///
    /// Gets the query type.
    fn get_type(&self) -> QueryType {
        QueryType::from_unchecked(unsafe { self.as_winapi().GetType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dquery9-issue)\]
    /// IDirect3DQuery9::Issue
    ///
    /// Issue a query.
    fn issue(&self, issue_flags: impl Into<Issue>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Issue(issue_flags.into().into()) };
        MethodError::check("IDirect3DQuery9::Issue", hr)
    }
}

impl<T: AsSafe<IDirect3DQuery9>> IDirect3DQuery9Ext for T {}
