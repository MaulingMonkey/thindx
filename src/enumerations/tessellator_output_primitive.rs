#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)\]
/// D3D_TESSELLATOR_OUTPUT_PRIMITIVE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_PRIMITIVE);

// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { TessellatorOutputPrimitive => D3D_TESSELLATOR_OUTPUT_PRIMITIVE; Undefined, Point, Line, TriangleCW, TriangleCCW }

#[allow(non_upper_case_globals)] impl TessellatorOutputPrimitive { // These are enum-like
    pub const Undefined     : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_UNDEFINED);
    pub const Point         : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_POINT);
    pub const Line          : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_LINE);
    pub const TriangleCW    : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW);
    pub const TriangleCCW   : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW);
}

#[doc(hidden)] impl TessellatorOutputPrimitive { // Ctrl+C Ctrl+V support
    pub const UNDEFINED     : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_UNDEFINED);
    pub const POINT         : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_POINT);
    pub const LINE          : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_LINE);
    pub const TRIANGLE_CW   : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW);
    pub const TRIANGLE_CCW  : TessellatorOutputPrimitive = TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW);
}

impl Default for TessellatorOutputPrimitive {
    fn default() -> Self { TessellatorOutputPrimitive(0) }
}
