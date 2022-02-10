#![allow(dead_code)] // TODO: remove

use crate::*;
use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9::*;
use winapi::shared::d3d9types::*;
use winapi::um::unknwnbase::IUnknown;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9)\]
/// (extends [Resource])
/// [Texture], [CubeTexture], or [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct BaseTexture(pub(crate) mcom::Rc<IDirect3DBaseTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9)\]
/// (extends [BaseTexture])
/// 6-faced 2D texture for use with [cube mapping](https://en.wikipedia.org/wiki/Cube_mapping)
#[derive(Clone)] #[repr(transparent)]
pub struct CubeTexture(pub(crate) mcom::Rc<IDirect3DCubeTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9)\]
/// (extends [BaseTexture])
/// A dense 2-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct Texture(pub(crate) mcom::Rc<IDirect3DTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9)\]
/// (extends [BaseTexture])
/// A dense 3-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct VolumeTexture(pub(crate) mcom::Rc<IDirect3DVolumeTexture9>);

unsafe impl AsSafe<IUnknown                 > for BaseTexture   { fn as_safe(&self) -> &IUnknown { &***self.0 } }
unsafe impl AsSafe<IUnknown                 > for CubeTexture   { fn as_safe(&self) -> &IUnknown { &****self.0 } }
unsafe impl AsSafe<IUnknown                 > for Texture       { fn as_safe(&self) -> &IUnknown { &****self.0 } }
unsafe impl AsSafe<IUnknown                 > for VolumeTexture { fn as_safe(&self) -> &IUnknown { &****self.0 } }

unsafe impl AsSafe<IDirect3DResource9       > for BaseTexture   { fn as_safe(&self) -> &IDirect3DResource9 { &**self.0 } }
unsafe impl AsSafe<IDirect3DResource9       > for CubeTexture   { fn as_safe(&self) -> &IDirect3DResource9 { &***self.0 } }
unsafe impl AsSafe<IDirect3DResource9       > for Texture       { fn as_safe(&self) -> &IDirect3DResource9 { &***self.0 } }
unsafe impl AsSafe<IDirect3DResource9       > for VolumeTexture { fn as_safe(&self) -> &IDirect3DResource9 { &***self.0 } }

unsafe impl AsSafe<IDirect3DBaseTexture9    > for BaseTexture   { fn as_safe(&self) -> &IDirect3DBaseTexture9 { &*self.0 } }
unsafe impl AsSafe<IDirect3DBaseTexture9    > for CubeTexture   { fn as_safe(&self) -> &IDirect3DBaseTexture9 { &**self.0 } }
unsafe impl AsSafe<IDirect3DBaseTexture9    > for Texture       { fn as_safe(&self) -> &IDirect3DBaseTexture9 { &**self.0 } }
unsafe impl AsSafe<IDirect3DBaseTexture9    > for VolumeTexture { fn as_safe(&self) -> &IDirect3DBaseTexture9 { &**self.0 } }

unsafe impl AsSafe<IDirect3DCubeTexture9    > for CubeTexture   { fn as_safe(&self) -> &IDirect3DCubeTexture9    { &*self.0 } }
unsafe impl AsSafe<IDirect3DTexture9        > for Texture       { fn as_safe(&self) -> &IDirect3DTexture9        { &*self.0 } }
unsafe impl AsSafe<IDirect3DVolumeTexture9  > for VolumeTexture { fn as_safe(&self) -> &IDirect3DVolumeTexture9  { &*self.0 } }



