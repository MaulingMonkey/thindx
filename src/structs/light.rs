use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlight9)\]
/// D3DLIGHT9
///
/// Defines a set of lighting properties.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Light(D3DLIGHT9);

impl Light {
    pub fn light_type(&self)    -> LightType { LightType::from_unchecked(self.0.Type) }
    pub fn diffuse(&self)       -> ColorValue { ColorValue::from(self.0.Diffuse) }
    pub fn specular(&self)      -> ColorValue { ColorValue::from(self.0.Specular) }
    pub fn ambient(&self)       -> ColorValue { ColorValue::from(self.0.Ambient) }
    pub fn position(&self)      -> Vector { Vector::from(self.0.Position) }
    pub fn direction(&self)     -> Vector { Vector::from(self.0.Direction) }
}

impl Default for Light {
    fn default() -> Self {
        Self(D3DLIGHT9 {
            // reduce usage of the danger keyword by manually initializing
            Type:           0,
            Diffuse:        D3DCOLORVALUE { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            Specular:       D3DCOLORVALUE { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            Ambient:        D3DCOLORVALUE { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            Position:       D3DVECTOR { x: 0.0, y: 0.0, z: 0.0 },
            Direction:      D3DVECTOR { x: 0.0, y: 0.0, z: 0.0 },
            Range:          0.0,
            Falloff:        0.0,
            Attenuation0:   0.0,
            Attenuation1:   0.0,
            Attenuation2:   0.0,
            Theta:          0.0,
            Phi:            0.0,
        })
    }
}

impl Deref for Light {
    type Target = D3DLIGHT9;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Light {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Debug for Light {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Light")
            .field("Type",          &self.light_type())
            .field("Diffuse",       &self.diffuse())
            .field("Specular",      &self.specular())
            .field("Ambient",       &self.ambient())
            .field("Position",      &self.position())
            .field("Direction",     &self.direction())
            .field("Range",         &self.0.Range)
            .field("Falloff",       &self.0.Falloff)
            .field("Attenuation0",  &self.0.Attenuation0)
            .field("Attenuation1",  &self.0.Attenuation1)
            .field("Attenuation2",  &self.0.Attenuation2)
            .field("Theta",         &self.0.Theta)
            .field("Phi",           &self.0.Phi)
            .finish()
    }
}

impl From<D3DLIGHT9> for Light {
    fn from(value: D3DLIGHT9) -> Self { Self(value) }
}

impl From<Light> for D3DLIGHT9 {
    fn from(value: Light) -> Self { value.0 }
}
