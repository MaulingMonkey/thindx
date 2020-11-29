use crate::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)\]
/// D3D11_SHADER_BUFFER_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ShaderBufferDesc<'s> {
    name:                           Option<CStrPtr<'s>>,
    r#type:                         CbufferType,
    variables:                      u32,
    size:                           u32,
    flags:                          ShaderCbufferFlags,
}

test_layout! { ShaderBufferDesc => unsafe D3D11_SHADER_BUFFER_DESC {
    name                            => Name,
    r#type                          => Type,
    variables                       => Variables,
    size                            => Size,
    flags                           => uFlags,
}}
