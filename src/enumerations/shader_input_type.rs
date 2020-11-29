#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)\]
/// D3D_SHADER_INPUT_TYPE / D3D_SIT_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderInputType(D3D_SHADER_INPUT_TYPE);
#[doc(hidden)] pub use ShaderInputType as SIT;

enumish! {
    SIT => D3D_SHADER_INPUT_TYPE;
}

#[allow(non_upper_case_globals)] impl SIT { // These are enum-like
    // TODO
}

#[doc(hidden)] impl SIT { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for SIT {
    fn default() -> Self { SIT(0) }
}