/// # Textures
/// Bind/Create/Update [Texture]s, [CubeTexture]s, and [VolumeTexture]s
impl SafeDevice {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settexture)\]
    /// IDirect3DDevice9::SetTexture
    ///
    /// Assigns a texture to a stage for a device.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `stage` >= `self.caps().MaxSimultaneousTextures`
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
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
        fn_context!(d3d9::SafeDevice::set_texture => IDirect3DDevice9::SetTexture);
        if stage >= self.caps().MaxSimultaneousTextures {
            fn_err!(D3DERR::INVALIDCALL)
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
    /// *   [D3DERR::INVALIDCALL]       If the device is a pure device?
    /// *   [D3DERR::INVALIDCALL]       If `stage` >= `device.caps().MaxSimultaneousTextures`
    /// *   Ok(Some([BaseTexture]))     If a texture was bound to that stage
    /// *   Ok(None)                    If no texture was bound to that stage
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// device.set_texture(1, &texture).unwrap();
    ///
    /// assert!(device.get_texture(0).unwrap().is_none());
    /// assert_eq!(device.get_texture(1).unwrap().unwrap().as_raw(), (*texture).as_raw());
    /// ```
    pub fn get_texture(&self, stage: u32) -> Result<Option<BaseTexture>, MethodError> {
        fn_context!(d3d9::SafeDevice::get_texture => IDirect3DDevice9::GetTexture);
        if stage >= self.caps().MaxSimultaneousTextures {
            fn_err!(D3DERR::INVALIDCALL)
        } else {
            // Safe, because we force-initialized up to MaxSimultaneousTextures (see SOUND1)
            unsafe { self.device().get_texture(stage) }
        }
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9)\]
/// IDirect3DBaseTexture9 extension methods
///
/// ### Methods
/// | thindx                                                        | docs.microsoft.com        | Description |
/// | ------------------------------------------------------------- | ------------------------- | ----------- |
/// | [generate_mip_sub_levels](Self::generate_mip_sub_levels)      | [GenerateMipSubLevels]    | Generate mipmap sublevels.
/// | [get_auto_gen_filter_type](Self::get_auto_gen_filter_type)    | [GetAutoGenFilterType]    | Get the filter type that is used for automatically generated mipmap sublevels.
/// | [get_level_count](Self::get_level_count)                      | [GetLevelCount]           | Returns the number of texture levels in a multilevel texture.
/// | [get_lod](Self::get_lod)                                      | [GetLOD]                  | Returns a value clamped to the maximum level-of-detail set for a managed texture (this method is not supported for an unmanaged texture).
/// | [set_auto_gen_filter_type](Self::set_auto_gen_filter_type)    | [SetAutoGenFilterType]    | Set the filter type that is used for automatically generated mipmap sublevels.
/// | [set_lod](Self::set_lod)                                      | [SetLOD]                  | Sets the most detailed level-of-detail for a managed texture.
///
/// [GenerateMipSubLevels]: https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-generatemipsublevels
/// [GetAutoGenFilterType]: https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getautogenfiltertype
/// [GetLevelCount]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlevelcount
/// [GetLOD]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlod
/// [SetAutoGenFilterType]: https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setautogenfiltertype
/// [SetLOD]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setlod
///
pub trait IDirect3DBaseTexture9Ext : AsSafe<IDirect3DBaseTexture9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-generatemipsublevels)\]
    /// IDirect3DBaseTexture9::GenerateMipSubLevels
    ///
    /// Generate mipmap sublevels.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// # let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.generate_mip_sub_levels();
    /// ```
    fn generate_mip_sub_levels(&self) {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::generate_mip_sub_levels => IDirect3DBaseTexture9::GenerateMipSubLevels);
        unsafe { self.as_winapi().GenerateMipSubLevels() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getautogenfiltertype)\]
    /// IDirect3DBaseTexture9::GetAutoGenFilterType
    ///
    /// Get the filter type that is used for automatically generated mipmap sublevels.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// # let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(TextureFilterType::Linear, texture.get_auto_gen_filter_type());
    /// ```
    fn get_auto_gen_filter_type(&self) -> TextureFilterType {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::get_auto_gen_filter_type => IDirect3DBaseTexture9::GetAutoGenFilterType);
        TextureFilterType::from_unchecked(unsafe { self.as_winapi().GetAutoGenFilterType() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlevelcount)\]
    /// IDirect3DBaseTexture9::GetLevelCount
    ///
    /// Returns the number of texture levels in a multilevel texture.
    ///
    /// **Warning:**  If you create a texture with [Usage::AutoGenMipMap] to make that texture automatically generate sublevels, get_level_count always returns 1 for the number of levels.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`1`)                 If created with [Usage::AutoGenMipMap]
    /// *   Ok(`levels`)            Otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// // Automatic level count
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(8, texture.get_level_count()); // [128, 64, 32, 16, 8, 4, 2, 1].len() == 8
    ///
    /// // Explicit level count
    /// let texture = device.create_texture(128, 128, 3, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(3, texture.get_level_count());
    /// ```
    fn get_level_count(&self) -> u32 {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::get_level_count => IDirect3DBaseTexture9::GetLevelCount);
        unsafe { self.as_winapi().GetLevelCount() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlod)\]
    /// IDirect3DBaseTexture9::GetLOD
    ///
    /// Returns a value clamped to the maximum level-of-detail set for a managed texture (this method is not supported for an unmanaged texture).
    ///
    /// ### Returns
    /// *   `lod`   [Pool::Managed] textures
    /// *   `0`     Unmanaged textures
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// assert_eq!(0, texture.get_lod());
    /// assert_eq!(0, texture.set_lod(5));
    /// assert_eq!(5, texture.get_lod());
    ///
    /// // Silently noops for unmanaged textures:
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(0, texture.get_lod());
    /// assert_eq!(0, texture.set_lod(5));
    /// assert_eq!(0, texture.get_lod());
    /// ```
    fn get_lod(&self) -> u32 {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::get_lod => IDirect3DBaseTexture9::GetLOD);
        unsafe { self.as_winapi().GetLOD() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setautogenfiltertype)\]
    /// IDirect3DBaseTexture9::SetAutoGenFilterType
    ///
    /// Changing the filter type "dirties" the mipmap sublevels and causes them to be regenerated.
    ///
    /// The (default) filter type set at texture creation time is [TexF::Linear].
    /// If the driver does not support a linear filter, the filter type will be set to [TexF::Point].
    /// All filter types supported by the driver for regular texture filtering are supported for autogeneration except [TexF::None].
    /// `set_auto_gen_filter_type` will fail unless the driver sets the appropriate `D3DPTFILTERCAPS_MINFxxx` caps.
    /// These values are specified in the TextureFilterCaps and/or CubeTextureFilterCaps members of [Caps].
    ///
    /// For more information about texture filter types, see [TextureFilterType].
    ///
    /// This method has no effect if the texture is not created with [Usage::AutoGenMipMap].
    /// In this case, no failure is returned. For more information about usage constants, see [Usage].
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   On invalid [TextureFilterType]s
    /// *   [D3DERR::INVALIDCALL]   On [TextureFilterType]s not supported by the driver
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// # let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.set_auto_gen_filter_type(TextureFilterType::Point).unwrap();
    /// texture.set_auto_gen_filter_type(TextureFilterType::Linear).unwrap();
    ///
    /// let _ = texture.set_auto_gen_filter_type(TextureFilterType::ConvolutionMono); // may not be supported
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, texture.set_auto_gen_filter_type(TextureFilterType::None).err());
    /// assert_eq!(D3DERR::INVALIDCALL, texture.set_auto_gen_filter_type(TextureFilterType::from_unchecked(9001)).err());
    /// assert_eq!(D3DERR::INVALIDCALL, texture.set_auto_gen_filter_type(TextureFilterType::from_unchecked(!0)).err());
    /// assert_eq!(D3DERR::INVALIDCALL, texture.set_auto_gen_filter_type(TextureFilterType::from_unchecked(!0-4)).err());
    /// assert_eq!(D3DERR::INVALIDCALL, texture.set_auto_gen_filter_type(TextureFilterType::from_unchecked(!0-100)).err());
    /// ```
    fn set_auto_gen_filter_type(&self, filter_type: impl Into<TextureFilterType>) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::set_auto_gen_filter_type => IDirect3DBaseTexture9::SetAutoGenFilterType);
        let hr = unsafe { self.as_winapi().SetAutoGenFilterType(filter_type.into().into()) };
        fn_check_hr!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setlod)\]
    /// IDirect3DBaseTexture9::SetLOD
    ///
    /// Sets the most detailed level-of-detail for a [Pool::Managed] texture.
    ///
    /// ### Returns
    /// *   `0`                 - for nonmanaged textures (e.g. not [Pool::Managed]?)
    /// *   `old_lod` : [u32]   - the previous most detailed level-of-detail supported.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// assert_eq!(0, texture.set_lod(5));
    /// assert_eq!(5, texture.set_lod(6));
    /// assert_eq!(6, texture.set_lod(9001));
    /// assert_eq!(7, texture.set_lod(0)); // 9001 was clamped to `get_level_count()-1`
    ///
    /// // Silently noops for unmanaged textures:
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(0, texture.get_lod());
    /// assert_eq!(0, texture.set_lod(5));
    /// assert_eq!(0, texture.get_lod());
    /// ```
    fn set_lod(&self, new_lod: u32) -> u32 {
        fn_context!(d3d9::IDirect3DBaseTexture9Ext::set_lod => IDirect3DBaseTexture9::SetLOD);
        unsafe { self.as_winapi().SetLOD(new_lod) }
    }
}

