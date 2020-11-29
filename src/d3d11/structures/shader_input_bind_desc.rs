use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_input_bind_desc)\]
/// D3D11_SHADER_INPUT_BIND_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ShaderDesc<'s> {
    name:           Option<CStrPtr<'s>>,
    r#type:         ShaderInputType,
    bind_point:     u32,
    bind_count:     u32,
    flags:          u32,
    return_type:    ResourceReturnType,
    dimension:      SrvDimension,
    num_samples:    u32,
}

test_layout! { ShaderDesc => unsafe D3D11_SHADER_INPUT_BIND_DESC {
    name            => Name,
    r#type          => Type,
    bind_point      => BindPoint,
    bind_count      => BindCount,
    flags           => uFlags,
    return_type     => ReturnType,
    dimension       => Dimension,
    num_samples     => NumSamples,
}}
