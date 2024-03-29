use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dcolorvalue)\]
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

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    ColorValue => D3DCOLORVALUE { r => r, g => g, b => b, a => a }
}

//#cpp2rust D3DCOLORVALUE = d3d::ColorValue
