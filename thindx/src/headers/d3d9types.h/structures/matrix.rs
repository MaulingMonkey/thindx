use bytemuck::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmatrix)\]
/// D3DMATRIX
///
/// Defines a 4x4-component [f32] matrix.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

impl Deref    for Matrix { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DMATRIX; }
impl DerefMut for Matrix { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DMATRIX> for Matrix { fn from(value: D3DMATRIX) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Matrix> for D3DMATRIX { fn from(value: Matrix   ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { Matrix => D3DMATRIX { m => m } }

impl Matrix {
    pub const fn identity() -> Self {
        Self { m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]}
    }
}

//#cpp2rust D3DMATRIX = d3d::Matrix
