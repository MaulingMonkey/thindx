#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)\]
/// UINT / D3D_SHADER_CBUFFER_FLAGS_\*
///
/// Flags controlling how [D3DCompiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderCbufferFlags(UINT);

flags! { ShaderCbufferFlags => UINT; None }

#[allow(non_upper_case_globals)] impl ShaderCbufferFlags { // These are enum-like
    const None          : ShaderCbufferFlags = ShaderCbufferFlags(0);
    // TODO
}

#[doc(hidden)] impl ShaderCbufferFlags { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for ShaderCbufferFlags {
    fn default() -> Self { ShaderCbufferFlags::None }
}
