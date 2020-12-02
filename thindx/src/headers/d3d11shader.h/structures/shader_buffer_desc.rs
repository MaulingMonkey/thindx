use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_buffer_desc)\]
/// D3D11_SHADER_BUFFER_DESC
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderBufferDesc<'s> {
    pub name:                           Option<&'s AbiCStr>,
    pub r#type:                         CBufferType,
    pub variables:                      u32,
    pub size:                           u32,
    pub flags:                          ShaderCbufferFlags,
}

impl ShaderBufferDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_BUFFER_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

test_layout! { ShaderBufferDesc => unsafe D3D11_SHADER_BUFFER_DESC {
    name                            => Name,
    r#type                          => Type,
    variables                       => Variables,
    size                            => Size,
    flags                           => uFlags,
}}
