use crate::*;

use winapi::Interface;
use winapi::shared::d3d9types::*;
use winapi::shared::guiddef::GUID;

use std::convert::TryInto;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// A dense 3-dimensional region of data, often belonging to a [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct Volume(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVolume9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// IDirect3DVolume9 extension methods
///
/// ### Methods
///
/// | thin3d9                                                       | docs.microsoft.com    | Description |
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
pub trait IDirect3DVolume9Ext : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-freeprivatedata)\]
    /// IDirect3DResource9::FreePrivateData
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
        MethodError::check("IDirect3DResource9::FreePrivateData", hr)
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
    /// IDirect3DResource9::GetDevice
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
        MethodError::check("IDirect3DResource9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getprivatedata)\]
    /// IDirect3DResource9::GetPrivateData
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
        let mut n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::get_private_data_inplace", D3DERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.as_winapi().GetPrivateData(guid, data.as_mut_ptr().cast(), &mut n) };
        MethodError::check("IDirect3DResource9::GetPrivateData", hr)?;
        Ok(&data[..(n as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok(`()`)
    fn set_private_data(&self, guid: &GUID, data: &[u8]) -> Result<(), MethodError> {
        let n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::set_private_data", D3DERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.as_winapi().SetPrivateData(guid, data.as_ptr().cast(), n, 0) };
        MethodError::check("IDirect3DResource9::SetPrivateData", hr)
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-lockbox)\]
    /// IDirect3DVolume9::LockBox
    ///
    /// ### Safety
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
        let mut locked = std::mem::zeroed::<D3DLOCKED_BOX>();
        let box_ = box_.into_box();
        let box_ = box_.as_ref().map_or(null(), |b| &**b);
        let hr = self.as_winapi().LockBox(&mut locked, box_, flags.into().into());
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

impl<T: private::Sealed> IDirect3DVolume9Ext for T {}

mod private {
    use winapi::shared::d3d9::IDirect3DVolume9;
    pub unsafe trait Sealed                             { fn as_winapi(&self) -> &IDirect3DVolume9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DVolume9>   { fn as_winapi(&self) -> &IDirect3DVolume9 { &**self } }
    unsafe impl Sealed for super::Volume                { fn as_winapi(&self) -> &IDirect3DVolume9 { &*self.0 } }
}

// TODO: examples
// TODO: integration tests
// TODO: make set_private_data generic on any data type?  bytemuck data types?
// TODO: make get_private_data_inplace generic on bytemuck data types?
