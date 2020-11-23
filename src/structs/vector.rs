use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvector)\]
/// D3DVECTOR
///
/// Defines a 3-component [f32] vector.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Vector(D3DVECTOR);

impl Default for Vector {
    fn default() -> Self {
        Self(D3DVECTOR {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })
    }
}

impl Deref for Vector {
    type Target = D3DVECTOR;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Vector")
            .field("x", &self.0.x)
            .field("y", &self.0.y)
            .field("z", &self.0.z)
            .finish()
    }
}

impl From<D3DVECTOR> for Vector {
    fn from(value: D3DVECTOR) -> Self { Self(value) }
}

impl From<Vector> for D3DVECTOR {
    fn from(value: Vector) -> Self { value.0 }
}
