#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)\]
/// D3D_TESSELLATOR_OUTPUT_PRIMITIVE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorOutputPrimitive(D3D_TESSELLATOR_OUTPUT_PRIMITIVE);

enumish! {
    TessellatorOutputPrimitive => D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
}

#[allow(non_upper_case_globals)] impl TessellatorOutputPrimitive { // These are enum-like
    // TODO
}

#[doc(hidden)] impl TessellatorOutputPrimitive { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for TessellatorOutputPrimitive {
    fn default() -> Self { TessellatorOutputPrimitive(0) }
}
