#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



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
pub use PrimitiveType as PT;

enumish! { PT => D3DPRIMITIVETYPE; PointList, LineList, LineStrip, TriangleList, TriangleStrip, TriangleFan }

#[allow(non_upper_case_globals)] impl PrimitiveType { // These are enum-like
    /// Renders the vertices as a collection of isolated points. This value is unsupported for indexed primitives.
    pub const PointList         : PrimitiveType = PrimitiveType(D3DPT_POINTLIST); // 1

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

//#cpp2rust D3DPRIMITIVETYPE    = d3d::PrimitiveType
//#cpp2rust D3DPT_POINTLIST     = d3d::PT::PointList
//#cpp2rust D3DPT_LINELIST      = d3d::PT::LineList
//#cpp2rust D3DPT_LINESTRIP     = d3d::PT::LineStrip
//#cpp2rust D3DPT_TRIANGLELIST  = d3d::PT::TriangleList
//#cpp2rust D3DPT_TRIANGLESTRIP = d3d::PT::TriangleStrip
//#cpp2rust D3DPT_TRIANGLEFAN   = d3d::PT::TriangleFan
