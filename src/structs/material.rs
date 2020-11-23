use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterial9)\]
/// D3DMATERIAL9
///
/// Defines material properties.
#[derive(Clone, Copy, Debug)]
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

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse:    ColorValue::default(),
            ambient:    ColorValue::default(),
            specular:   ColorValue::default(),
            emissive:   ColorValue::default(),
            power:      0.0,
        }
    }
}

impl Deref    for Material { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DMATERIAL9; }
impl DerefMut for Material { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DMATERIAL9> for Material { fn from(value: D3DMATERIAL9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Material> for D3DMATERIAL9 { fn from(value: Material    ) -> Self { unsafe { std::mem::transmute(value) } } }

#[test] fn layout() {
    let thin = Material::default();
    let d3d  = unsafe { std::mem::zeroed::<D3DMATERIAL9>() };

    assert_eq!(std::mem::size_of_val (&thin), std::mem::size_of_val (&d3d));
    assert_eq!(std::mem::align_of_val(&thin), std::mem::align_of_val(&d3d));
    assert_eq!(offset(&thin, &thin.diffuse ), offset(&d3d, &d3d.Diffuse ));
    assert_eq!(offset(&thin, &thin.ambient ), offset(&d3d, &d3d.Ambient ));
    assert_eq!(offset(&thin, &thin.specular), offset(&d3d, &d3d.Specular));
    assert_eq!(offset(&thin, &thin.emissive), offset(&d3d, &d3d.Emissive));
    assert_eq!(offset(&thin, &thin.power   ), offset(&d3d, &d3d.Power   ));

    fn offset<S, F>(s: &S, f: &F) -> usize {
        let s : *const S = s;
        let f : *const F = f;
        (f as usize) - (s as usize)
    }
}
