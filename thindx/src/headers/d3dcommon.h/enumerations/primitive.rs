#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive)\]
/// D3D_PRIMITIVE
///
/// ### See Also
/// *   [d3d::PrimitiveTopology]
/// *   [Primitive Topologies](https://learn.microsoft.com/en-us/windows/win32/direct3d11/d3d10-graphics-programming-guide-primitive-topologies)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Primitive(D3D_PRIMITIVE);

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    Primitive => D3D_PRIMITIVE;
    default: Undefined == 0;
    Undefined, Point, Line, Triangle, LineAdj, TriangleAdj,
    _1ControlPointPatch, _2ControlPointPatch, _3ControlPointPatch, _4ControlPointPatch,
    _5ControlPointPatch, _6ControlPointPatch, _7ControlPointPatch, _8ControlPointPatch,
    _9ControlPointPatch, _10ControlPointPatch, _11ControlPointPatch, _12ControlPointPatch,
    _13ControlPointPatch, _14ControlPointPatch, _15ControlPointPatch, _16ControlPointPatch,
    _17ControlPointPatch, _18ControlPointPatch, _19ControlPointPatch, _20ControlPointPatch,
    _21ControlPointPatch, _22ControlPointPatch, _23ControlPointPatch, _24ControlPointPatch,
    _25ControlPointPatch, _26ControlPointPatch, _27ControlPointPatch, _28ControlPointPatch,
    _29ControlPointPatch, _30ControlPointPatch, _31ControlPointPatch, _32ControlPointPatch,
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl Primitive { // These are enum-like
    pub const Undefined                 : Primitive = Primitive(D3D_PRIMITIVE_UNDEFINED); // 0
    pub const Point                     : Primitive = Primitive(D3D_PRIMITIVE_POINT);
    pub const Line                      : Primitive = Primitive(D3D_PRIMITIVE_LINE);
    pub const Triangle                  : Primitive = Primitive(D3D_PRIMITIVE_TRIANGLE);
    pub const LineAdj                   : Primitive = Primitive(D3D_PRIMITIVE_LINE_ADJ);
    pub const TriangleAdj               : Primitive = Primitive(D3D_PRIMITIVE_TRIANGLE_ADJ);
    pub const _1ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_1_CONTROL_POINT_PATCH);
    pub const _2ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_2_CONTROL_POINT_PATCH);
    pub const _3ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_3_CONTROL_POINT_PATCH);
    pub const _4ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_4_CONTROL_POINT_PATCH);
    pub const _5ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_5_CONTROL_POINT_PATCH);
    pub const _6ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_6_CONTROL_POINT_PATCH);
    pub const _7ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_7_CONTROL_POINT_PATCH);
    pub const _8ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_8_CONTROL_POINT_PATCH);
    pub const _9ControlPointPatch       : Primitive = Primitive(D3D_PRIMITIVE_9_CONTROL_POINT_PATCH);
    pub const _10ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_10_CONTROL_POINT_PATCH);
    pub const _11ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_11_CONTROL_POINT_PATCH);
    pub const _12ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_12_CONTROL_POINT_PATCH);
    pub const _13ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_13_CONTROL_POINT_PATCH);
    pub const _14ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_14_CONTROL_POINT_PATCH);
    pub const _15ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_15_CONTROL_POINT_PATCH);
    pub const _16ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_16_CONTROL_POINT_PATCH);
    pub const _17ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_17_CONTROL_POINT_PATCH);
    pub const _18ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_18_CONTROL_POINT_PATCH);
    pub const _19ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_19_CONTROL_POINT_PATCH);
    pub const _20ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_20_CONTROL_POINT_PATCH);
    pub const _21ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_21_CONTROL_POINT_PATCH);
    pub const _22ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_22_CONTROL_POINT_PATCH);
    pub const _23ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_23_CONTROL_POINT_PATCH);
    pub const _24ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_24_CONTROL_POINT_PATCH);
    pub const _25ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_25_CONTROL_POINT_PATCH);
    pub const _26ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_26_CONTROL_POINT_PATCH);
    pub const _27ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_27_CONTROL_POINT_PATCH);
    pub const _28ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_28_CONTROL_POINT_PATCH);
    pub const _29ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_29_CONTROL_POINT_PATCH);
    pub const _30ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_30_CONTROL_POINT_PATCH);
    pub const _31ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_31_CONTROL_POINT_PATCH);
    pub const _32ControlPointPatch      : Primitive = Primitive(D3D_PRIMITIVE_32_CONTROL_POINT_PATCH);
}

