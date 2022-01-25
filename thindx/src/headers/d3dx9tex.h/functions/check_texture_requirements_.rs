use crate::*;

use winapi::shared::d3d9::*;
use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxcheckcubetexturerequirements)\]
/// D3DXCheckCubeTextureRequirements
///
/// ### ⚠️ Safety ⚠️
///
/// This function may be safe, but lacks fuzzing/testing coverage.
/// Likely causes of bugs include allocation overflows, unchecked invalid parameters, buggy runtimes, etc.
///
/// ### Arguments
/// *   `device`            - [d3d::Device] to create the texture with
/// *   `size`              - [None], or [Some]\(square size in pixels\) (in: desired, out: corrected)
/// *   `num_mip_levels`    - [None], or [Some]\(mipmap levels\) (in: desired, out: corrected)
/// *   `usage`             - [d3d::Usage] to create the texture with
/// *   `pool`              - [d3d::Pool] to create the texture with
///
pub unsafe fn check_cube_texture_requirements<'a>(
    device:         &impl AsSafe<IDirect3DDevice9>,
    size:           impl Into<Option<&'a mut u32>>,
    num_mip_levels: impl Into<Option<&'a mut u32>>,
    usage:          impl Into<d3d::Usage>,
    format:         impl Into<Option<&'a mut d3d::Format>>,
    pool:           impl Into<d3d::Pool>,
) -> ErrorKind {
    let device          = device        .as_mut_ptr();
    let size            = size          .into().map_or(null_mut(), |v| v);
    let num_mip_levels  = num_mip_levels.into().map_or(null_mut(), |v| v);
    let format          = format        .into().map_or(null_mut(), |v| v);
    let usage           = usage         .into().into();
    let pool            = pool          .into().into();

    ErrorKind(D3DXCheckCubeTextureRequirements(device, size, num_mip_levels, usage, format.cast(), pool) as _)
}


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxchecktexturerequirements)\]
/// D3DXCheckTextureRequirements
///
/// ### ⚠️ Safety ⚠️
///
/// This function may be safe, but lacks fuzzing/testing coverage.
/// Likely causes of bugs include allocation overflows, unchecked invalid parameters, buggy runtimes, etc.
///
/// ### Arguments
/// *   `device`            - [d3d::Device] to create the texture with
/// *   `width`             - [None], or [Some]\(dimension in pixels\) (in: desired, out: corrected)
/// *   `height`            - [None], or [Some]\(dimension in pixels\) (in: desired, out: corrected)
/// *   `num_mip_levels`    - [None], or [Some]\(mipmap levels\) (in: desired, out: corrected)
/// *   `usage`             - [d3d::Usage] to create the texture with
/// *   `pool`              - [d3d::Pool] to create the texture with
///
pub unsafe fn check_texture_requirements<'a>(
    device:         &impl AsSafe<IDirect3DDevice9>,
    width:          impl Into<Option<&'a mut u32>>,
    height:         impl Into<Option<&'a mut u32>>,
    num_mip_levels: impl Into<Option<&'a mut u32>>,
    usage:          impl Into<d3d::Usage>,
    format:         impl Into<Option<&'a mut d3d::Format>>,
    pool:           impl Into<d3d::Pool>,
) -> ErrorKind {
    let device          = device        .as_mut_ptr();
    let width           = width         .into().map_or(null_mut(), |v| v);
    let height          = height        .into().map_or(null_mut(), |v| v);
    let num_mip_levels  = num_mip_levels.into().map_or(null_mut(), |v| v);
    let format          = format        .into().map_or(null_mut(), |v| v);
    let usage           = usage         .into().into();
    let pool            = pool          .into().into();

    ErrorKind(D3DXCheckTextureRequirements(device, width, height, num_mip_levels, usage, format.cast(), pool) as _)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dxcheckvolumetexturerequirements)\]
/// D3DXCheckVolumeTextureRequirements
///
/// ### ⚠️ Safety ⚠️
///
/// This function may be safe, but lacks fuzzing/testing coverage.
/// Likely causes of bugs include allocation overflows, unchecked invalid parameters, buggy runtimes, etc.
///
/// ### Arguments
/// *   `device`            - [d3d::Device] to create the texture with
/// *   `width`             - [None], or [Some]\(dimension in pixels\) (in: desired, out: corrected)
/// *   `height`            - [None], or [Some]\(dimension in pixels\) (in: desired, out: corrected)
/// *   `depth`             - [None], or [Some]\(dimension in pixels\) (in: desired, out: corrected)
/// *   `num_mip_levels`    - [None], or [Some]\(mipmap levels\) (in: desired, out: corrected)
/// *   `usage`             - [d3d::Usage] to create the texture with
/// *   `pool`              - [d3d::Pool] to create the texture with
///
pub unsafe fn check_volume_texture_requirements<'a>(
    device:         &impl AsSafe<IDirect3DDevice9>,
    width:          impl Into<Option<&'a mut u32>>,
    height:         impl Into<Option<&'a mut u32>>,
    depth:          impl Into<Option<&'a mut u32>>,
    num_mip_levels: impl Into<Option<&'a mut u32>>,
    usage:          impl Into<d3d::Usage>,
    format:         impl Into<Option<&'a mut d3d::Format>>,
    pool:           impl Into<d3d::Pool>,
) -> ErrorKind {
    let device          = device        .as_mut_ptr();
    let width           = width         .into().map_or(null_mut(), |v| v);
    let height          = height        .into().map_or(null_mut(), |v| v);
    let depth           = depth         .into().map_or(null_mut(), |v| v);
    let num_mip_levels  = num_mip_levels.into().map_or(null_mut(), |v| v);
    let format          = format        .into().map_or(null_mut(), |v| v);
    let usage           = usage         .into().into();
    let pool            = pool          .into().into();

    ErrorKind(D3DXCheckVolumeTextureRequirements(device, width, height, depth, num_mip_levels, usage, format.cast(), pool) as _)
}



#[link(name = "d3dx9")] extern "system" { // WINAPI
    fn D3DXCheckTextureRequirements(
        pDevice:        *mut IDirect3DDevice9,
        pWidth:         *mut UINT,
        pHeight:        *mut UINT,
        pNumMipLevels:  *mut UINT,
        Usage:          DWORD,
        pFormat:        *mut D3DFORMAT,
        Pool:           D3DPOOL,
    ) -> HRESULT;

    fn D3DXCheckCubeTextureRequirements(
        pDevice:        *mut IDirect3DDevice9,
        pSize:          *mut UINT,
        pNumMipLevels:  *mut UINT,
        Usage:          DWORD,
        pFormat:        *mut D3DFORMAT,
        Pool:           D3DPOOL,
    ) -> HRESULT;

    fn D3DXCheckVolumeTextureRequirements(
        pDevice:        *mut IDirect3DDevice9,
        pWidth:         *mut UINT,
        pHeight:        *mut UINT,
        pDepth:         *mut UINT,
        pNumMipLevels:  *mut UINT,
        Usage:          DWORD,
        pFormat:        *mut D3DFORMAT,
        Pool:           D3DPOOL,
    ) -> HRESULT;
}



//#cpp2rust D3DXCheckCubeTextureRequirements                = d3dx9::check_cube_texture_requirements
//#cpp2rust D3DXCheckTextureRequirements                    = d3dx9::check_texture_requirements
//#cpp2rust D3DXCheckVolumeTextureRequirements              = d3dx9::check_volume_texture_requirements
