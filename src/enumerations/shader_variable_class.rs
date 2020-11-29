#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)\]
/// D3D_SHADER_VARIABLE_CLASS / D3D_SVC_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVariableClass(D3D_SHADER_VARIABLE_CLASS);
#[doc(hidden)] pub use ShaderVariableClass as SVC;

enumish! {
    SVC => D3D_SHADER_VARIABLE_CLASS;
}

#[allow(non_upper_case_globals)] impl SVC { // These are enum-like
    // TODO
}

#[doc(hidden)] impl SVC { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for SVC {
    fn default() -> Self { SVC(0) }
}