impl<T: AsSafe<IDirect3DBaseTexture9>> IDirect3DBaseTexture9Ext for T {}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9)\]
/// IDirect3DCubeTexture9 extension methods
///
/// ### Methods
/// | thindx                                                                            | docs.microsoft.com        | Description |
/// | --------------------------------------------------------------------------------- | ------------------------- | ----------- |
/// | [add_dirty_rect](Self::add_dirty_rect)                                            | [AddDirtyRect]            | Adds a dirty region to a cube texture resource.
/// | [get_cube_map_surface](Self::get_cube_map_surface)                                | [GetCubeMapSurface]       | Retrieves a cube texture map surface.
/// | [get_level_desc](Self::get_level_desc)                                            | [GetLevelDesc]            | Retrieves a description of one face of the specified cube texture level.
/// | [lock_rect_unchecked](Self::lock_rect_unchecked)                                  | [LockRect]                | Locks a rectangle on a cube texture resource.
/// | [unlock_rect](Self::unlock_rect)                                                  | [UnlockRect]              | Unlocks a rectangle on a cube texture resource.
/// | **[IDirect3DBaseTexture9Ext]::\***                                                | **[IDirect3DBaseTexture9]::\***   |
/// | [generate_mip_sub_levels](IDirect3DBaseTexture9Ext::generate_mip_sub_levels)      | [GenerateMipSubLevels]    | Generate mipmap sublevels.
/// | [get_auto_gen_filter_type](IDirect3DBaseTexture9Ext::get_auto_gen_filter_type)    | [GetAutoGenFilterType]    | Get the filter type that is used for automatically generated mipmap sublevels.
/// | [get_level_count](IDirect3DBaseTexture9Ext::get_level_count)                      | [GetLevelCount]           | Returns the number of texture levels in a multilevel texture.
/// | [get_lod](IDirect3DBaseTexture9Ext::get_lod)                                      | [GetLOD]                  | Returns a value clamped to the maximum level-of-detail set for a managed texture (this method is not supported for an unmanaged texture).
/// | [set_auto_gen_filter_type](IDirect3DBaseTexture9Ext::set_auto_gen_filter_type)    | [SetAutoGenFilterType]    | Set the filter type that is used for automatically generated mipmap sublevels.
/// | [set_lod](IDirect3DBaseTexture9Ext::set_lod)                                      | [SetLOD]                  | Sets the most detailed level-of-detail for a managed texture.
///
/// [AddDirtyRect]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-adddirtyrect
/// [GetCubeMapSurface]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-getcubemapsurface
/// [GetLevelDesc]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-getleveldesc
/// [LockRect]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-lockrect
/// [UnlockRect]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-unlockrect
///
/// [IDirect3DBaseTexture9]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9
/// [GenerateMipSubLevels]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-generatemipsublevels
/// [GetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getautogenfiltertype
/// [GetLevelCount]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlevelcount
/// [GetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlod
/// [SetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setautogenfiltertype
/// [SetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setlod
///
pub trait IDirect3DCubeTexture9Ext : AsSafe<IDirect3DCubeTexture9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-adddirtyrect)\]
    /// IDirect3DCubeTexture9::AddDirtyRect
    ///
    /// Adds a dirty region to a cube texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `face` is invalid
    /// *   [D3DERR::INVALIDCALL]   if `bounds` exceeds bounds or has negative dimensions
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_cube_texture(128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.add_dirty_rect(CubeMapFace::PositiveX, ..).unwrap();
    /// texture.add_dirty_rect(CubeMapFace::PositiveY, (0,0) .. (128,128)).unwrap();
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect(CubeMapFace::PositiveY, (-1, -1) .. (128, 128)).err(), "out of bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect(CubeMapFace::PositiveY, (0, 0) .. (129, 129)).err(), "out of bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect(CubeMapFace::PositiveY, (128, 128) .. (0, 0)).err(), "inverted bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect(CubeMapFace::PositiveY, (i32::MIN, i32::MIN) .. (i32::MAX, i32::MAX)).err(), "out of bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect(CubeMapFace::from_unchecked(6), ..).err(), "invalid face");
    /// ```
    fn add_dirty_rect(&self, face: impl Into<CubeMapFace>, dirty_rect: impl IntoRectOrFull) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DCubeTexture9Ext::add_dirty_rect => IDirect3DCubeTexture9::AddDirtyRect);
        let face = face.into().into();
        let dirty_rect = dirty_rect.into_rect();
        let dirty_rect = dirty_rect.as_ref().map_or(null(), |dr| &**dr);
        let hr = unsafe { self.as_winapi().AddDirtyRect(face, dirty_rect.cast()) };
        fn_check_hr!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-getcubemapsurface)\]
    /// IDirect3DCubeTexture9::GetCubeMapSurface
    ///
    /// Retrieves a cube texture map surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Surface])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_cube_texture(128, 8, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let surface0px : Surface = texture.get_cube_map_surface(CubeMapFace::PositiveX, 0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.get_cube_map_surface(CubeMapFace::PositiveX, 8).err(), "only levels 0 .. 7 are valid");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_cube_map_surface(CubeMapFace::PositiveX, !0).err(), "level !0");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_cube_map_surface(CubeMapFace::from_unchecked(!0), 0).err(), "invalid face");
    /// ```
    fn get_cube_map_surface(&self, face: impl Into<CubeMapFace>, level: u32) -> Result<Surface, MethodError> {
        fn_context!(d3d9::IDirect3DCubeTexture9Ext::get_cube_map_surface => IDirect3DCubeTexture9::GetCubeMapSurface);
        let face = face.into().into();
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().GetCubeMapSurface(face, level, &mut surface) };
        fn_check_hr!(hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-getleveldesc)\]
    /// IDirect3DCubeTexture9::GetLevelDesc
    ///
    /// Retrieves a description of one face of the specified cube texture level.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `level` >= `get_level_count`
    /// *   Ok([SurfaceDesc])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_cube_texture(128, 8, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let desc : SurfaceDesc = texture.get_level_desc(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(8).err(), "only levels 0..7 are valid");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0).err(), "!0");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0-4).err(), "!0-4");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0-100).err(), "!0-100");
    /// ```
    fn get_level_desc(&self, level: u32) -> Result<SurfaceDesc, MethodError> {
        fn_context!(d3d9::IDirect3DCubeTexture9Ext::get_level_desc => IDirect3DCubeTexture9::GetLevelDesc);
        let mut desc = SurfaceDesc::zeroed();
        let hr = unsafe { self.as_winapi().GetLevelDesc(level, &mut *desc) };
        fn_check_hr!(hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-lockrect)\]
    /// IDirect3DCubeTexture9::LockRect
    ///
    /// Locks a rectangle on a cube texture resource.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `face` must be a valid cubemap face
    /// *   `level` must be a valid mipmap level
    /// *   `rect` must be `..` or a valid subregion of the surface in question
    /// *   `self` should be lockable in the style specified by `flags`... and not already locked?
    /// *   `self` may need to be unlocked again before being bound, drawn, or released
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([d3d::LockedRect])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// const S : usize = 128;
    /// let texture = device.create_cube_texture(S as u32, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let data = [[Color::argb(0xFF112233); S]; S];
    /// unsafe {
    ///     let lock = texture.lock_rect_unchecked(CubeMapFace::PositiveX, 0, .., Lock::None).unwrap();
    ///     for y in 0..S {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), S);
    ///     }
    ///     texture.unlock_rect(CubeMapFace::PositiveX, 0).unwrap();
    /// }
    /// ```
    unsafe fn lock_rect_unchecked(&self, face: impl Into<CubeMapFace>, level: u32, rect: impl IntoRectOrFull, flags: impl Into<Lock>) -> Result<LockedRect, MethodError> {
        fn_context!(d3d9::IDirect3DCubeTexture9Ext::lock_rect_unchecked => IDirect3DCubeTexture9::LockRect);
        let face = face.into().into();
        let rect = rect.into_rect();
        let rect = rect.as_ref().map_or(null(), |r| &**r);
        let flags = flags.into().into();
        let mut locked = LockedRect::zeroed();
        let hr = unsafe { self.as_winapi().LockRect(face, level, locked.as_mut(), rect.cast(), flags) };
        fn_check_hr!(hr)?;
        Ok(locked)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dcubetexture9-unlockrect)\]
    /// IDirect3DCubeTexture9::UnlockRect
    ///
    /// Unlocks a rectangle on a cube texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// const S : usize = 128;
    /// let texture = device.create_cube_texture(S as u32, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let data = [[Color::argb(0xFF112233); S]; S];
    /// unsafe {
    ///     let lock = texture.lock_rect_unchecked(CubeMapFace::PositiveX, 0, .., Lock::None).unwrap();
    ///     for y in 0..S {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), S);
    ///     }
    ///     texture.unlock_rect(CubeMapFace::PositiveX, 0).unwrap();
    /// }
    /// // TODO
    /// ```
    fn unlock_rect(&self, face: impl Into<CubeMapFace>, level: u32) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DCubeTexture9Ext::unlock_rect => IDirect3DCubeTexture9::UnlockRect);
        let hr = unsafe { self.as_winapi().UnlockRect(face.into().into(), level) };
        fn_check_hr!(hr)
    }
}

