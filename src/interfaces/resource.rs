use crate::*;

use winapi::shared::guiddef::GUID;

use std::convert::TryInto;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// [\*Texture\*](crate::BaseTexture), [Surface] (but not <strike>[Volume]</strike>!), [IndexBuffer], [VertexBuffer], but not <strike>[\*Shader](crate::PixelShader)</strike>!
///
/// ### See Also
///
/// *   [IDirect3DDevice9Ext::create_cube_texture]
/// *   [IDirect3DDevice9Ext::create_texture]
/// *   [IDirect3DDevice9Ext::create_volume_texture]
/// *   [IDirect3DDevice9Ext::create_index_buffer]
/// *   [IDirect3DDevice9Ext::create_vertex_buffer]
#[derive(Clone)] #[repr(transparent)]
pub struct Resource(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DResource9>);

impl Resource {
    /// Check if `self` is compatible with `device`, returning an `Err(...)` if it isn't.
    pub fn check_compatible_with(&self, device: &impl IDirect3DDevice9Ext, method: &'static str) -> Result<(), MethodError> {
        let my_device = self.get_device()?;
        if my_device.as_raw() == device.as_winapi() as *const _ as *mut _ {
            Ok(())
        } else {
            Err(MethodError(method, D3DERR::DEVICE_MISMATCH))
        }
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// IDirect3DResource9 extension methods
///
/// ### Methods
///
/// | thin3d9                                                       | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [free_private_data](Self::free_private_data)                  | [FreePrivateData]     | Frees the specified private data associated with this resource.
/// | [get_device](Self::get_device)                                | [GetDevice]           | Retrieves the device associated with a resource.
/// | [get_priority](Self::get_priority)                            | [GetPriority]         | Retrieves the priority for this resource.
/// | [get_private_data_inplace](Self::get_private_data_inplace)    | [GetPrivateData]      | Copies the private data associated with the resource to a provided buffer.
/// | <span style="opacity: 25%">get_private_data_com</span>        | [GetPrivateData]      | Retrieves a COM object associated with the resource.
/// | [get_type](Self::get_type)                                    | [GetType]             | Returns the type of the resource.
/// | [preload](Self::preload)                                      | [PreLoad]             | Preloads a managed resource.
/// | [set_priority](Self::set_priority)                            | [SetPriority]         | Assigns the priority of a resource for scheduling purposes.
/// | [set_private_data](Self::set_private_data)                    | [SetPrivateData]      | Associates data with the resource for use by the application.
/// | <span style="opacity: 25%">set_debug_name</span>              | [SetPrivateData]      | Associates a debug name for graphics debuggers.
/// | <span style="opacity: 25%">set_private_data_com</span>        | [SetPrivateData]      | Associates a COM object with the resource for use by the application.
///
/// [FreePrivateData]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-freeprivatedata
/// [GetDevice]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getdevice
/// [GetPriority]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getpriority
/// [GetPrivateData]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getprivatedata
/// [GetType]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-gettype
/// [PreLoad]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-preload
/// [SetPriority]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setpriority
/// [SetPrivateData]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata
///
pub trait IDirect3DResource9Ext : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-freeprivatedata)\]
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getdevice)\]
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getpriority)\]
    /// IDirect3DResource9::GetPriority
    ///
    /// Retrieves the priority for this resource.
    ///
    /// [Resource::get_priority] is used for priority control of managed resources. This method returns `0` on nonmanaged resources.
    ///
    /// Priorities are used to determine when managed resources are to be removed from memory.
    /// A resource assigned a low priority is removed before a resource with a high priority.
    /// If two resources have the same priority, the resource that was used more recently is kept in memory; the other resource is removed.
    /// Managed resources have a default priority of `0`.
    fn get_priority(&self) -> u32 {
        unsafe { self.as_winapi().GetPriority() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getprivatedata)\]
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-gettype)\]
    /// IDirect3DResource9::GetType
    ///
    /// Returns the type of the resource.
    fn get_type(&self) -> ResourceType {
        ResourceType::from_unchecked(unsafe { self.as_winapi().GetType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-preload)\]
    /// IDirect3DResource9::PreLoad
    ///
    /// Preloads a managed resource.
    fn preload(&self) {
        unsafe { self.as_winapi().PreLoad() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setpriority)\]
    /// IDirect3DResource9::SetPriority
    ///
    /// ### Returns
    ///
    /// The previous priority
    fn set_priority(&self, priority: u32) -> u32 {
        // see also https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9-resource-priority
        unsafe { self.as_winapi().SetPriority(priority) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok(())
    fn set_private_data(&self, guid: &GUID, data: &[u8]) -> Result<(), MethodError> {
        let n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::set_private_data", D3DERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.as_winapi().SetPrivateData(guid, data.as_ptr().cast(), n, 0) };
        MethodError::check("IDirect3DResource9::SetPrivateData", hr)
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?
}

impl<T: private::Sealed> IDirect3DResource9Ext for T {}

mod private {
    use winapi::shared::d3d9::*;
    pub unsafe trait Sealed                                     { fn as_winapi(&self) -> &IDirect3DResource9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DResource9>         { fn as_winapi(&self) -> &IDirect3DResource9 { &**self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DBaseTexture9>      { fn as_winapi(&self) -> &IDirect3DResource9 { &***self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DCubeTexture9>      { fn as_winapi(&self) -> &IDirect3DResource9 { &****self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DVolumeTexture9>    { fn as_winapi(&self) -> &IDirect3DResource9 { &****self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DTexture9>          { fn as_winapi(&self) -> &IDirect3DResource9 { &****self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DSurface9>          { fn as_winapi(&self) -> &IDirect3DResource9 { &***self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DIndexBuffer9>      { fn as_winapi(&self) -> &IDirect3DResource9 { &***self } }
    unsafe impl Sealed for mcom::Rc<IDirect3DVertexBuffer9>     { fn as_winapi(&self) -> &IDirect3DResource9 { &***self } }
    unsafe impl Sealed for super::Resource                      { fn as_winapi(&self) -> &IDirect3DResource9 { &*self.0 } }
    unsafe impl Sealed for super::BaseTexture                   { fn as_winapi(&self) -> &IDirect3DResource9 { &**self.0 } }
    unsafe impl Sealed for super::CubeTexture                   { fn as_winapi(&self) -> &IDirect3DResource9 { &***self.0 } }
    unsafe impl Sealed for super::VolumeTexture                 { fn as_winapi(&self) -> &IDirect3DResource9 { &***self.0 } }
    unsafe impl Sealed for super::Texture                       { fn as_winapi(&self) -> &IDirect3DResource9 { &***self.0 } }
    unsafe impl Sealed for super::Surface                       { fn as_winapi(&self) -> &IDirect3DResource9 { &**self.0 } }
    unsafe impl Sealed for super::IndexBuffer                   { fn as_winapi(&self) -> &IDirect3DResource9 { &**self.0 } }
    unsafe impl Sealed for super::VertexBuffer                  { fn as_winapi(&self) -> &IDirect3DResource9 { &**self.0 } }
}

// TODO: examples
// TODO: integration tests
// TODO: make set_private_data generic on any data type?  bytemuck data types?
// TODO: make get_private_data_inplace generic on bytemuck data types?
