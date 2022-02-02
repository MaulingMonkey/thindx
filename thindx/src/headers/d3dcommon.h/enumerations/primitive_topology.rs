#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)\]
/// D3D_PRIMITIVE_TOPOLOGY
///
/// ### See Also
/// *   [Primitive Topologies](https://docs.microsoft.com/en-us/windows/win32/direct3d11/d3d10-graphics-programming-guide-primitive-topologies)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY);

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    PrimitiveTopology => D3D_PRIMITIVE_TOPOLOGY;
    default: Undefined == 0;
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
    pub const Undefined                     : PrimitiveTopology = PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_UNDEFINED); // 0
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

//#cpp2rust D3D_PRIMITIVE_TOPOLOGY                              = d3d::PrimitiveTopology

//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_UNDEFINED                    = d3d::PrimitiveTopology::Undefined
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_POINTLIST                    = d3d::PrimitiveTopology::PointList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_LINELIST                     = d3d::PrimitiveTopology::LineList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_LINESTRIP                    = d3d::PrimitiveTopology::LineStrip
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST                 = d3d::PrimitiveTopology::TriangleList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP                = d3d::PrimitiveTopology::TriangleStrip
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ                 = d3d::PrimitiveTopology::LineListAdj
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ                = d3d::PrimitiveTopology::LineStripAdj
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ             = d3d::PrimitiveTopology::TriangleListAdj
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ            = d3d::PrimitiveTopology::TriangleStripAdj
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_1ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_2ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_3ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_4ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_5ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_6ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_7ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_8ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST    = d3d::PrimitiveTopology::_9ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_10ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_11ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_12ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_13ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_14ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_15ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_16ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_17ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_18ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_19ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_20ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_21ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_22ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_23ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_24ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_25ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_26ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_27ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_28ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_29ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_30ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_31ControlPointPatchList
//#cpp2rust D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST   = d3d::PrimitiveTopology::_32ControlPointPatchList

//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_UNDEFINED                  = d3d::PrimitiveTopology::Undefined
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_POINTLIST                  = d3d::PrimitiveTopology::PointList
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_LINELIST                   = d3d::PrimitiveTopology::LineList
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP                  = d3d::PrimitiveTopology::LineStrip
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST               = d3d::PrimitiveTopology::TriangleList
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP              = d3d::PrimitiveTopology::TriangleStrip
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_LINELIST_ADJ               = d3d::PrimitiveTopology::LineListAdj
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ              = d3d::PrimitiveTopology::LineStripAdj
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ           = d3d::PrimitiveTopology::TriangleListAdj
//#cpp2rust D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ          = d3d::PrimitiveTopology::TriangleStripAdj

//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_UNDEFINED                  = d3d::PrimitiveTopology::Undefined
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_POINTLIST                  = d3d::PrimitiveTopology::PointList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_LINELIST                   = d3d::PrimitiveTopology::LineList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP                  = d3d::PrimitiveTopology::LineStrip
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST               = d3d::PrimitiveTopology::TriangleList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP              = d3d::PrimitiveTopology::TriangleStrip
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_LINELIST_ADJ               = d3d::PrimitiveTopology::LineListAdj
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ              = d3d::PrimitiveTopology::LineStripAdj
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ           = d3d::PrimitiveTopology::TriangleListAdj
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ          = d3d::PrimitiveTopology::TriangleStripAdj
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_1ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_2ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_3ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_4ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_5ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_6ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_7ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_8ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST  = d3d::PrimitiveTopology::_9ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_10ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_11ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_12ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_13ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_14ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_15ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_16ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_17ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_18ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_19ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_20ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_21ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_22ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_23ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_24ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_25ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_26ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_27ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_28ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_29ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_30ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_31ControlPointPatchList
//#cpp2rust D3D11_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST = d3d::PrimitiveTopology::_32ControlPointPatchList
