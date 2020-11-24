#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces)\]
/// D3DCUBEMAP_FACES
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CubeMapFace(D3DCUBEMAP_FACES);

impl CubeMapFace {
    /// Convert a raw [D3DCUBEMAP_FACES] value into a [CubeMapFace].  This is *probably* safe... probably....
    ///
    /// [D3DCUBEMAP_FACES]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces
    pub const fn from_unchecked(cubemapface: D3DCUBEMAP_FACES) -> Self { Self(cubemapface) }

    /// Convert a [CubeMapFace] into a raw [D3DCUBEMAP_FACES].
    ///
    /// [D3DCUBEMAP_FACES]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces
    pub const fn into(self) -> D3DCUBEMAP_FACES { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl CubeMapFace {
    pub const PositiveX     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_X);
    pub const NegativeX     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_X);
    pub const PositiveY     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_Y);
    pub const NegativeY     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_Y);
    pub const PositiveZ     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_POSITIVE_Z);
    pub const NegativeZ     : CubeMapFace = CubeMapFace(D3DCUBEMAP_FACE_NEGATIVE_Z);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for CubeMapFace {
    fn default() -> Self { CubeMapFace::PositiveX }
}

impl Debug for CubeMapFace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CubeMapFace::PositiveX  => write!(f, "CubeMapFace::PositiveX"),
            CubeMapFace::NegativeX  => write!(f, "CubeMapFace::NegativeX"),
            CubeMapFace::PositiveY  => write!(f, "CubeMapFace::PositiveY"),
            CubeMapFace::NegativeY  => write!(f, "CubeMapFace::NegativeY"),
            CubeMapFace::PositiveZ  => write!(f, "CubeMapFace::PositiveZ"),
            CubeMapFace::NegativeZ  => write!(f, "CubeMapFace::NegativeZ"),
            other                   => write!(f, "CubeMapFace({})", other.0),
        }
    }
}

impl From<CubeMapFace> for D3DCUBEMAP_FACES {
    fn from(value: CubeMapFace) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCUBEMAP_FACES> for CubeMapFace {
    fn from(value: D3DCUBEMAP_FACES) -> Self { Self(value) }
}
