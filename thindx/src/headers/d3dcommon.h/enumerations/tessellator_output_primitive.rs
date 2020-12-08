#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)\]
/// D3D_TESSELLATOR_OUTPUT_PRIMITIVE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_PRIMITIVE);
#[doc(hidden)] pub use TessellatorOutputPrimitive as TessellatorOutput;

// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! { TessellatorOutput => D3D_TESSELLATOR_OUTPUT_PRIMITIVE; Undefined, Point, Line, TriangleCW, TriangleCCW }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl TessellatorOutput { // These are enum-like
    pub const Undefined     : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_UNDEFINED);
    pub const Point         : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_POINT);
    pub const Line          : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_LINE);
    pub const TriangleCW    : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW);
    pub const TriangleCCW   : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW);
}

#[doc(hidden)] impl TessellatorOutput { // Ctrl+C Ctrl+V support
    pub const UNDEFINED     : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_UNDEFINED);
    pub const POINT         : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_POINT);
    pub const LINE          : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_LINE);
    pub const TRIANGLE_CW   : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW);
    pub const TRIANGLE_CCW  : TessellatorOutput = TessellatorOutput(D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW);
}

impl Default for TessellatorOutput {
    fn default() -> Self { TessellatorOutput(0) }
}
