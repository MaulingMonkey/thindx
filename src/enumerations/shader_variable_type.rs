#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_type)\]
/// D3D_SHADER_VARIABLE_TYPE / D3D_SVT_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVariableType(D3D_SHADER_VARIABLE_TYPE);
#[doc(hidden)] pub use ShaderVariableType as SVT;

enumish! {
    SVT => D3D_SHADER_VARIABLE_TYPE;
}

#[allow(non_upper_case_globals)] impl SVT { // These are enum-like
    // TODO
}

#[doc(hidden)] impl SVT { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for SVT {
    fn default() -> Self { SVT(0) }
}
