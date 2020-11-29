#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)\]
/// D3D_SHADER_INPUT_FLAGS / D3D_SIF_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderInputFlags(D3D_SHADER_INPUT_FLAGS);
#[doc(hidden)] pub use ShaderInputFlags as SIF;

enumish! {
    SIF => D3D_SHADER_INPUT_FLAGS;
}

#[allow(non_upper_case_globals)] impl SIF { // These are enum-like
    // TODO
}

#[doc(hidden)] impl SIF { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for SIF {
    fn default() -> Self { SIF(0) }
}
