#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)\]
/// UINT / D3D_SHADER_VARIABLE_FLAGS_\*
///
/// Flags controlling how [D3DCompiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderVariableFlags(UINT);

flags! { ShaderVariableFlags => UINT; None }

#[allow(non_upper_case_globals)] impl ShaderVariableFlags { // These are enum-like
    pub const None          : ShaderVariableFlags = ShaderVariableFlags(0);
    // TODO
}

#[doc(hidden)] impl ShaderVariableFlags { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for ShaderVariableFlags {
    fn default() -> Self { ShaderVariableFlags::None }
}
