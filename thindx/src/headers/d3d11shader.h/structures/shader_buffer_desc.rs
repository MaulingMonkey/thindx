#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_buffer_desc)\]
/// D3D11_SHADER_BUFFER_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflectionConstantBuffer::get_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderBufferDesc<'s> {
    pub name:                           CStrPtr<'s>, // maybe never null?
    pub ty:                             CBufferType,
    pub variables:                      u32,
    pub size:                           u32,
    pub flags:                          ShaderCbufferFlags,
}

impl ShaderBufferDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_BUFFER_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `name`)
    // forbidden: AsMut     (could invalidate `name`)
    // forbidden: DerefMut  (could invalidate `name`)
    // forbidden: FromD3D   (could invalidate `name`)
    ShaderBufferDesc<'_> => D3D11_SHADER_BUFFER_DESC {
        name                            => Name,
        ty                              => Type,
        variables                       => Variables,
        size                            => Size,
        flags                           => uFlags,
    }
}

//#cpp2rust D3D11_SHADER_BUFFER_DESC                = d3d11::ShaderBufferDesc
