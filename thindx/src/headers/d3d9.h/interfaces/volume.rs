use crate::*;
use crate::d3d9::*;

use winapi::Interface;
use winapi::shared::d3d9::IDirect3DVolume9;
use winapi::shared::d3d9types::*;
use winapi::shared::guiddef::GUID;
use winapi::um::unknwnbase::IUnknown;

use std::convert::TryInto;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// A dense 3-dimensional region of data, often belonging to a [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct Volume(pub(crate) mcom::Rc<IDirect3DVolume9>);

unsafe impl AsSafe<IUnknown         > for Volume { fn as_safe(&self) -> &IUnknown         { &*self.0 } }
unsafe impl AsSafe<IDirect3DVolume9 > for Volume { fn as_safe(&self) -> &IDirect3DVolume9 { &*self.0 } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// IDirect3DVolume9 extension methods
///
/// ### Methods
///
/// | thindx                                                        | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [free_private_data](Self::free_private_data)                  | [FreePrivateData]     | Frees the specified private data associated with this volume.
/// | [get_container](Self::get_container)                          | [GetContainer]        | Provides access to the parent volume texture object, if this surface is a child level of a volume texture.
/// | [get_desc](Self::get_desc)                                    | [GetDesc]             | Retrieves a description of the volume.
/// | [get_device](Self::get_device)                                | [GetDevice]           | Retrieves the device associated with a volume.
/// | [get_private_data_inplace](Self::get_private_data_inplace)    | [GetPrivateData]      | Copies the private data associated with the volume to a provided buffer.
/// | <span style="opacity: 25%">get_private_data_com</span>        | [GetPrivateData]      | Retrieves a COM object associated with the volume.
/// | [lock_box_unchecked](Self::lock_box_unchecked)                | [LockBox]             | Locks a box on a volume resource.
/// | [set_private_data](Self::set_private_data)                    | [SetPrivateData]      | Associates data with the volume that is intended for use by the application, not by Direct3D.
/// | <span style="opacity: 25%">set_debug_name</span>              | [SetPrivateData]      | Associates a debug name with the volume for graphics debuggers.
/// | <span style="opacity: 25%">set_private_data_com</span>        | [SetPrivateData]      | Associates a COM object with the resource for use by the application.
/// | [unlock_box](Self::unlock_box)                                | [UnlockBox]           | Unlocks a box on a volume resource.
///
/// [FreePrivateData]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-freeprivatedata
/// [GetContainer]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getcontainer
/// [GetDesc]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdesc
/// [GetDevice]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdevice
/// [GetPrivateData]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getprivatedata
/// [LockBox]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-lockbox
/// [SetPrivateData]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata
/// [UnlockBox]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-unlockbox
///
pub trait IDirect3DVolume9Ext : AsSafe<IDirect3DVolume9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-freeprivatedata)\]
    /// IDirect3DVolume9::FreePrivateData
    ///
    /// Frees the specified private data associated with this resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::NOTFOUND]
    /// *   Ok(`()`)
    fn free_private_data(&self, guid: &GUID) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().FreePrivateData(guid) };
        MethodError::check("IDirect3DVolume9::FreePrivateData", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getcontainer)\]
    /// IDirect3DVolume9::GetContainer
    ///
    /// Provides access to the parent volume texture object, if this surface is a child level of a volume texture.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`C`)
    fn get_container<C: Raw>(&self) -> Result<C, MethodError> where C::Raw : Interface {
        let mut container = null_mut();
        let hr = unsafe { self.as_winapi().GetContainer(&C::Raw::uuidof(), &mut container) };
        MethodError::check("IDirect3DVolume9::GetContainer", hr)?;
        Ok(unsafe { C::from_raw(container.cast()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdesc)\]
    /// IDirect3DVolume9::GetDesc
    ///
    /// Retrieves a description of the volume.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VolumeDesc])
    fn get_desc(&self) -> Result<VolumeDesc, MethodError> {
        let mut desc = VolumeDesc::default();
        let hr = unsafe { self.as_winapi().GetDesc(&mut *desc) };
        MethodError::check("IDirect3DVolume9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdevice)\]
    /// IDirect3DVolume9::GetDevice
    ///
    /// Retrieves the device associated with a resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Device])
    fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.as_winapi().GetDevice(&mut device) };
        MethodError::check("IDirect3DVolume9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getprivatedata)\]
    /// IDirect3DVolume9::GetPrivateData
    ///
    /// Copies the private data associated with the resource to a provided buffer.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::MOREDATA]
    /// *   [D3DERR::NOTFOUND]
    /// *   Ok(`read_slice`)
    fn get_private_data_inplace<'s>(&self, guid: &GUID, data: &'s mut [u8]) -> Result<&'s [u8], MethodError> {
        let mut n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::get_private_data_inplace", THINERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.as_winapi().GetPrivateData(guid, data.as_mut_ptr().cast(), &mut n) };
        MethodError::check("IDirect3DVolume9::GetPrivateData", hr)?;
        Ok(&data[..(n as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok(`()`)
    fn set_private_data(&self, guid: &GUID, data: &[u8]) -> Result<(), MethodError> {
        let n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::set_private_data", THINERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.as_winapi().SetPrivateData(guid, data.as_ptr().cast(), n, 0) };
        MethodError::check("IDirect3DVolume9::SetPrivateData", hr)
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-lockbox)\]
    /// IDirect3DVolume9::LockBox
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   `box_` must be `..` or a valid subregion of the volume in question
    /// *   `self` should be lockable in the style specified by `flags`... and not already locked?
    /// *   `self` may need to be unlocked again before being bound, drawn, or released
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([D3DLOCKED_BOX])
    unsafe fn lock_box_unchecked(&self, box_: impl IntoBoxOrFull, flags: impl Into<Lock>) -> Result<D3DLOCKED_BOX, MethodError> {
        let mut locked = unsafe { std::mem::zeroed::<D3DLOCKED_BOX>() };
        let box_ = box_.into_box();
        let box_ = box_.as_ref().map_or(null(), |b| &**b);
        let hr = unsafe { self.as_winapi().LockBox(&mut locked, box_, flags.into().into()) };
        MethodError::check("IDirect3DVolume9::LockBox", hr)?;
        Ok(locked)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-unlockbox)\]
    /// IDirect3DVolume9::UnlockBox
    ///
    /// Unlocks a box on a volume resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn unlock_box(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().UnlockBox() };
        MethodError::check("IDirect3DVolume9::UnlockBox", hr)
    }
}

impl<T: AsSafe<IDirect3DVolume9>> IDirect3DVolume9Ext for T {}



// TODO: examples
// TODO: integration tests
// TODO: make set_private_data generic on any data type?  bytemuck data types?
// TODO: make get_private_data_inplace generic on bytemuck data types?



//#cpp2rust IDirect3DVolume9                        = d3d9::Volume
//#cpp2rust IDirect3DVolume9                        = d3d9::IDirect3DVolume9Ext

//#cpp2rust IDirect3DVolume9::FreePrivateData       = d3d9::IDirect3DVolume9Ext::free_private_data
//#cpp2rust IDirect3DVolume9::GetContainer          = d3d9::IDirect3DVolume9Ext::get_container
//#cpp2rust IDirect3DVolume9::GetDesc               = d3d9::IDirect3DVolume9Ext::get_desc
//#cpp2rust IDirect3DVolume9::GetDevice             = d3d9::IDirect3DVolume9Ext::get_device
//#cpp2rust IDirect3DVolume9::GetPrivateData        = d3d9::IDirect3DVolume9Ext::get_private_data_inplace
//TODO:     IDirect3DVolume9::GetPrivateData        = d3d9::IDirect3DVolume9Ext::get_private_data_com
//#cpp2rust IDirect3DVolume9::LockBox               = d3d9::IDirect3DVolume9Ext::lock_box_unchecked
//#cpp2rust IDirect3DVolume9::SetPrivateData        = d3d9::IDirect3DVolume9Ext::set_private_data
//TODO:     IDirect3DVolume9::SetPrivateData        = d3d9::IDirect3DVolume9Ext::set_debug_name
//TODO:     IDirect3DVolume9::SetPrivateData        = d3d9::IDirect3DVolume9Ext::set_private_data_com
//#cpp2rust IDirect3DVolume9::UnlockBox             = d3d9::IDirect3DVolume9Ext::unlock_box
