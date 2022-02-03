#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_type_desc)\]
/// D3D11_SHADER_TYPE_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflectionType::get_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderTypeDesc<'s> {
    pub class:      ShaderVariableClass,
    pub ty:         ShaderVariableType,
    pub rows:       u32,
    pub columns:    u32,
    pub elements:   u32,
    pub members:    u32,
    pub offset:     u32,
    pub name:       CStrPtr<'s>, // maybe never null?
}

impl ShaderTypeDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_TYPE_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `name`)
    // forbidden: AsMut     (could invalidate `name`)
    // forbidden: DerefMut  (could invalidate `name`)
    // forbidden: FromD3D   (could invalidate `name`)
    ShaderTypeDesc<'_> => D3D11_SHADER_TYPE_DESC {
        class       => Class,
        ty          => Type,
        rows        => Rows,
        columns     => Columns,
        elements    => Elements,
        members     => Members,
        offset      => Offset,
        name        => Name,
    }
}

//#cpp2rust D3D11_SHADER_TYPE_DESC                  = d3d11::ShaderTypeDesc
