use crate::*;
use crate::core::AbiCStr;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_signature_parameter_desc)\]
/// D3D11_SIGNATURE_PARAMETER_DESC
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct SignatureParameterDesc<'s> {
    semantic_name:      Option<&'s AbiCStr>,
    semantic_index:     u32,
    register:           u32,
    system_value_type:  Name,
    component_type:     RegisterComponentType,
    mask:               u8,
    read_write_mask:    u8,
    stream:             u32,
    min_precision:      MinPrecision,
}

test_layout! { SignatureParameterDesc => unsafe winapi::um::d3d11shader::D3D11_SIGNATURE_PARAMETER_DESC {
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
