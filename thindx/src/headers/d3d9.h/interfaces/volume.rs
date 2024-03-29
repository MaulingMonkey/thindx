use crate::*;
use crate::d3d9::*;

use bytemuck::*;

use winapi::Interface;
use winapi::shared::d3d9::IDirect3DVolume9;
use winapi::um::unknwnbase::IUnknown;

use std::mem::size_of_val;
use std::ptr::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// A dense 3-dimensional region of data, often belonging to a [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct Volume(pub(crate) mcom::Rc<IDirect3DVolume9>);

unsafe impl AsSafe<IUnknown         > for Volume { fn as_safe(&self) -> &IUnknown         { &*self.0 } }
unsafe impl AsSafe<IDirect3DVolume9 > for Volume { fn as_safe(&self) -> &IDirect3DVolume9 { &*self.0 } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// IDirect3DVolume9 extension methods
///
/// ### Methods
/// | thindx                                                        | microsoft.com         | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [free_private_data](Self::free_private_data)                  | [FreePrivateData]     | Frees the specified private data associated with this volume.
/// | [get_container](Self::get_container)                          | [GetContainer]        | Provides access to the parent volume texture object, if this surface is a child level of a volume texture.
/// | [get_desc](Self::get_desc)                                    | [GetDesc]             | Retrieves a description of the volume.
/// | [get_device](Self::get_device)                                | [GetDevice]           | Retrieves the device associated with a volume.
/// | [get_private_data_inplace](Self::get_private_data_inplace)    | [GetPrivateData]      | Copies the private data associated with the volume to a provided buffer.
/// | <span style="opacity: 25%">get_private_data_com</span>        | [GetPrivateData]      | Retrieves a COM object associated with the volume.
/// | [lock_box_unchecked](Self::lock_box_unchecked)                | [LockBox]             | Locks a box on a volume resource.
/// | [set_private_data](Self::set_private_data)                    | [SetPrivateData]      | Associates data with the volume that is intended for use by the application, not by Direct3D.
/// | [set_object_name](Self::set_object_name)                      | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
/// | [set_object_name_a](Self::set_object_name_a)                  | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
/// | [set_object_name_w](Self::set_object_name_w)                  | [SetPrivateData]      | Associates a debug name with the resource for graphics debuggers.
/// | <span style="opacity: 25%">set_private_data_com</span>        | [SetPrivateData]      | Associates a COM object with the resource for use by the application.
/// | [unlock_box](Self::unlock_box)                                | [UnlockBox]           | Unlocks a box on a volume resource.
///
/// [FreePrivateData]:  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-freeprivatedata
/// [GetContainer]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getcontainer
/// [GetDesc]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdesc
/// [GetDevice]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdevice
/// [GetPrivateData]:   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getprivatedata
/// [LockBox]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-lockbox
/// [SetPrivateData]:   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata
/// [UnlockBox]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-unlockbox
///
pub trait IDirect3DVolume9Ext : AsSafe<IDirect3DVolume9> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-freeprivatedata)\]
    /// IDirect3DVolume9::FreePrivateData
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
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    ///
    /// // wkpdid::D3DDebugObjectName not yet set:
    /// assert_eq!(D3DERR::NOTFOUND, v.free_private_data(&wkpdid::D3DDebugObjectName));
    ///
    /// v.set_object_name("triangle index buffer").unwrap();
    ///
    /// // wkpdid::D3DDebugObjectName was set by set_object_name, so now it should succeed:
    /// v.free_private_data(&wkpdid::D3DDebugObjectName).unwrap();
    /// ```
    fn free_private_data(&self, guid: &impl AsRef<Guid>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::free_private_data => IDirect3DVolume9::FreePrivateData);
        fn_check_hr!(unsafe { self.as_winapi().FreePrivateData(guid.as_ref().as_ref()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getcontainer)\]
    /// IDirect3DVolume9::GetContainer
    ///
    /// Provides access to the parent volume texture object, if this surface is a child level of a volume texture.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [E::NOINTERFACE]        If the container doesn't implement the interface `C`
    /// *   Ok(`C`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    ///
    /// assert_eq!(E::NOINTERFACE, v.get_container::<Device>().map(|_| ())); // use .get_device() instead
    /// let t2 : VolumeTexture = v.get_container().unwrap();
    ///
    /// let t  = mcom::Rc::<winapi::shared::d3d9::IDirect3DVolumeTexture9>::from(t );
    /// let t2 = mcom::Rc::<winapi::shared::d3d9::IDirect3DVolumeTexture9>::from(t2);
    /// assert_eq!(t.as_ptr(), t2.as_ptr());
    /// ```
    fn get_container<C: Raw>(&self) -> Result<C, Error> where C::Raw : Interface {
        fn_context!(d3d9::IDirect3DVolume9Ext::get_container => IDirect3DVolume9::GetContainer);
        let mut container = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetContainer(&C::Raw::uuidof(), &mut container) })?;
        Ok(unsafe { C::from_raw(container.cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdesc)\]
    /// IDirect3DVolume9::GetDesc
    ///
    /// Retrieves a description of the volume.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VolumeDesc])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// dbg!(v.get_desc().unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// v.get_desc().unwrap() = VolumeDesc {
    ///     format: Format(D3DFMT_A8R8G8B8),
    ///     ty: ResourceType::Volume,
    ///     usage: Usage::None,
    ///     pool: Pool::Default,
    ///     width: 8,
    ///     height: 8,
    ///     depth: 8,
    /// }
    /// ```
    fn get_desc(&self) -> Result<VolumeDesc, Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::get_desc => IDirect3DVolume9::GetDesc);
        let mut desc = VolumeDesc::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetDesc(&mut *desc) })?;
        Ok(desc)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getdevice)\]
    /// IDirect3DVolume9::GetDevice
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
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// let dev2 = v.get_device().unwrap();
    ///
    /// let dev  = mcom::Rc::<winapi::shared::d3d9::IDirect3DDevice9>::from(dev );
    /// let dev2 = mcom::Rc::<winapi::shared::d3d9::IDirect3DDevice9>::from(dev2);
    /// assert_eq!(dev.as_ptr(), dev2.as_ptr());
    /// ```
    fn get_device(&self) -> Result<Device, Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::get_device => IDirect3DVolume9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(unsafe { Device::from_raw(device) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-getprivatedata)\]
    /// IDirect3DVolume9::GetPrivateData
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
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    ///
    /// assert_eq!(D3DERR::NOTFOUND, v.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]));
    ///
    /// let name : &[u8] = b"triangle index buffer";
    /// v.set_object_name_a(name).unwrap();
    ///
    /// assert_eq!(name, v.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]).unwrap());
    ///
    /// let mut buf = [0u8];
    /// assert_eq!(D3DERR::MOREDATA, v.get_private_data_inplace(&wkpdid::D3DDebugObjectName, &mut buf[..]));
    /// ```
    fn get_private_data_inplace<'s>(&self, guid: &impl AsRef<Guid>, data: &'s mut [u8]) -> Result<&'s [u8], Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::get_private_data_inplace => IDirect3DVolume9::GetPrivateData);
        let mut n = fn_param_try_len32!(data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetPrivateData(guid.as_ref().as_ref(), data.as_mut_ptr().cast(), &mut n) })?;
        Ok(&data[..(n as usize)])
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData
    ///
    /// Associate arbitrary data with a volume and guid.
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
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// unsafe { v.set_private_data(&wkpdid::D3DDebugObjectName, b"my volume") }.unwrap();
    /// // prefer set_object_name{,_a,_w} as a safe alternative to the above:
    /// v.set_object_name("my volume").unwrap();
    /// ```
    ///
    /// ### See Also
    /// *   [IDirect3DVolume9Ext::get_private_data_inplace]
    /// *   [IDirect3DVolume9Ext::set_object_name]
    /// *   [IDirect3DVolume9Ext::set_object_name_a]
    /// *   [IDirect3DVolume9Ext::set_object_name_w]
    /// *   [IDirect3DResource9Ext::set_private_data]
    unsafe fn set_private_data(&self, guid: &impl AsRef<Guid>, data: &[u8]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::set_private_data => IDirect3DVolume9::SetPrivateData);
        let n = fn_param_try_len32!(data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetPrivateData(guid.as_ref().as_ref(), data.as_ptr().cast(), n, 0) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData w/ D3DSPD_IUNKNOWN
    ///
    /// Associate arbitrary COM objects with a volume and guid.
    ///
    /// ### ⚠️ Safety ⚠️
    /// It's possible for third party code to impose soundness requirements on the data associated with a given volume and guid.
    /// Such soundness requirements might include "is a valid COM pointer to a specific type".
    /// Since this function does not and cannot enforce such requirements, it is `unsafe`.
    ///
    /// I wouldn't be suprised if allocation failures might also result in undefined behavior.
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
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    ///
    /// // I can't think of anything fun to associate with a volume,
    /// // so let's just associate an index buffer for no reason:
    /// const MY_ASSOCIATED_INDEX_BUFFER : Guid = guid!("bbfd6f01-0c62-4f69-8be1-db0e439b6185");
    /// let ib = dev.create_index_buffer(6, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// unsafe { v.set_private_data_com(&MY_ASSOCIATED_INDEX_BUFFER, ib) }.unwrap();
    /// ```
    unsafe fn set_private_data_com(&self, guid: &impl AsRef<Guid>, data: impl AsSafe<IUnknown>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::set_private_data_com => IDirect3DVolume9::SetPrivateData);
        let data : *const IUnknown = data.as_safe();
        unsafe { (*data).AddRef() }; // SetPrivateData takes ownership over 1 refcount
        // N.B.: we don't pass a pointer to a buffer containing the *const IUnknown, we just pass the *const IUnknown directly.
        fn_check_hr!(unsafe { self.as_winapi().SetPrivateData(guid.as_ref().as_ref(), data.cast(), size_of_val(&data) as _, D3DSPD_IUNKNOWN) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData(WKPDID_D3DDebugObjectName, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// v.set_object_name("my volume").unwrap();
    /// ```
    fn set_object_name(&self, name: &str) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::set_object_name => IDirect3DVolume9::SetPrivateData);
        self.set_object_name_a(name.as_bytes())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData(WKPDID_D3DDebugObjectName, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// v.set_object_name_a(b"my volume").unwrap();
    /// ```
    fn set_object_name_a(&self, name: &[u8]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::set_object_name_a => IDirect3DVolume9::SetPrivateData);
        unsafe { self.set_private_data(&wkpdid::D3DDebugObjectName, name) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-setprivatedata)\]
    /// IDirect3DVolume9::SetPrivateData(WKPDID_D3DDebugObjectNameW, ...)
    ///
    /// Set a human-readable name for this object, to make graphics debug captures easier to understand.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// v.set_object_name_w(abistr::cstr16!("my volume").to_units()).unwrap();
    /// ```
    fn set_object_name_w(&self, name: &[u16]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::set_object_name_w => IDirect3DVolume9::SetPrivateData);
        unsafe { self.set_private_data(&wkpdid::D3DDebugObjectNameW, bytemuck::cast_slice(name)) }
    }

    // TODO: set_private_data_unk
    // TODO: get_private_data_unk ?
    // figure out where unsoundness should lie - both of those fns?  set_private_data too, as it can invalidate unknown ptrs?

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-lockbox)\]
    /// IDirect3DVolume9::LockBox
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `box_` must be `..` or a valid subregion of the volume in question
    /// *   `self` should be lockable in the style specified by `flags`... and not already locked?
    /// *   `self` may need to be unlocked again before being bound, drawn, or released
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([d3d::LockedBox])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// let lock = unsafe { v.lock_box_unchecked(.., Lock::None).unwrap() };
    /// // TODO: init data
    /// v.unlock_box().unwrap();
    /// ```
    unsafe fn lock_box_unchecked(&self, box_: impl IntoBoxOrFull, flags: impl Into<Lock>) -> Result<LockedBox, Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::lock_box_unchecked => IDirect3DVolume9::LockBox);
        let mut locked = LockedBox::zeroed();
        let box_ = box_.into_box();
        let box_ = box_.as_ref().map_or(null(), |b| &**b);
        fn_check_hr!(unsafe { self.as_winapi().LockBox(locked.as_mut(), box_, flags.into().into()) })?;
        Ok(locked)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolume9-unlockbox)\]
    /// IDirect3DVolume9::UnlockBox
    ///
    /// Unlocks a box on a volume resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let dev = device_pure();
    /// let t = dev.create_volume_texture(8, 8, 8, 0, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let v = t.get_volume_level(0).unwrap();
    /// let lock = unsafe { v.lock_box_unchecked(.., Lock::None).unwrap() };
    /// # assert_eq!(D3DERR::INVALIDCALL, unsafe { v.lock_box_unchecked(.., Lock::None) }, "was already locked");
    /// // ...
    /// v.unlock_box().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, v.unlock_box(), "was already unlocked");
    /// ```
    fn unlock_box(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DVolume9Ext::unlock_box => IDirect3DVolume9::UnlockBox);
        fn_check_hr!(unsafe { self.as_winapi().UnlockBox() })
    }
}

