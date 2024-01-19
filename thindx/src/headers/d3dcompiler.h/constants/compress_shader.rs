#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::minwindef::UINT;
const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS : UINT = 0x00000001; // not part of winapi 0.3.9



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders#parameters)\]
/// UINT / D3D_COMPRESS_SHADER_*
///
/// Flags controlling the behavior of [d3d::Compiler::compress_shaders]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct CompressShader(UINT);

flags! { CompressShader => UINT; None, KeepAllParts }

#[allow(non_upper_case_globals)] impl CompressShader { // These are enum-like
    #[doc=""]
    pub const None              : CompressShader = CompressShader(0);

    /// Don't strip sections when using [d3d::Compiler::compress_shaders]?
    pub const KeepAllParts      : CompressShader = CompressShader(D3D_COMPRESS_SHADER_KEEP_ALL_PARTS);
}



//#cpp2rust D3D_COMPRESS_SHADER_KEEP_ALL_PARTS = d3d::CompressShader::KeepAllParts
