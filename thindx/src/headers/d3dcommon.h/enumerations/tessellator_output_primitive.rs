#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)\]
/// D3D_TESSELLATOR_OUTPUT_PRIMITIVE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_PRIMITIVE);
#[doc(hidden)] pub use TessellatorOutputPrimitive as TessellatorOutput;

// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { TessellatorOutput => D3D_TESSELLATOR_OUTPUT_PRIMITIVE; default: Undefined == 0; Undefined, Point, Line, TriangleCW, TriangleCCW }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl TessellatorOutput { // These are enum-like
    pub const Undefined     : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_UNDEFINED); // 0
    pub const Point         : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_POINT);
    pub const Line          : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_LINE);
    pub const TriangleCW    : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW);
    pub const TriangleCCW   : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW);
}

//#cpp2rust D3D_TESSELLATOR_OUTPUT_PRIMITIVE        = d3d::TessellatorOutputPrimitive

//#cpp2rust D3D_TESSELLATOR_OUTPUT_UNDEFINED        = d3d::TessellatorOutput::Undefined
//#cpp2rust D3D_TESSELLATOR_OUTPUT_POINT            = d3d::TessellatorOutput::Point
//#cpp2rust D3D_TESSELLATOR_OUTPUT_LINE             = d3d::TessellatorOutput::Line
//#cpp2rust D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW      = d3d::TessellatorOutput::TriangleCW
//#cpp2rust D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW     = d3d::TessellatorOutput::TriangleCCW

//#cpp2rust D3D11_TESSELLATOR_OUTPUT_UNDEFINED      = d3d::TessellatorOutput::Undefined
//#cpp2rust D3D11_TESSELLATOR_OUTPUT_POINT          = d3d::TessellatorOutput::Point
//#cpp2rust D3D11_TESSELLATOR_OUTPUT_LINE           = d3d::TessellatorOutput::Line
//#cpp2rust D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CW    = d3d::TessellatorOutput::TriangleCW
//#cpp2rust D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CCW   = d3d::TessellatorOutput::TriangleCCW
