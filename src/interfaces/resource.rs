use crate::*;

use winapi::shared::guiddef::GUID;

use std::convert::TryInto;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// [\*Texture\*](crate::BaseTexture), [Surface] (but not <strike>[Volume]</strike>!), [IndexBuffer], [VertexBuffer], but not <strike>[\*Shader](crate::PixelShader)</strike>!
///
/// ### See Also
///
/// *   [Device::create_cube_texture]
/// *   [Device::create_texture]
/// *   [Device::create_volume_texture]
/// *   [Device::create_index_buffer]
/// *   [Device::create_vertex_buffer]
#[derive(Clone)] #[repr(transparent)]
pub struct Resource(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DResource9>);

impl Resource {
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
    pub fn free_private_data(&self, guid: &GUID) -> Result<(), MethodError> {
        let hr = unsafe { self.0.FreePrivateData(guid) };
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
    pub fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.0.GetDevice(&mut device) };
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
    pub fn get_priority(&self) -> u32 {
        unsafe { self.0.GetPriority() }
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
    pub fn get_private_data_inplace<'s>(&self, guid: &GUID, data: &'s mut [u8]) -> Result<&'s [u8], MethodError> {
        let mut n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::get_private_data_inplace", D3DERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.0.GetPrivateData(guid, data.as_mut_ptr().cast(), &mut n) };
        MethodError::check("IDirect3DResource9::GetPrivateData", hr)?;
        Ok(&data[..(n as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-gettype)\]
    /// IDirect3DResource9::GetType
    ///
    /// Returns the type of the resource.
    pub fn get_type(&self) -> ResourceType {
        ResourceType::from_unchecked(unsafe { self.0.GetType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-preload)\]
    /// IDirect3DResource9::PreLoad
    ///
    /// Preloads a managed resource.
    pub fn preload(&self) {
        unsafe { self.0.PreLoad() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setpriority)\]
    /// IDirect3DResource9::SetPriority
    ///
    /// ### Returns
    ///
    /// The previous priority
    pub fn set_priority(&self, priority: u32) -> u32 {
        // see also https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9-resource-priority
        unsafe { self.0.SetPriority(priority) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok(())
    pub fn set_private_data(&self, guid: &GUID, data: &[u8]) -> Result<(), MethodError> {
        let n : u32 = data.len().try_into().map_err(|_| MethodError("Resource::set_private_data", D3DERR::SLICE_OVERFLOW))?;
        let hr = unsafe { self.0.SetPrivateData(guid, data.as_ptr().cast(), n, 0) };
        MethodError::check("IDirect3DResource9::SetPrivateData", hr)
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?
}

// TODO: examples
// TODO: integration tests
// TODO: make set_private_data generic on any data type?  bytemuck data types?
// TODO: make get_private_data_inplace generic on bytemuck data types?