//#cpp2rust D3D_PRIMITIVE                           = d3d::Primitive

//#cpp2rust D3D_PRIMITIVE_UNDEFINED                 = d3d::Primitive::Undefined
//#cpp2rust D3D_PRIMITIVE_POINT                     = d3d::Primitive::Point
//#cpp2rust D3D_PRIMITIVE_LINE                      = d3d::Primitive::Line
//#cpp2rust D3D_PRIMITIVE_TRIANGLE                  = d3d::Primitive::Triangle
//#cpp2rust D3D_PRIMITIVE_LINE_ADJ                  = d3d::Primitive::LineAdj
//#cpp2rust D3D_PRIMITIVE_TRIANGLE_ADJ              = d3d::Primitive::TriangleAdj
//#cpp2rust D3D_PRIMITIVE_1_CONTROL_POINT_PATCH     = d3d::Primitive::_1ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_2_CONTROL_POINT_PATCH     = d3d::Primitive::_2ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_3_CONTROL_POINT_PATCH     = d3d::Primitive::_3ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_4_CONTROL_POINT_PATCH     = d3d::Primitive::_4ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_5_CONTROL_POINT_PATCH     = d3d::Primitive::_5ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_6_CONTROL_POINT_PATCH     = d3d::Primitive::_6ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_7_CONTROL_POINT_PATCH     = d3d::Primitive::_7ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_8_CONTROL_POINT_PATCH     = d3d::Primitive::_8ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_9_CONTROL_POINT_PATCH     = d3d::Primitive::_9ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_10_CONTROL_POINT_PATCH    = d3d::Primitive::_10ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_11_CONTROL_POINT_PATCH    = d3d::Primitive::_11ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_12_CONTROL_POINT_PATCH    = d3d::Primitive::_12ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_13_CONTROL_POINT_PATCH    = d3d::Primitive::_13ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_14_CONTROL_POINT_PATCH    = d3d::Primitive::_14ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_15_CONTROL_POINT_PATCH    = d3d::Primitive::_15ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_16_CONTROL_POINT_PATCH    = d3d::Primitive::_16ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_17_CONTROL_POINT_PATCH    = d3d::Primitive::_17ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_18_CONTROL_POINT_PATCH    = d3d::Primitive::_18ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_19_CONTROL_POINT_PATCH    = d3d::Primitive::_19ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_20_CONTROL_POINT_PATCH    = d3d::Primitive::_20ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_21_CONTROL_POINT_PATCH    = d3d::Primitive::_21ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_22_CONTROL_POINT_PATCH    = d3d::Primitive::_22ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_23_CONTROL_POINT_PATCH    = d3d::Primitive::_23ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_24_CONTROL_POINT_PATCH    = d3d::Primitive::_24ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_25_CONTROL_POINT_PATCH    = d3d::Primitive::_25ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_26_CONTROL_POINT_PATCH    = d3d::Primitive::_26ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_27_CONTROL_POINT_PATCH    = d3d::Primitive::_27ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_28_CONTROL_POINT_PATCH    = d3d::Primitive::_28ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_29_CONTROL_POINT_PATCH    = d3d::Primitive::_29ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_30_CONTROL_POINT_PATCH    = d3d::Primitive::_30ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_31_CONTROL_POINT_PATCH    = d3d::Primitive::_31ControlPointPatch
//#cpp2rust D3D_PRIMITIVE_32_CONTROL_POINT_PATCH    = d3d::Primitive::_32ControlPointPatch

