use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterial9)\]
/// D3DMATERIAL9
///
/// Defines material properties.
///
/// ### See Also
///
/// *   [Device::set_material]
/// *   [Device::get_material]
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



/// # Materials
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial)\]
    /// IDirect3DDevice9::SetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let material = Material {
    ///     diffuse: ColorValue::default(),
    ///     .. Material::default()
    /// };
    /// device.set_material(material).unwrap();
    /// ```
    pub fn set_material(&self, material: impl Into<Material>) -> Result<(), MethodError> {
        let hr = unsafe { self.0.SetMaterial(&*material.into()) };
        MethodError::check("IDirect3DDevice9::SetMaterial", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial)\]
    /// IDirect3DDevice9::GetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok([Material])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let material = device.get_material().unwrap();
    /// ```
    pub fn get_material(&self) -> Result<Material, MethodError> {
        let mut material = Material::default();
        let hr = unsafe { self.0.GetMaterial(&mut *material) };
        MethodError::check("IDirect3DDevice9::GetMaterial", hr)?;
        Ok(material)
    }
}
