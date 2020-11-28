#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;
const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS : UINT = 0x00000001; // not part of winapi 0.3.9



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants)\]
/// UINT / D3D_COMPRESS_SHADER_*
///
/// Flags controlling the behavior of [D3DCompiler::compress_shaders]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CompressShader(UINT);

flags! { CompressShader => UINT; None, KeepAllParts }

#[allow(non_upper_case_globals)] impl CompressShader { // These are enum-like
    pub const None              : CompressShader = CompressShader(0);
    pub const KeepAllParts      : CompressShader = CompressShader(D3D_COMPRESS_SHADER_KEEP_ALL_PARTS);
}

#[doc(hidden)] impl CompressShader { // Ctrl+C Ctrl+V support
    pub const NONE              : CompressShader = CompressShader(0);
    pub const KEEP_ALL_PARTS    : CompressShader = CompressShader(D3D_COMPRESS_SHADER_KEEP_ALL_PARTS);
}

impl Default for CompressShader {
    fn default() -> Self { CompressShader::None }
}
