#![allow(dead_code)] // TODO: remove

use crate::*;

use std::ptr::{null_mut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9)\]
/// (extends [Resource])
/// [Texture], [CubeTexture], or [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct BaseTexture(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DBaseTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9)\]
/// (extends [BaseTexture])
/// 6-faced 2D texture for use with [cube mapping](https://en.wikipedia.org/wiki/Cube_mapping)
#[derive(Clone)] #[repr(transparent)]
pub struct CubeTexture(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DCubeTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9)\]
/// (extends [BaseTexture])
/// A dense 2-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct Texture(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9)\]
/// (extends [BaseTexture])
/// A dense 3-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct VolumeTexture(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DVolumeTexture9>);



/// [TextureBase] creation methods
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    pub(crate) fn create_cube_texture(&self, edge_length: u32, levels: u32, usage: Usage, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<CubeTexture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.0.CreateCubeTexture(edge_length, levels, usage.into(), format.into(), pool.into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateCubeTexture", hr)?;
        Ok(unsafe { CubeTexture::from_raw(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    pub(crate) fn create_texture(&self, width: u32, height: u32, levels: u32, usage: Usage, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<Texture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.0.CreateTexture(width, height, levels, usage.into(), format.into(), pool.into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateTexture", hr)?;
        Ok(unsafe { Texture::from_raw(texture) })
    }
}

// #[test] fn create_cube_texture() {} // TODO
// #[test] fn create_texture() {} // TODO
