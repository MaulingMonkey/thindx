#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces)\]
/// D3DCUBEMAP_FACES
///
/// Defines the face of a cubemap.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CubeMapFace(D3DCUBEMAP_FACES);
pub use CubeMapFace as CubeMapFaces;

enumish! { CubeMapFace => D3DCUBEMAP_FACES; PositiveX, NegativeX, PositiveY, NegativeY, PositiveZ, NegativeZ }

#[allow(non_upper_case_globals)] impl CubeMapFace { // These are enum-like
    pub const PositiveX     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_X); // 0
    pub const NegativeX     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_X); // 1
    pub const PositiveY     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_Y); // 2
    pub const NegativeY     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_Y);
    pub const PositiveZ     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_Z);
    pub const NegativeZ     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_Z);
}

impl Default for CubeMapFace {
    fn default() -> Self { CubeMapFace::PositiveX } // 0
}
