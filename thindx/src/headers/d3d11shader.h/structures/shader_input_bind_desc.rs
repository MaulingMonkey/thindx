#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_input_bind_desc)\]
/// D3D11_SHADER_INPUT_BIND_DESC
///
/// ### See Also
/// *   [d3d11::FunctionReflection::get_resource_binding_desc]
/// *   [d3d11::FunctionReflection::get_resource_binding_desc_by_name]
/// *   [d3d11::ShaderReflection::get_resource_binding_desc]
/// *   [d3d11::ShaderReflection::get_resource_binding_desc_by_name]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderInputBindDesc<'s> {
    pub name:           CStrPtr<'s>, // maybe never null?
    pub ty:             ShaderInputType,
    pub bind_point:     u32,
    pub bind_count:     u32,
    pub flags:          ShaderInputFlags,
    pub return_type:    ResourceReturnType,
    pub dimension:      SrvDimension,
    pub num_samples:    u32,
}

impl ShaderInputBindDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_INPUT_BIND_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `name`)
    // forbidden: AsMut     (could invalidate `name`)
    // forbidden: DerefMut  (could invalidate `name`)
    // forbidden: FromD3D   (could invalidate `name`)
    ShaderInputBindDesc<'_> => D3D11_SHADER_INPUT_BIND_DESC {
        name            => Name,
        ty              => Type,
        bind_point      => BindPoint,
        bind_count      => BindCount,
        flags           => uFlags,
        return_type     => ReturnType,
        dimension       => Dimension,
        num_samples     => NumSamples,
    }
}

//#cpp2rust D3D11_SHADER_INPUT_BIND_DESC            = d3d11::ShaderInputBindDesc
