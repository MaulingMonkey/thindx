use crate::*;

use winapi::shared::minwindef::LPVOID;
use winapi::um::d3d11shader::*;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_variable_desc)\]
/// D3D11_SHADER_VARIABLE_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy)]
#[repr(C)] pub struct ShaderVariableDesc<'s> {
    name:           Option<CStrPtr<'s>>,
    start_offset:   u32,
    size:           u32,
    flags:          u32,
    default_value:  LPVOID, // TODO: hmm... this is OKish for read, but not for write
    start_texture:  u32,
    texture_size:   u32,
    start_sampler:  u32,
    sampler_size:   u32,
}

impl Default for ShaderVariableDesc<'static> {
    fn default() -> Self {
        Self {
            name:           None,
            start_offset:   0,
            size:           0,
            flags:          0,
            default_value:  null_mut(),
            start_texture:  0,
            texture_size:   0,
            start_sampler:  0,
            sampler_size:   0,
        }
    }
}

test_layout! { ShaderVariableDesc => unsafe D3D11_SHADER_VARIABLE_DESC {
    name            => Name,
    start_offset    => StartOffset,
    size            => Size,
    flags           => uFlags,
    default_value   => DefaultValue,
    start_texture   => StartTexture,
    texture_size    => TextureSize,
    start_sampler   => StartSampler,
    sampler_size    => SamplerSize,
}}
