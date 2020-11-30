use crate::*;
use crate::core::AbiCStr;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_input_bind_desc)\]
/// D3D11_SHADER_INPUT_BIND_DESC
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ShaderDesc<'s> {
    name:           Option<&'s AbiCStr>,
    r#type:         ShaderInputType,
    bind_point:     u32,
    bind_count:     u32,
    flags:          ShaderInputFlags,
    return_type:    ResourceReturnType,
    dimension:      SrvDimension,
    num_samples:    u32,
}

test_layout! { ShaderDesc => unsafe winapi::um::d3d11shader::D3D11_SHADER_INPUT_BIND_DESC {
    name            => Name,
    r#type          => Type,
    bind_point      => BindPoint,
    bind_count      => BindCount,
    flags           => uFlags,
    return_type     => ReturnType,
    dimension       => Dimension,
    num_samples     => NumSamples,
}}
