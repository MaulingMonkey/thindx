#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::shared::d3d9types::*;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9)\]
/// (extends [Resource])
/// [Texture], [CubeTexture], or [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct BaseTexture(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DBaseTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9)\]
/// (extends [BaseTexture])
/// 6-faced 2D texture for use with [cube mapping](https://en.wikipedia.org/wiki/Cube_mapping)
#[derive(Clone)] #[repr(transparent)]
pub struct CubeTexture(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DCubeTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9)\]
/// (extends [BaseTexture])
/// A dense 2-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct Texture(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9)\]
/// (extends [BaseTexture])
/// A dense 3-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct VolumeTexture(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVolumeTexture9>);



/// # Textures
/// Bind/Create/Update [Texture]s, [CubeTexture]s, and [VolumeTexture]s
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([CubeTexture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// // Create a 6 x 128x128 ARGB cubemap with no mipmaps
    /// let texture = device.create_cube_texture(128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_cube_texture(1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    pub fn create_cube_texture(&self, edge_length: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<CubeTexture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.0.CreateCubeTexture(edge_length, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateCubeTexture", hr)?;
        Ok(unsafe { CubeTexture::from_raw(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)\]
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([Texture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// // Create a 128x128 ARGB texture with no mipmaps
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_texture(1 << 15, 1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    pub fn create_texture(&self, width: u32, height: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<Texture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.0.CreateTexture(width, height, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateTexture", hr)?;
        Ok(unsafe { Texture::from_raw(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture)\]
    /// IDirect3DDevice9::CreateVolumeTexture
    ///
    /// Creates a volume texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::OUTOFMEMORY]
    /// *   Ok([VolumeTexture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// // Create a 32x32x32 volumetric ARGB texture with no mipmaps
    /// let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_volume_texture(1 << 10, 1 << 10, 1 << 10, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    pub fn create_volume_texture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VolumeTexture, MethodError> {
        let mut volumetexture = null_mut();
        let hr = unsafe { self.0.CreateVolumeTexture(width, height, depth, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut volumetexture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateVolumeTexture", hr)?;
        Ok(unsafe { VolumeTexture::from_raw(volumetexture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-gettexture)\]
    /// IDirect3DDevice9::GetTexture
    ///
    /// Retrieves a texture assigned to a stage for a device.
    ///
    /// ### Safety
    ///
    /// *   This function will crash (or worse!) if `set_texture` was never called for `stage`!
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device is a pure device?
    /// *   Ok(Some([BaseTexture]))     If a texture was bound to that stage
    /// *   Ok(None)                    If no texture was bound to that stage
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// // XXX: No texture set for stage 0, this may crash!!!
    /// // let texture = unsafe { device.get_texture(0) }.unwrap();
    ///
    /// unsafe { device.set_texture(0, None) }.unwrap();
    /// let texture0 = unsafe { device.get_texture(0) };
    /// assert!(texture0.unwrap().is_none());
    ///
    /// // XXX: No texture set for stage 1, this may crash!!!
    /// // let texture = unsafe { device.get_texture(1) }.unwrap();
    ///
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// unsafe { device.set_texture(0, &texture) }.unwrap();
    /// assert_eq!(unsafe { device.get_texture(0) }.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// ```
    pub unsafe fn get_texture(&self, stage: u32) -> Result<Option<BaseTexture>, MethodError> {
        let mut texture = null_mut();
        let hr = self.0.GetTexture(stage, &mut texture);
        MethodError::check("IDirect3DDevice9::GetTexture", hr)?;
        Ok(BaseTexture::from_raw_opt(texture))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settexture)\]
    /// IDirect3DDevice9::SetTexture
    ///
    /// Assigns a texture to a stage for a device.
    ///
    /// ### Safety
    ///
    /// *   This function will crash (or worse!) if `stage` is too large (> `MaxSimultaneousTextures`?)
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    ///
    /// unsafe{device.set_texture(0,      &texture  )}.unwrap();
    /// unsafe{device.set_texture(1, Some(&*texture))}.unwrap();
    /// unsafe{device.set_texture(2, None           )}.unwrap();
    /// assert_eq!(unsafe{device.get_texture(0)}.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert_eq!(unsafe{device.get_texture(1)}.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert!(unsafe{device.get_texture(2)}.unwrap().is_none());
    ///
    /// // unsafe{device.set_texture(1000,  &texture)}.unwrap(); // XXX: works on my machine, but might could be doing bad shit like corrupting memory?
    /// // unsafe{device.set_texture(10000, &texture)}.unwrap(); // XXX: crashes on my machine!
    /// ```
    pub unsafe fn set_texture<'t>(&self, stage: u32, texture: impl Into<Option<&'t BaseTexture>>) -> Result<(), MethodError> {
        // TODO: make a more convenient trait for texture binding
        let texture = texture.into();
        let texture = texture.map_or(null_mut(), |t| t.as_raw());
        let hr = self.0.SetTexture(stage, texture);
        MethodError::check("IDirect3DDevice9::SetTexture", hr)
    }
}



/// # Textures
/// Bind/Create/Update [Texture]s, [CubeTexture]s, and [VolumeTexture]s
impl SafeDevice {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settexture)\]
    /// IDirect3DDevice9::SetTexture
    ///
    /// Assigns a texture to a stage for a device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `stage` >= `self.caps().MaxSimultaneousTextures`
    /// *   Ok(())
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = SafeDevice::pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    ///
    /// device.set_texture(0, &texture).unwrap();
    /// device.set_texture(1, Some(&*texture)).unwrap();
    /// device.set_texture(2, None).unwrap();
    ///
    /// assert_eq!(device.get_texture(0).unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert_eq!(device.get_texture(1).unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert!(   device.get_texture(2).unwrap().is_none());
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, device.set_texture(1000,   &texture));
    /// assert_eq!(D3DERR::INVALIDCALL, device.set_texture(10000,  &texture));
    /// assert_eq!(D3DERR::INVALIDCALL, device.set_texture(100000, &texture));
    /// assert_eq!(D3DERR::INVALIDCALL, device.set_texture(!0,     &texture));
    /// ```
    pub fn set_texture<'t>(&self, stage: u32, texture: impl Into<Option<&'t BaseTexture>>) -> Result<(), MethodError> {
        if stage >= self.caps().MaxSimultaneousTextures {
            Err(MethodError("SafeDevice::set_texture", D3DERR::INVALIDCALL))
        } else {
            // Safe thanks to bounds check
            unsafe { self.device().set_texture(stage, texture) }
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-gettexture)\]
    /// IDirect3DDevice9::GetTexture
    ///
    /// Retrieves a texture assigned to a stage for a device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device is a pure device?
    /// *   [D3DERR::INVALIDCALL]       If `stage` >= `device.caps().MaxSimultaneousTextures`
    /// *   Ok(Some([BaseTexture]))     If a texture was bound to that stage
    /// *   Ok(None)                    If no texture was bound to that stage
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = SafeDevice::pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// device.set_texture(1, &texture).unwrap();
    /// 
    /// assert!(device.get_texture(0).unwrap().is_none());
    /// assert_eq!(device.get_texture(1).unwrap().unwrap().as_raw(), (*texture).as_raw());
    /// ```
    pub fn get_texture(&self, stage: u32) -> Result<Option<BaseTexture>, MethodError> {
        if stage >= self.caps().MaxSimultaneousTextures {
            Err(MethodError("SafeDevice::get_texture", D3DERR::INVALIDCALL))
        } else {
            // Safe, because we force-initialized up to MaxSimultaneousTextures (see SOUND1)
            unsafe { self.device().get_texture(stage) }
        }
    }
}



impl VolumeTexture {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-adddirtybox)\]
    /// IDirect3DVolumeTexture9::AddDirtyBox
    ///
    /// Adds a dirty region to a volume texture resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.add_dirty_box(..).unwrap();
    /// texture.add_dirty_box((0,0,0) .. (32,32,32)).unwrap();
    /// texture.add_dirty_box(Box::from((0,0,0) .. (32,32,32))).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    pub fn add_dirty_box(&self, dirty_box: impl IntoBoxOrFull) -> Result<(), MethodError> {
        let dirty_box   = dirty_box.into_box();
        let dirty_box   = dirty_box.as_ref().map_or(null(), |b| &**b);
        let hr = unsafe { self.0.AddDirtyBox(dirty_box) };
        MethodError::check("IDirect3DVolumeTexture9::AddDirtyBox", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getleveldesc)\]
    /// IDirect3DVolumeTexture9::GetLevelDesc
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let level0 : VolumeDesc = texture.get_level_desc(0).unwrap();
    /// assert_eq!(level0.format, Format::A8R8G8B8);
    /// assert_eq!(level0.r#type, ResourceType::Volume);
    /// assert_eq!(level0.usage,  Usage::None);
    /// assert_eq!(level0.pool,   Pool::Default);
    /// assert_eq!(level0.width,  32);
    /// assert_eq!(level0.height, 32);
    /// assert_eq!(level0.depth,  32);
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VolumeDesc])
    pub fn get_level_desc(&self, level: u32) -> Result<VolumeDesc, MethodError> {
        let mut desc = VolumeDesc::default();
        let hr = unsafe { self.0.GetLevelDesc(level, &mut *desc) };
        MethodError::check("IDirect3DVolumeTexture9::GetLevelDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getvolumelevel)\]
    /// IDirect3DVolumeTexture9::GetVolumeLevel
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let level0 : Volume = texture.get_volume_level(0).unwrap();
    /// // get_container, get_desc, get_device, lock_box, ...
    /// let desc : VolumeDesc = level0.get_desc().unwrap();
    /// assert_eq!(desc.format, Format::A8R8G8B8);
    /// assert_eq!(desc.r#type, ResourceType::Volume);
    /// assert_eq!(desc.usage,  Usage::None);
    /// assert_eq!(desc.pool,   Pool::Default);
    /// assert_eq!(desc.width,  32);
    /// assert_eq!(desc.height, 32);
    /// assert_eq!(desc.depth,  32);
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Volume])
    pub fn get_volume_level(&self, level: u32) -> Result<Volume, MethodError> {
        let mut volume = null_mut();
        let hr = unsafe { self.0.GetVolumeLevel(level, &mut volume) };
        MethodError::check("IDirect3DVolumeTexture9::GetVolumeLevel", hr)?;
        Ok(unsafe { Volume::from_raw(volume) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-lockbox)\]
    /// IDirect3DVolumeTexture9::LockBox
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// // Pool::Default textures cannot be locked
    /// let texture = device.create_volume_texture(32, 32, 32, 1, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, unsafe { texture.lock_box_unchecked(0, .., Lock::None) }.err());
    ///
    /// // Pool::Managed textures *can* be locked
    /// let data = [[[Color::argb(0xFF112233); 32]; 32]; 32];
    /// let texture = device.create_volume_texture(32, 32, 32, 1, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// unsafe {
    ///     let bits = texture.lock_box_unchecked(0, Box::from((0,0,0)..(32,32,4)), Lock::None).unwrap();
    ///     for z in 0..32 { for y in 0..32 {
    ///         let src = data[z][y].as_ptr();
    ///         let dst = (bits.pBits as *mut u8).add(y * bits.RowPitch as usize + z * bits.SlicePitch as usize);
    ///         std::ptr::copy(src, dst.cast(), 32);
    ///     }}
    /// }
    /// texture.unlock_box(0).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the texture belongs to [Pool::Default]
    /// *   Ok([D3DLOCKED_BOX])
    pub unsafe fn lock_box_unchecked(&self, level: u32, box_: impl IntoBoxOrFull, lock: impl Into<Lock>) -> Result<D3DLOCKED_BOX, MethodError> {
        let box_    = box_.into_box();
        let box_    = box_.as_ref().map_or(null(), |b| &**b);
        let lock    = lock.into().into();
        let mut lockedbox = std::mem::zeroed::<D3DLOCKED_BOX>();
        let hr = self.0.LockBox(level, &mut lockedbox, box_, lock);
        MethodError::check("IDirect3DVolumeTexture9::LockBox", hr)?;
        Ok(lockedbox)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-unlockbox)\]
    /// IDirect3DVolumeTexture9::UnlockBox
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.unlock_box(0));
    ///
    /// unsafe {
    ///     let bits = texture.lock_box_unchecked(0, .., Lock::None).unwrap();
    ///     // ...copy data to bits.pBits...
    /// }
    /// texture.unlock_box(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.unlock_box(0));
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the texture wasn't locked
    /// *   Ok(`()`)
    pub fn unlock_box(&self, level: u32) -> Result<(), MethodError> {
        let hr = unsafe { self.0.UnlockBox(level) };
        MethodError::check("IDirect3DVolumeTexture9::UnlockBox", hr)
    }

    // TODO: Saner texture init/update methods
}



impl<'t> From<&'t Texture> for Option<&'t BaseTexture> { fn from(t: &'t Texture) -> Self { Some(&*t) } }
