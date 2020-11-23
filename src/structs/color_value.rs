use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolorvalue)\]
/// D3DCOLORVALUE
///
/// Describes color values.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct ColorValue(D3DCOLORVALUE);

impl ColorValue {
    pub fn red(&self) -> f32 { self.0.r }
    pub fn green(&self) -> f32 { self.0.g }
    pub fn blue(&self) -> f32 { self.0.b }
    pub fn alpha(&self) -> f32 { self.0.a }
}

impl Default for ColorValue {
    fn default() -> Self { Self(D3DCOLORVALUE { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }) }
}

impl Deref for ColorValue {
    type Target = D3DCOLORVALUE;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ColorValue {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Debug for ColorValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ColorValue")
            .field("r", &self.0.r)
            .field("g", &self.0.g)
            .field("b", &self.0.b)
            .field("a", &self.0.a)
            .finish()
    }
}

impl From<D3DCOLORVALUE> for ColorValue {
    fn from(value: D3DCOLORVALUE) -> Self { Self(value) }
}

impl From<ColorValue> for D3DCOLORVALUE {
    fn from(value: ColorValue) -> Self { value.0 }
}
