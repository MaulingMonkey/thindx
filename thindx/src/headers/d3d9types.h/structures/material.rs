use crate::d3d9::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterial9)\]
/// D3DMATERIAL9
///
/// Defines material properties.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::set_material]
/// *   [IDirect3DDevice9Ext::get_material]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Material {
    /// [Diffuse](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/diffuse-lighting) lighting
    pub diffuse:    ColorValue,

    /// [Ambient](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/ambient-lighting) lighting
    pub ambient:    ColorValue,

    /// [Specular](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/specular-lighting) highlighting
    pub specular:   ColorValue,

    /// [Emissive](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/emissive-lighting) glow
    pub emissive:   ColorValue,

    /// Floating-point value specifying the sharpness of specular highlights. The higher the value, the sharper the highlight.
    pub power:      f32,
}

impl Deref    for Material { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DMATERIAL9; }
impl DerefMut for Material { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DMATERIAL9> for Material { fn from(value: D3DMATERIAL9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Material> for D3DMATERIAL9 { fn from(value: Material    ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { Material => unsafe D3DMATERIAL9 { diffuse => Diffuse, ambient => Ambient, specular => Specular, emissive => Emissive, power => Power } }