//#cpp2rust D3D10_PRIMITIVE_UNDEFINED               = d3d::Primitive::Undefined
//#cpp2rust D3D10_PRIMITIVE_POINT                   = d3d::Primitive::Point
//#cpp2rust D3D10_PRIMITIVE_LINE                    = d3d::Primitive::Line
//#cpp2rust D3D10_PRIMITIVE_TRIANGLE                = d3d::Primitive::Triangle
//#cpp2rust D3D10_PRIMITIVE_LINE_ADJ                = d3d::Primitive::LineAdj
//#cpp2rust D3D10_PRIMITIVE_TRIANGLE_ADJ            = d3d::Primitive::TriangleAdj

//#cpp2rust D3D11_PRIMITIVE_UNDEFINED               = d3d::Primitive::Undefined
//#cpp2rust D3D11_PRIMITIVE_POINT                   = d3d::Primitive::Point
//#cpp2rust D3D11_PRIMITIVE_LINE                    = d3d::Primitive::Line
//#cpp2rust D3D11_PRIMITIVE_TRIANGLE                = d3d::Primitive::Triangle
//#cpp2rust D3D11_PRIMITIVE_LINE_ADJ                = d3d::Primitive::LineAdj
//#cpp2rust D3D11_PRIMITIVE_TRIANGLE_ADJ            = d3d::Primitive::TriangleAdj
//#cpp2rust D3D11_PRIMITIVE_1_CONTROL_POINT_PATCH   = d3d::Primitive::_1ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_2_CONTROL_POINT_PATCH   = d3d::Primitive::_2ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_3_CONTROL_POINT_PATCH   = d3d::Primitive::_3ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_4_CONTROL_POINT_PATCH   = d3d::Primitive::_4ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_5_CONTROL_POINT_PATCH   = d3d::Primitive::_5ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_6_CONTROL_POINT_PATCH   = d3d::Primitive::_6ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_7_CONTROL_POINT_PATCH   = d3d::Primitive::_7ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_8_CONTROL_POINT_PATCH   = d3d::Primitive::_8ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_9_CONTROL_POINT_PATCH   = d3d::Primitive::_9ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_10_CONTROL_POINT_PATCH  = d3d::Primitive::_10ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_11_CONTROL_POINT_PATCH  = d3d::Primitive::_11ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_12_CONTROL_POINT_PATCH  = d3d::Primitive::_12ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_13_CONTROL_POINT_PATCH  = d3d::Primitive::_13ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_14_CONTROL_POINT_PATCH  = d3d::Primitive::_14ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_15_CONTROL_POINT_PATCH  = d3d::Primitive::_15ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_16_CONTROL_POINT_PATCH  = d3d::Primitive::_16ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_17_CONTROL_POINT_PATCH  = d3d::Primitive::_17ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_18_CONTROL_POINT_PATCH  = d3d::Primitive::_18ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_19_CONTROL_POINT_PATCH  = d3d::Primitive::_19ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_20_CONTROL_POINT_PATCH  = d3d::Primitive::_20ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_21_CONTROL_POINT_PATCH  = d3d::Primitive::_21ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_22_CONTROL_POINT_PATCH  = d3d::Primitive::_22ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_23_CONTROL_POINT_PATCH  = d3d::Primitive::_23ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_24_CONTROL_POINT_PATCH  = d3d::Primitive::_24ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_25_CONTROL_POINT_PATCH  = d3d::Primitive::_25ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_26_CONTROL_POINT_PATCH  = d3d::Primitive::_26ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_27_CONTROL_POINT_PATCH  = d3d::Primitive::_27ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_28_CONTROL_POINT_PATCH  = d3d::Primitive::_28ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_29_CONTROL_POINT_PATCH  = d3d::Primitive::_29ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_30_CONTROL_POINT_PATCH  = d3d::Primitive::_30ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_31_CONTROL_POINT_PATCH  = d3d::Primitive::_31ControlPointPatch
//#cpp2rust D3D11_PRIMITIVE_32_CONTROL_POINT_PATCH  = d3d::Primitive::_32ControlPointPatch
