use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmatrix)\]
/// D3DMATRIX
///
/// Defines a 4x4-component [f32] matrix.
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Matrix => D3DMATRIX { m => m }
}

impl Matrix {
    /// A matrix filled with `0`s.  A poor default: consider [`Matrix::identity`] instead.
    pub const fn zeroed() -> Self {
        Self { m: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]}
    }

    /// A matrix filled with `1` on the diagonal, `0` otherwise.
    pub const fn identity() -> Self {
        Self { m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]}
    }
}

// TODO: matrix indexing ops?
// TODO: matrix math ops ala d3dvec.inl?

//#cpp2rust D3DMATRIX = d3d::Matrix
