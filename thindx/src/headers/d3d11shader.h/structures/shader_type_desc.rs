use crate::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_type_desc)\]
/// D3D11_SHADER_TYPE_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflectionType::get_desc]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderTypeDesc<'s> {
    pub class:      ShaderVariableClass,
    pub r#type:     ShaderVariableType,
    pub rows:       u32,
    pub columns:    u32,
    pub elements:   u32,
    pub members:    u32,
    pub offset:     u32,
    pub name:       ConstCStrPtrNullIsEmpty<'s>, // maybe never null?
}

impl ShaderTypeDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_TYPE_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

test_layout! { ShaderTypeDesc => unsafe D3D11_SHADER_TYPE_DESC {
    class       => Class,
    r#type      => Type,
    rows        => Rows,
    columns     => Columns,
    elements    => Elements,
    members     => Members,
    offset      => Offset,
    name        => Name,
}}
