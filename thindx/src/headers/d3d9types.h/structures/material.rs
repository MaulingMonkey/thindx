use crate::d3d9::*;

use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterial9)\]
/// D3DMATERIAL9
///
/// Defines material properties.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::set_material]
/// *   [IDirect3DDevice9Ext::get_material]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Material {
    /// [Diffuse](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/diffuse-lighting) lighting
    pub diffuse:    ColorValue,

    /// [Ambient](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/ambient-lighting) lighting
    pub ambient:    ColorValue,

    /// [Specular](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/specular-lighting) highlighting
    pub specular:   ColorValue,

    /// [Emissive](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/emissive-lighting) glow
    pub emissive:   ColorValue,

    /// Floating-point value specifying the sharpness of specular highlights. The higher the value, the sharper the highlight.
    pub power:      f32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Material => D3DMATERIAL9 {
        diffuse     => Diffuse,
        ambient     => Ambient,
        specular    => Specular,
        emissive    => Emissive,
        power       => Power
    }
}

//#cpp2rust D3DMATERIAL9 = d3d9::Material
