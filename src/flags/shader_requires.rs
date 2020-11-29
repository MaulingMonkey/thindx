#[allow(unused_imports)] use crate::*;

use winapi::shared::basetsd::UINT64;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getrequiresflags#return-value)\]
/// UINT64 / D3D_SHADER_REQUIRES_\*
///
/// Flags controlling how [D3DCompiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderRequires(UINT64);

flags! { ShaderRequires => UINT64; None }

#[allow(non_upper_case_globals)] impl ShaderRequires { // These are enum-like
    const None          : ShaderRequires = ShaderRequires(0);
    // TODO
}

#[doc(hidden)] impl ShaderRequires { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for ShaderRequires {
    fn default() -> Self { ShaderRequires::None }
}
