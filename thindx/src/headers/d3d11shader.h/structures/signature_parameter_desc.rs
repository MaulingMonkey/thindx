use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_signature_parameter_desc)\]
/// D3D11_SIGNATURE_PARAMETER_DESC
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct SignatureParameterDesc<'s> {
    pub semantic_name:      Option<&'s AbiCStr>,
    pub semantic_index:     u32,
    pub register:           u32,
    pub system_value_type:  Name,
    pub component_type:     RegisterComponentType,
    pub mask:               u8,
    pub read_write_mask:    u8,
    pub stream:             u32,
    pub min_precision:      MinPrecision,
}

impl SignatureParameterDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SIGNATURE_PARAMETER_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

test_layout! { SignatureParameterDesc => unsafe D3D11_SIGNATURE_PARAMETER_DESC {
    semantic_name       => SemanticName,
    semantic_index      => SemanticIndex,
    register            => Register,
    system_value_type   => SystemValueType,
    component_type      => ComponentType,
    mask                => Mask,
    read_write_mask     => ReadWriteMask,
    stream              => Stream,
    min_precision       => MinPrecision,
}}