impl<T: AsSafe<IDirect3DVolume9>> IDirect3DVolume9Ext for T {}



// TODO: integration tests



//#cpp2rust IDirect3DVolume9                        = d3d9::Volume
//#cpp2rust IDirect3DVolume9                        = d3d9::IDirect3DVolume9Ext

//TODO:     IDirect3DVolume9::GetPrivateData        = d3d9::IDirect3DVolume9Ext::get_private_data_com

//#cpp2rust D3D_SET_OBJECT_NAME_A                   = d3d9::IDirect3DVolume9Ext::set_object_name
//#cpp2rust D3D_SET_OBJECT_NAME_N_A                 = d3d9::IDirect3DVolume9Ext::set_object_name
//#cpp2rust D3D_SET_OBJECT_NAME_A                   = d3d9::IDirect3DVolume9Ext::set_object_name_a
//#cpp2rust D3D_SET_OBJECT_NAME_N_A                 = d3d9::IDirect3DVolume9Ext::set_object_name_a
//#cpp2rust D3D_SET_OBJECT_NAME_N_W                 = d3d9::IDirect3DVolume9Ext::set_object_name_w
//#cpp2rust D3D_SET_OBJECT_NAME_W                   = d3d9::IDirect3DVolume9Ext::set_object_name_w
