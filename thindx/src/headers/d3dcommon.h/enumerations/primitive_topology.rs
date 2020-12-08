#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)\]
/// D3D_PRIMITIVE_TOPOLOGY
///
/// ### See Also
/// *   [Primitive Topologies](https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d10-graphics-programming-guide-primitive-topologies)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY);

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    PrimitiveTopology => D3D_PRIMITIVE_TOPOLOGY;
    Undefined, PointList, LineList, LineStrip, TriangleList, TriangleStrip, LineListAdj, LineStripAdj, TriangleListAdj, TriangleStripAdj,
    _1ControlPointPatchList, _2ControlPointPatchList, _3ControlPointPatchList, _4ControlPointPatchList,
    _5ControlPointPatchList, _6ControlPointPatchList, _7ControlPointPatchList, _8ControlPointPatchList,
    _9ControlPointPatchList, _10ControlPointPatchList, _11ControlPointPatchList, _12ControlPointPatchList,
    _13ControlPointPatchList, _14ControlPointPatchList, _15ControlPointPatchList, _16ControlPointPatchList,
    _17ControlPointPatchList, _18ControlPointPatchList, _19ControlPointPatchList, _20ControlPointPatchList,
    _21ControlPointPatchList, _22ControlPointPatchList, _23ControlPointPatchList, _24ControlPointPatchList,
    _25ControlPointPatchList, _26ControlPointPatchList, _27ControlPointPatchList, _28ControlPointPatchList,
    _29ControlPointPatchList, _30ControlPointPatchList, _31ControlPointPatchList, _32ControlPointPatchList,
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl PrimitiveTopology { // These are enum-like
    pub const Undefined                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_UNDEFINED);
    pub const PointList                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_POINTLIST);
    pub const LineList                      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST);
    pub const LineStrip                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINESTRIP);
    pub const TriangleList                  : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    pub const TriangleStrip                 : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
    pub const LineListAdj                   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ);
    pub const LineStripAdj                  : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ);
    pub const TriangleListAdj               : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ);
    pub const TriangleStripAdj              : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ);
    pub const _1ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST);
    pub const _2ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST);
    pub const _3ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST);
    pub const _4ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST);
    pub const _5ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST);
    pub const _6ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST);
    pub const _7ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST);
    pub const _8ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST);
    pub const _9ControlPointPatchList       : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST);
    pub const _10ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST);
    pub const _11ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST);
    pub const _12ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST);
    pub const _13ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST);
    pub const _14ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST);
    pub const _15ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST);
    pub const _16ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST);
    pub const _17ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST);
    pub const _18ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST);
    pub const _19ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST);
    pub const _20ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST);
    pub const _21ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST);
    pub const _22ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST);
    pub const _23ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST);
    pub const _24ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST);
    pub const _25ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST);
    pub const _26ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST);
    pub const _27ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST);
    pub const _28ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST);
    pub const _29ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST);
    pub const _30ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST);
    pub const _31ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST);
    pub const _32ControlPointPatchList      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST);
}

#[doc(hidden)] impl PrimitiveTopology { // Ctrl+C Ctrl+V support
    pub const UNDEFINED                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_UNDEFINED);
    pub const POINTLIST                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_POINTLIST);
    pub const LINELIST                      : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST);
    pub const LINESTRIP                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINESTRIP);
    pub const TRIANGLELIST                  : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    pub const TRIANGLESTRIP                 : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
    pub const LINELIST_ADJ                  : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ);
    pub const LINESTRIP_ADJ                 : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ);
    pub const TRIANGLELIST_ADJ              : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ);
    pub const TRIANGLESTRIP_ADJ             : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ);
    pub const _1_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST);
    pub const _2_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST);
    pub const _3_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST);
    pub const _4_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST);
    pub const _5_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST);
    pub const _6_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST);
    pub const _7_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST);
    pub const _8_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST);
    pub const _9_CONTROL_POINT_PATCHLIST    : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST);
    pub const _10_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST);
    pub const _11_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST);
    pub const _12_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST);
    pub const _13_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST);
    pub const _14_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST);
    pub const _15_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST);
    pub const _16_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST);
    pub const _17_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST);
    pub const _18_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST);
    pub const _19_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST);
    pub const _20_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST);
    pub const _21_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST);
    pub const _22_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST);
    pub const _23_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST);
    pub const _24_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST);
    pub const _25_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST);
    pub const _26_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST);
    pub const _27_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST);
    pub const _28_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST);
    pub const _29_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST);
    pub const _30_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST);
    pub const _31_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST);
    pub const _32_CONTROL_POINT_PATCHLIST   : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST);
}

impl Default for PrimitiveTopology {
    fn default() -> Self { PrimitiveTopology(0) }
}
