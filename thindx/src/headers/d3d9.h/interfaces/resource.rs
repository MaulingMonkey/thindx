use crate::*;
use crate::d3d9::*;

use winapi::shared::d3d9::IDirect3DResource9;
use winapi::um::unknwnbase::IUnknown;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// [\*Texture\*](crate::BaseTexture), [Surface] (but not <strike>[Volume]</strike>!), [IndexBuffer], [VertexBuffer], but not <strike>[\*Shader](crate::PixelShader)</strike>!
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::create_cube_texture]
/// *   [IDirect3DDevice9Ext::create_texture]
/// *   [IDirect3DDevice9Ext::create_volume_texture]
/// *   [IDirect3DDevice9Ext::create_index_buffer]
/// *   [IDirect3DDevice9Ext::create_vertex_buffer]
#[derive(Clone)] #[repr(transparent)]
pub struct Resource(pub(crate) mcom::Rc<IDirect3DResource9>);

unsafe impl AsSafe<IUnknown             > for Resource { fn as_safe(&self) -> &IUnknown             { &**self.0 } }
unsafe impl AsSafe<IDirect3DResource9   > for Resource { fn as_safe(&self) -> &IDirect3DResource9   { &*self.0 } }

impl Resource {
    /// Check if `self` is compatible with `device`, returning an `Err(...)` if it isn't.
    pub(crate) fn check_compatible_with(&self, device: &impl IDirect3DDevice9Ext) -> Result<(), ErrorKind> {
        let my_device = self.get_device().map_err(|e| e.kind())?;
        if my_device.as_raw() == device.as_winapi() as *const _ as *mut _ {
            Ok(())
        } else {
            Err(THINERR::DEVICE_MISMATCH.into())
        }
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// IDirect3DResource9 extension methods
///
/// ### Methods
/// | thindx                                                        | docs.microsoft.com    | Description |
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
/// | [set_object_name](Self::set_object_name)                      | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
/// | [set_object_name_a](Self::set_object_name_a)                  | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
/// | [set_object_name_w](Self::set_object_name_w)                  | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
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
pub trait IDirect3DResource9Ext : AsSafe<IDirect3DResource9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-freeprivatedata)\]
    /// IDirect3DResource9::FreePrivateData
    ///
    /// Frees the specified private data associated with this resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::NOTFOUND]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    ///
    /// // wkpdid::D3DDebugObjectName not yet set:
    /// assert_eq!(D3DERR::NOTFOUND, r.free_private_data(&wkpdid::D3DDebugObjectName));
    ///
    /// r.set_object_name("triangle index buffer").unwrap();
    ///
    /// // wkpdid::D3DDebugObjectName was set by set_object_name, so now it should succeed:
    /// r.free_private_data(&wkpdid::D3DDebugObjectName).unwrap();
    /// ```
    fn free_private_data(&self, guid: &impl AsRef<Guid>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::free_private_data => IDirect3DResource9::FreePrivateData);
        fn_check_hr!(unsafe { self.as_winapi().FreePrivateData(guid.as_ref().as_ref()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getdevice)\]
    /// IDirect3DResource9::GetDevice
    ///
    /// Retrieves the device associated with a resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Device])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// let dev2 = r.get_device().unwrap();
    ///
    /// let dev  = mcom::Rc::<winapi::shared::d3d9::IDirect3DDevice9>::from(dev );
    /// let dev2 = mcom::Rc::<winapi::shared::d3d9::IDirect3DDevice9>::from(dev2);
    /// assert_eq!(dev.as_ptr(), dev2.as_ptr());
    /// ```
    fn get_device(&self) -> Result<Device, Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::get_device => IDirect3DResource9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getpriority)\]
    /// IDirect3DResource9::GetPriority
    ///
    /// Retrieves the priority for this resource.
    ///
    /// [get_priority](Self::get_priority) is used for priority control of managed resources. This method returns `0` on nonmanaged resources.
    ///
    /// Priorities are used to determine when managed resources are to be removed from memory.
    /// A resource assigned a low priority is removed before a resource with a high priority.
    /// If two resources have the same priority, the resource that was used more recently is kept in memory; the other resource is removed.
    /// Managed resources have a default priority of `0`.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// assert_eq!(0, r.get_priority());
    /// r.set_priority(42);
    /// assert_eq!(42, r.get_priority());
    /// ```
    fn get_priority(&self) -> u32 {
        fn_context!(d3d9::IDirect3DResource9Ext::get_priority => IDirect3DResource9::GetPriority);
        unsafe { self.as_winapi().GetPriority() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-getprivatedata)\]
    /// IDirect3DResource9::GetPrivateData
    ///
    /// Copies the private data associated with the resource to a provided buffer.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::MOREDATA]
    /// *   [D3DERR::NOTFOUND]
    /// *   Ok(`read_slice`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let mut buf = [0u8; 128];
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    ///
    /// assert_eq!(D3DERR::NOTFOUND, r.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]));
    ///
    /// let name : &[u8] = b"triangle index buffer";
    /// r.set_object_name_a(name).unwrap();
    ///
    /// assert_eq!(name, r.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]).unwrap());
    ///
    /// let mut buf = [0u8];
    /// assert_eq!(D3DERR::MOREDATA, r.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]));
    /// ```
    fn get_private_data_inplace<'s>(&self, guid: &impl AsRef<Guid>, data: &'s mut [u8]) -> Result<&'s [u8], Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::get_private_data_inplace => IDirect3DResource9::GetPrivateData);
        let mut n = fn_param_try_len32!(data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetPrivateData(guid.as_ref().as_ref(), data.as_mut_ptr().cast(), &mut n) })?;
        Ok(&data[..(n as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-gettype)\]
    /// IDirect3DResource9::GetType
    ///
    /// Returns the type of the resource.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// assert_eq!(r.get_type(), ResourceType::IndexBuffer);
    /// ```
    fn get_type(&self) -> ResourceType {
        fn_context!(d3d9::IDirect3DResource9Ext::get_type => IDirect3DResource9::GetType);
        ResourceType::from_unchecked(unsafe { self.as_winapi().GetType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-preload)\]
    /// IDirect3DResource9::PreLoad
    ///
    /// Hint to Direct3D 9 that a [d3d::Pool::Managed] resource will be needed soon.
    ///
    /// Does nothing for unmanaged resources, or if "thashing" conditions were detected (where e.g. more resources were needed for a frame than could be loaded simultaniously.)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// r.preload();
    /// ```
    fn preload(&self) {
        fn_context!(d3d9::IDirect3DResource9Ext::preload => IDirect3DResource9::PreLoad);
        unsafe { self.as_winapi().PreLoad() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setpriority)\]
    /// IDirect3DResource9::SetPriority
    ///
    /// Used to manage [d3d::Pool::Managed] resource priorities.
    /// Lower priority resources will be unloaded in favor of higher priority resources if necessary.
    /// The default priority is `0`.
    ///
    /// ### Returns
    /// *   The previous priority for [d3d::Pool::Managed] resources.
    /// *   `0` for other resources.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// assert_eq!(0, r.set_priority(0));
    /// assert_eq!(0, r.set_priority(!0));
    /// assert_eq!(!0, r.set_priority(0));
    /// ```
    ///
    /// ### See Also
    /// *   <https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9-resource-priority>
    fn set_priority(&self, priority: u32) -> u32 {
        fn_context!(d3d9::IDirect3DResource9Ext::set_priority => IDirect3DResource9::SetPriority);
        unsafe { self.as_winapi().SetPriority(priority) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData
    ///
    /// Associate arbitrary data with a resource and guid.
    ///
    /// ### ⚠️ Safety ⚠️
    /// It's possible for third party code to impose soundness requirements on the data associated with a given resource and guid.
    /// The existence of the `D3DSPD_IUNKNOWN` flag implies it's valid for one such requirement to be "points to a valid COM object."
    /// Since this function does not and cannot enforce such requirements, it is `unsafe`.
    ///
    /// I wouldn't be suprised if allocation failures - or sufficiently large buffers causing 32-bit overflows - might also result in undefined behavior.
    ///
    /// This function is otherwise sound.
    /// Additionally, I would consider it sound to make a wrapper around this function, so long as you enforce any invariants associated with the `guid` in question.
    /// If you generated the GUID yourself, you can presumably choose your own desired invariants, if any.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// unsafe { r.set_private_data(&wkpdid::D3DDebugObjectName, b"triangle index buffer") }.unwrap();
    /// // prefer set_object_name{,_a,_w} as a safe alternative to the above:
    /// r.set_object_name("triangle index buffer").unwrap();
    /// ```
    ///
    /// ### See Also
    /// *   [IDirect3DResource9Ext::get_private_data_inplace]
    /// *   [IDirect3DResource9Ext::set_object_name]
    /// *   [IDirect3DResource9Ext::set_object_name_a]
    /// *   [IDirect3DResource9Ext::set_object_name_w]
    /// *   [IDirect3DVolume9Ext::set_private_data]
    unsafe fn set_private_data(&self, guid: &impl AsRef<Guid>, data: &[u8]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::set_private_data => IDirect3DResource9::SetPrivateData);
        let n = fn_param_try_len32!(data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetPrivateData(guid.as_ref().as_ref(), data.as_ptr().cast(), n, 0) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData(WKPDID_D3DDebugObjectName, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// r.set_object_name("triangle index buffer").unwrap();
    /// ```
    fn set_object_name(&self, name: &str) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::set_object_name => IDirect3DResource9::SetPrivateData);
        self.set_object_name_a(name.as_bytes())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData(WKPDID_D3DDebugObjectName, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// r.set_object_name_a(b"triangle index buffer").unwrap();
    /// ```
    fn set_object_name_a(&self, name: &[u8]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::set_object_name_a => IDirect3DResource9::SetPrivateData);
        unsafe { self.set_private_data(&wkpdid::D3DDebugObjectName, name) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dresource9-setprivatedata)\]
    /// IDirect3DResource9::SetPrivateData(WKPDID_D3DDebugObjectNameW, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let r = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// r.set_object_name_w(abistr::cstr16!("triangle index buffer").to_units()).unwrap();
    /// ```
    fn set_object_name_w(&self, name: &[u16]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DResource9Ext::set_object_name_w => IDirect3DResource9::SetPrivateData);
        unsafe { self.set_private_data(&wkpdid::D3DDebugObjectNameW, bytemuck::cast_slice(name)) }
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?
}

impl<T: AsSafe<IDirect3DResource9>> IDirect3DResource9Ext for T {}



// TODO: integration tests



//#cpp2rust IDirect3DResource9                      = d3d9::Resource
//#cpp2rust IDirect3DResource9                      = d3d9::IDirect3DResource9Ext

//TODO:     IDirect3DResource9::GetPrivateData      = d3d9::IDirect3DResource9Ext::get_private_data_com
//TODO:     IDirect3DResource9::SetPrivateData      = d3d9::IDirect3DResource9Ext::set_private_data_com

//#cpp2rust D3D_SET_OBJECT_NAME_A                   = d3d9::IDirect3DResource9Ext::set_object_name
//#cpp2rust D3D_SET_OBJECT_NAME_N_A                 = d3d9::IDirect3DResource9Ext::set_object_name
//#cpp2rust D3D_SET_OBJECT_NAME_A                   = d3d9::IDirect3DResource9Ext::set_object_name_a
//#cpp2rust D3D_SET_OBJECT_NAME_N_A                 = d3d9::IDirect3DResource9Ext::set_object_name_a
//#cpp2rust D3D_SET_OBJECT_NAME_N_W                 = d3d9::IDirect3DResource9Ext::set_object_name_w
//#cpp2rust D3D_SET_OBJECT_NAME_W                   = d3d9::IDirect3DResource9Ext::set_object_name_w