impl<T: AsSafe<IDirect3DCubeTexture9>> IDirect3DCubeTexture9Ext for T {}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9)\]
/// IDirect3DTexture9 extension methods
///
/// ### Methods
/// | thindx                                                                            | docs.microsoft.com        | Description |
/// | --------------------------------------------------------------------------------- | ------------------------- | ----------- |
/// | [add_dirty_rect](Self::add_dirty_rect)                                            | [AddDirtyRect]            | Adds a dirty region to a texture resource.
/// | [get_level_desc](Self::get_level_desc)                                            | [GetLevelDesc]            | Retrieves a level description of a texture resource.
/// | [get_surface_level](Self::get_surface_level)                                      | [GetSurfaceLevel]         | Retrieves the specified texture surface level.
/// | [lock_rect_unchecked](Self::lock_rect_unchecked)                                  | [LockRect]                | Locks a rectangle on a texture resource.
/// | [unlock_rect](Self::unlock_rect)                                                  | [UnlockRect]              | Unlocks a rectangle on a texture resource.
/// | **[IDirect3DBaseTexture9Ext]::\***                                                | **[IDirect3DBaseTexture9]::\***   |
/// | [generate_mip_sub_levels](IDirect3DBaseTexture9Ext::generate_mip_sub_levels)      | [GenerateMipSubLevels]    | Generate mipmap sublevels.
/// | [get_auto_gen_filter_type](IDirect3DBaseTexture9Ext::get_auto_gen_filter_type)    | [GetAutoGenFilterType]    | Get the filter type that is used for automatically generated mipmap sublevels.
/// | [get_level_count](IDirect3DBaseTexture9Ext::get_level_count)                      | [GetLevelCount]           | Returns the number of texture levels in a multilevel texture.
/// | [get_lod](IDirect3DBaseTexture9Ext::get_lod)                                      | [GetLOD]                  | Returns a value clamped to the maximum level-of-detail set for a managed texture (this method is not supported for an unmanaged texture).
/// | [set_auto_gen_filter_type](IDirect3DBaseTexture9Ext::set_auto_gen_filter_type)    | [SetAutoGenFilterType]    | Set the filter type that is used for automatically generated mipmap sublevels.
/// | [set_lod](IDirect3DBaseTexture9Ext::set_lod)                                      | [SetLOD]                  | Sets the most detailed level-of-detail for a managed texture.
///
/// [AddDirtyRect]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-adddirtyrect
/// [GetLevelDesc]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-getleveldesc
/// [GetSurfaceLevel]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-getsurfacelevel
/// [LockRect]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-lockrect
/// [UnlockRect]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-unlockrect
///
/// [IDirect3DBaseTexture9]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9
/// [GenerateMipSubLevels]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-generatemipsublevels
/// [GetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getautogenfiltertype
/// [GetLevelCount]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlevelcount
/// [GetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlod
/// [SetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setautogenfiltertype
/// [SetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setlod
///
pub trait IDirect3DTexture9Ext : AsSafe<IDirect3DTexture9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-adddirtyrect)\]
    /// IDirect3DTexture9::AddDirtyRect
    ///
    /// Adds a dirty region to a texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `bounds` exceeds bounds or has negative dimensions
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.add_dirty_rect(..).unwrap();
    /// texture.add_dirty_rect((0,0) .. (128,128)).unwrap();
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect((-1, -1) .. (128, 128)).err(), "out of bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect((0, 0) .. (129, 129)).err(), "out of bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect((128, 128) .. (0, 0)).err(), "inverted bounds");
    /// assert_eq!(D3DERR::INVALIDCALL, texture.add_dirty_rect((i32::MIN, i32::MIN) .. (i32::MAX, i32::MAX)).err(), "out of bounds");
    /// ```
    fn add_dirty_rect(&self, dirty_rect: impl IntoRectOrFull) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DTexture9Ext::add_dirty_rect => IDirect3DTexture9::AddDirtyRect);
        let dirty_rect = dirty_rect.into_rect();
        let dirty_rect = dirty_rect.as_ref().map_or(null(), |dr| &**dr);
        let hr = unsafe { self.as_winapi().AddDirtyRect(dirty_rect.cast()) };
        fn_check_hr!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-getsurfacelevel)\]
    /// IDirect3DTexture9::GetSurfaceLevel
    ///
    /// Retrieves a texture map surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Surface])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let surface0px : Surface = texture.get_surface_level(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.get_surface_level(8).err(), "only levels 0 .. 7 are valid");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_surface_level(!0).err(), "level !0");
    /// ```
    fn get_surface_level(&self, level: u32) -> Result<Surface, MethodError> {
        fn_context!(d3d9::IDirect3DTexture9Ext::get_surface_level => IDirect3DTexture9::GetSurfaceLevel);
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().GetSurfaceLevel(level, &mut surface) };
        fn_check_hr!(hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-getleveldesc)\]
    /// IDirect3DTexture9::GetLevelDesc
    ///
    /// Retrieves a description of one face of the specified cube texture level.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `level` >= `get_level_count`
    /// *   Ok([SurfaceDesc])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let desc : SurfaceDesc = texture.get_level_desc(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(8).err(), "only levels 0..7 are valid");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0).err(), "!0");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0-4).err(), "!0-4");
    /// # assert_eq!(D3DERR::INVALIDCALL, texture.get_level_desc(!0-100).err(), "!0-100");
    /// ```
    fn get_level_desc(&self, level: u32) -> Result<SurfaceDesc, MethodError> {
        fn_context!(d3d9::IDirect3DTexture9Ext::get_level_desc => IDirect3DTexture9::GetLevelDesc);
        let mut desc = SurfaceDesc::zeroed();
        let hr = unsafe { self.as_winapi().GetLevelDesc(level, &mut *desc) };
        fn_check_hr!(hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-lockrect)\]
    /// IDirect3DTexture9::LockRect
    ///
    /// Locks a rectangle on a cube texture resource.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `level` must be a valid mipmap level
    /// *   `rect` must be `..` or a valid subregion of the surface in question
    /// *   `self` should be lockable in the style specified by `flags`... and not already locked?
    /// *   `self` may need to be unlocked again before being bound, drawn, or released
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([D3DLOCKED_RECT])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// const W : usize = 128;
    /// const H : usize = 128;
    /// let texture = device.create_texture(W as u32, H as u32, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let data = [[Color::argb(0xFF112233); W]; H];
    /// unsafe {
    ///     let lock = texture.lock_rect_unchecked(0, .., Lock::None).unwrap();
    ///     for y in 0..H {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), W);
    ///     }
    ///     texture.unlock_rect(0).unwrap();
    /// }
    /// ```
    unsafe fn lock_rect_unchecked(&self, level: u32, rect: impl IntoRectOrFull, flags: impl Into<Lock>) -> Result<D3DLOCKED_RECT, MethodError> {
        fn_context!(d3d9::IDirect3DTexture9Ext::lock_rect_unchecked => IDirect3DTexture9::LockRect);
        let rect = rect.into_rect();
        let rect = rect.as_ref().map_or(null(), |r| &**r);
        let flags = flags.into().into();
        let mut locked = unsafe { std::mem::zeroed::<D3DLOCKED_RECT>() };
        let hr = unsafe { self.as_winapi().LockRect(level, &mut locked, rect.cast(), flags) };
        fn_check_hr!(hr)?;
        Ok(locked)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dtexture9-unlockrect)\]
    /// IDirect3DTexture9::UnlockRect
    ///
    /// Unlocks a rectangle on a cube texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = safe_device_pure();
    /// const W : usize = 128;
    /// const H : usize = 128;
    /// let texture = device.create_texture(W as u32, H as u32, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let data = [[Color::argb(0xFF112233); W]; H];
    /// unsafe {
    ///     let lock = texture.lock_rect_unchecked(0, .., Lock::None).unwrap();
    ///     for y in 0..H {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), W);
    ///     }
    ///     texture.unlock_rect(0).unwrap();
    /// }
    /// ```
    fn unlock_rect(&self, level: u32) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DTexture9Ext::unlock_rect => IDirect3DTexture9::UnlockRect);
        let hr = unsafe { self.as_winapi().UnlockRect(level) };
        fn_check_hr!(hr)
    }
}

