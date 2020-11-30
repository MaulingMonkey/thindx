use crate::*;
use crate::core::AbiCStr;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_type_desc)\]
/// D3D11_SHADER_TYPE_DESC
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ShaderTypeDesc<'s> {
    class:      ShaderVariableClass,
    r#type:     ShaderVariableType,
    rows:       u32,
    columns:    u32,
    elements:   u32,
    members:    u32,
    offset:     u32,
    name:       Option<&'s AbiCStr>,
}

test_layout! { ShaderTypeDesc => unsafe winapi::um::d3d11shader::D3D11_SHADER_TYPE_DESC {
    class       => Class,
    r#type      => Type,
    rows        => Rows,
    columns     => Columns,
    elements    => Elements,
    members     => Members,
    offset      => Offset,
    name        => Name,
}}
