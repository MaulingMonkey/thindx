use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolorvalue)\]
/// D3DCOLORVALUE
///
/// Describes color values.
///
/// You can set the members of this structure to values outside the range of 0 through 1 to implement some unusual effects.
/// Values greater than 1 produce strong lights that tend to wash out a scene.
/// Negative values produce dark lights that actually remove light from a scene.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ColorValue {
    /// Floating-point value that specifies the <span style="color: red">red</span> component of a color.
    /// This value generally is in the range from 0.0 through 1.0.
    /// A value of 0.0 indicates the complete absence of the red component, while a value of 1.0 indicates that red is fully present.
    pub r: f32,

    /// Floating-point value that specifies the <span style="color: green">green</span> component of a color.
    /// This value generally is in the range from 0.0 through 1.0.
    /// A value of 0.0 indicates the complete absence of the green component, while a value of 1.0 indicates that green is fully present.
    pub g: f32,

    /// Floating-point value that specifies the <span style="color: blue">blue</span> component of a color.
    /// This value generally is in the range from 0.0 through 1.0.
    /// A value of 0.0 indicates the complete absence of the blue component, while a value of 1.0 indicates that blue is fully present.
    pub b: f32,

    /// Floating-point value that specifies the alpha component of a color.
    /// This value generally is in the range from 0.0 through 1.0.
    /// A value of 0.0 indicates fully transparent, while a value of 1.0 indicates fully opaque.
    pub a: f32,
}

impl ColorValue {
    pub fn red(&self) -> f32 { self.r }
    pub fn green(&self) -> f32 { self.g }
    pub fn blue(&self) -> f32 { self.b }
    pub fn alpha(&self) -> f32 { self.a }
}

impl Deref    for ColorValue { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DCOLORVALUE; }
impl DerefMut for ColorValue { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DCOLORVALUE> for ColorValue { fn from(value: D3DCOLORVALUE) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<ColorValue> for D3DCOLORVALUE { fn from(value: ColorValue   ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { ColorValue => D3DCOLORVALUE { r => r, g => g, b => b, a => a } }