impl<T: AsSafe<IDirect3DTexture9>> IDirect3DTexture9Ext for T {}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9)\]
/// IDirect3DVolumeTexture9 extension methods
///
/// ### Methods
/// | thindx                                                                            | docs.microsoft.com        | Description |
/// | --------------------------------------------------------------------------------- | ------------------------- | ----------- |
/// | [add_dirty_box](Self::add_dirty_box)                                              | [AddDirtyBox]             | Adds a dirty region to a volume texture resource.
/// | [get_level_desc](Self::get_level_desc)                                            | [GetLevelDesc]            | Retrieves a level description of a volume texture resource.
/// | [get_volume_level](Self::get_volume_level)                                        | [GetVolumeLevel]          | Retrieves the specified volume texture level.
/// | [lock_box_unchecked](Self::lock_box_unchecked)                                    | [LockBox]                 | Locks a box on a volume texture resource.
/// | [unlock_box](Self::unlock_box)                                                    | [UnlockBox]               | Unlocks a box on a volume texture resource.
/// | **[IDirect3DBaseTexture9Ext]::\***                                                | **[IDirect3DBaseTexture9]::\***   |
/// | [generate_mip_sub_levels](IDirect3DBaseTexture9Ext::generate_mip_sub_levels)      | [GenerateMipSubLevels]    | Generate mipmap sublevels.
/// | [get_auto_gen_filter_type](IDirect3DBaseTexture9Ext::get_auto_gen_filter_type)    | [GetAutoGenFilterType]    | Get the filter type that is used for automatically generated mipmap sublevels.
/// | [get_level_count](IDirect3DBaseTexture9Ext::get_level_count)                      | [GetLevelCount]           | Returns the number of texture levels in a multilevel texture.
/// | [get_lod](IDirect3DBaseTexture9Ext::get_lod)                                      | [GetLOD]                  | Returns a value clamped to the maximum level-of-detail set for a managed texture (this method is not supported for an unmanaged texture).
/// | [set_auto_gen_filter_type](IDirect3DBaseTexture9Ext::set_auto_gen_filter_type)    | [SetAutoGenFilterType]    | Set the filter type that is used for automatically generated mipmap sublevels.
/// | [set_lod](IDirect3DBaseTexture9Ext::set_lod)                                      | [SetLOD]                  | Sets the most detailed level-of-detail for a managed texture.
///
/// [AddDirtyBox]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-adddirtybox
/// [GetLevelDesc]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getleveldesc
/// [GetVolumeLevel]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getvolumelevel
/// [LockBox]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-lockbox
/// [UnlockBox]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-unlockbox
///
/// [IDirect3DBaseTexture9]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9
/// [GenerateMipSubLevels]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-generatemipsublevels
/// [GetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getautogenfiltertype
/// [GetLevelCount]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlevelcount
/// [GetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-getlod
/// [SetAutoGenFilterType]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setautogenfiltertype
/// [SetLOD]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dbasetexture9-setlod
///
pub trait IDirect3DVolumeTexture9Ext : AsSafe<IDirect3DVolumeTexture9> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-adddirtybox)\]
    /// IDirect3DVolumeTexture9::AddDirtyBox
    ///
    /// Adds a dirty region to a volume texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// texture.add_dirty_box(..).unwrap();
    /// texture.add_dirty_box((0,0,0) .. (32,32,32)).unwrap();
    /// texture.add_dirty_box(Box::from((0,0,0) .. (32,32,32))).unwrap();
    /// ```
    fn add_dirty_box(&self, dirty_box: impl IntoBoxOrFull) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DVolumeTexture9Ext::add_dirty_box => IDirect3DVolumeTexture9::AddDirtyBox);
        let dirty_box   = dirty_box.into_box();
        let dirty_box   = dirty_box.as_ref().map_or(null(), |b| &**b);
        let hr = unsafe { self.as_winapi().AddDirtyBox(dirty_box) };
        fn_check_hr!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getleveldesc)\]
    /// IDirect3DVolumeTexture9::GetLevelDesc
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VolumeDesc])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let level0 : VolumeDesc = texture.get_level_desc(0).unwrap();
    /// assert_eq!(level0.format, Format::A8R8G8B8);
    /// assert_eq!(level0.ty,     ResourceType::Volume);
    /// assert_eq!(level0.usage,  Usage::None);
    /// assert_eq!(level0.pool,   Pool::Default);
    /// assert_eq!(level0.width,  32);
    /// assert_eq!(level0.height, 32);
    /// assert_eq!(level0.depth,  32);
    /// ```
    fn get_level_desc(&self, level: u32) -> Result<VolumeDesc, MethodError> {
        fn_context!(d3d9::IDirect3DVolumeTexture9Ext::get_level_desc => IDirect3DVolumeTexture9::GetLevelDesc);
        let mut desc = VolumeDesc::zeroed();
        let hr = unsafe { self.as_winapi().GetLevelDesc(level, &mut *desc) };
        fn_check_hr!(hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-getvolumelevel)\]
    /// IDirect3DVolumeTexture9::GetVolumeLevel
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Volume])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// let level0 : Volume = texture.get_volume_level(0).unwrap();
    /// // get_container, get_desc, get_device, lock_box, ...
    /// let desc : VolumeDesc = level0.get_desc().unwrap();
    /// assert_eq!(desc.format, Format::A8R8G8B8);
    /// assert_eq!(desc.ty,     ResourceType::Volume);
    /// assert_eq!(desc.usage,  Usage::None);
    /// assert_eq!(desc.pool,   Pool::Default);
    /// assert_eq!(desc.width,  32);
    /// assert_eq!(desc.height, 32);
    /// assert_eq!(desc.depth,  32);
    /// ```
    fn get_volume_level(&self, level: u32) -> Result<Volume, MethodError> {
        fn_context!(d3d9::IDirect3DVolumeTexture9Ext::get_volume_level => IDirect3DVolumeTexture9::GetVolumeLevel);
        let mut volume = null_mut();
        let hr = unsafe { self.as_winapi().GetVolumeLevel(level, &mut volume) };
        fn_check_hr!(hr)?;
        Ok(unsafe { Volume::from_raw(volume) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-lockbox)\]
    /// IDirect3DVolumeTexture9::LockBox
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `level` must be a valid mipmap level
    /// *   `box_` must be `..` or a valid subregion of the volume in question
    /// *   `self` should be lockable in the style specified by `flags`... and not already locked?
    /// *   `self` may need to be unlocked again before being bound, drawn, or released
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the texture belongs to [Pool::Default]
    /// *   Ok([d3d::LockedBox])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
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
    unsafe fn lock_box_unchecked(&self, level: u32, box_: impl IntoBoxOrFull, flags: impl Into<Lock>) -> Result<LockedBox, MethodError> {
        fn_context!(d3d9::IDirect3DVolumeTexture9Ext::lock_box_unchecked => IDirect3DVolumeTexture9::LockBox);
        let box_    = box_.into_box();
        let box_    = box_.as_ref().map_or(null(), |b| &**b);
        let flags   = flags.into().into();
        let mut lockedbox = LockedBox::zeroed();
        let hr = unsafe { self.as_winapi().LockBox(level, lockedbox.as_mut(), box_, flags) };
        fn_check_hr!(hr)?;
        Ok(lockedbox)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvolumetexture9-unlockbox)\]
    /// IDirect3DVolumeTexture9::UnlockBox
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the texture wasn't locked
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
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
    fn unlock_box(&self, level: u32) -> Result<(), MethodError> {
        fn_context!(d3d9::IDirect3DVolumeTexture9Ext::unlock_box => IDirect3DVolumeTexture9::UnlockBox);
        let hr = unsafe { self.as_winapi().UnlockBox(level) };
        fn_check_hr!(hr)
    }

    // TODO: Saner texture init/update methods
}

