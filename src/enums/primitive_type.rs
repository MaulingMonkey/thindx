#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype)\]
/// D3DPRIMITIVETYPE
///
/// Defines the primitives supported by Direct3D.
///
/// Using [Triangle Strips] or [Triangle Fans (Direct3D 9)] is often more efficient than using triangle lists because fewer vertices are duplicated.
///
/// [Triangle Strips]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/triangle-strips
/// [Triangle Fans (Direct3D 9)]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/triangle-fans
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PrimitiveType(D3DPRIMITIVETYPE);

impl PrimitiveType {
    /// Convert a raw [D3DPRIMITIVETYPE] value into a [PrimitiveType].  This is *probably* safe... probably....
    ///
    /// [D3DPRIMITIVETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype
    pub const fn from_unchecked(primitivetype: D3DPRIMITIVETYPE) -> Self { Self(primitivetype) }

    /// Convert a [PrimitiveType] into a raw [D3DPRIMITIVETYPE].
    ///
    /// [D3DPRIMITIVETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype
    pub const fn into(self) -> D3DPRIMITIVETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl PrimitiveType {
    /// Renders the vertices as a collection of isolated points. This value is unsupported for indexed primitives.
    pub const PointList         : PrimitiveType = PrimitiveType(D3DPT_POINTLIST);

    /// Renders the vertices as a list of isolated straight line segments.
    pub const LineList          : PrimitiveType = PrimitiveType(D3DPT_LINELIST);

    /// Renders the vertices as a single polyline.
    pub const LineStrip         : PrimitiveType = PrimitiveType(D3DPT_LINESTRIP);

    /// Renders the specified vertices as a sequence of isolated triangles. Each group of three vertices defines a separate triangle.
    ///
    /// Back-face culling is affected by the current winding-order render state.
    pub const TriangleList      : PrimitiveType = PrimitiveType(D3DPT_TRIANGLELIST);

    /// Renders the vertices as a triangle strip. The backface-culling flag is automatically flipped on even-numbered triangles.
    pub const TriangleStrip     : PrimitiveType = PrimitiveType(D3DPT_TRIANGLESTRIP);

    /// Renders the vertices as a triangle fan.
    pub const TriangleFan       : PrimitiveType = PrimitiveType(D3DPT_TRIANGLEFAN);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for PrimitiveType {
    fn default() -> Self { PrimitiveType(0) }
}

impl Debug for PrimitiveType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PrimitiveType::PointList        => write!(f, "PrimitiveType::PointList"),
            PrimitiveType::LineList         => write!(f, "PrimitiveType::LineList"),
            PrimitiveType::LineStrip        => write!(f, "PrimitiveType::LineStrip"),
            PrimitiveType::TriangleList     => write!(f, "PrimitiveType::TriangleList"),
            PrimitiveType::TriangleStrip    => write!(f, "PrimitiveType::TriangleStrip"),
            PrimitiveType::TriangleFan      => write!(f, "PrimitiveType::TriangleFan"),
            other                           => write!(f, "PrimitiveType({})", other.0),
        }
    }
}

impl From<PrimitiveType> for D3DPRIMITIVETYPE {
    fn from(value: PrimitiveType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DPRIMITIVETYPE> for PrimitiveType {
    fn from(value: D3DPRIMITIVETYPE) -> Self { Self(value) }
}
