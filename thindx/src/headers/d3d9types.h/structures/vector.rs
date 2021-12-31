use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvector)\]
/// D3DVECTOR
///
/// Defines a 3-component [f32] vector.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Deref    for Vector { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DVECTOR; }
impl DerefMut for Vector { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DVECTOR> for Vector { fn from(value: D3DVECTOR) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Vector> for D3DVECTOR { fn from(value: Vector   ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { Vector => unsafe D3DVECTOR { x => x, y => y, z => z } }