impl<T: AsSafe<IDirect3DVolumeTexture9>> IDirect3DVolumeTexture9Ext for T {}

impl<'t> From<&'t CubeTexture   > for Option<&'t BaseTexture> { fn from(t: &'t CubeTexture      ) -> Self { Some(&*t) } }
impl<'t> From<&'t Texture       > for Option<&'t BaseTexture> { fn from(t: &'t Texture          ) -> Self { Some(&*t) } }
impl<'t> From<&'t VolumeTexture > for Option<&'t BaseTexture> { fn from(t: &'t VolumeTexture    ) -> Self { Some(&*t) } }



//#cpp2rust IDirect3DBaseTexture9                               = d3d9::BaseTexture
//#cpp2rust IDirect3DBaseTexture9                               = d3d9::IDirect3DBaseTexture9Ext
//#cpp2rust IDirect3DCubeTexture9                               = d3d9::CubeTexture
//#cpp2rust IDirect3DCubeTexture9                               = d3d9::IDirect3DCubeTexture9Ext
//#cpp2rust IDirect3DTexture9                                   = d3d9::Texture
//#cpp2rust IDirect3DTexture9                                   = d3d9::IDirect3DTexture9Ext
//#cpp2rust IDirect3DVolumeTexture9                             = d3d9::VolumeTexture
//#cpp2rust IDirect3DVolumeTexture9                             = d3d9::IDirect3DVolumeTexture9Ext
