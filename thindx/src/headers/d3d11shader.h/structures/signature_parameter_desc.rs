#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_signature_parameter_desc)\]
/// D3D11_SIGNATURE_PARAMETER_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflection::get_input_parameter_desc]
/// *   [d3d11::ShaderReflection::get_output_parameter_desc]
/// *   [d3d11::ShaderReflection::get_patch_constant_parameter_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct SignatureParameterDesc<'s> {
    /// An input semantic name such as "POSITION", "TEXCOORD", etc.
    pub semantic_name:      CStrPtr<'s>, // maybe never null?
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

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `semantic_name`)
    // forbidden: AsMut     (could invalidate `semantic_name`)
    // forbidden: DerefMut  (could invalidate `semantic_name`)
    // forbidden: FromD3D   (could invalidate `semantic_name`)
    SignatureParameterDesc<'_> => D3D11_SIGNATURE_PARAMETER_DESC {
        semantic_name       => SemanticName,
        semantic_index      => SemanticIndex,
        register            => Register,
        system_value_type   => SystemValueType,
        component_type      => ComponentType,
        mask                => Mask,
        read_write_mask     => ReadWriteMask,
        stream              => Stream,
        min_precision       => MinPrecision,
    }
}

//#cpp2rust D3D11_SIGNATURE_PARAMETER_DESC          = d3d11::SignatureParameterDesc
