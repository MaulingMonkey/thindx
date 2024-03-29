#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::shared::minwindef::LPVOID;
use winapi::um::d3d11shader::*;

use std::ptr::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_variable_desc)\]
/// D3D11_SHADER_VARIABLE_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflectionVariable::get_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
#[repr(C)] pub struct ShaderVariableDesc<'s> {
    pub name:           CStrPtr<'s>, // maybe never null?
    pub start_offset:   u32,
    pub size:           u32,
    pub flags:          ShaderVariableFlags,
    pub default_value:  LPVOID, // TODO: hmm... this is OKish for read, but not for write
    pub start_texture:  u32,
    pub texture_size:   u32,
    pub start_sampler:  u32,
    pub sampler_size:   u32,
}

impl Default for ShaderVariableDesc<'static> {
    fn default() -> Self {
        Self {
            name:           Default::default(),
            start_offset:   0,
            size:           0,
            flags:          ShaderVariableFlags::None,
            default_value:  null_mut(),
            start_texture:  0,
            texture_size:   0,
            start_sampler:  0,
            sampler_size:   0,
        }
    }
}

impl ShaderVariableDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_VARIABLE_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `name`)
    // forbidden: AsMut     (could invalidate `name`)
    // forbidden: DerefMut  (could invalidate `name`)
    // forbidden: FromD3D   (could invalidate `name`)
    ShaderVariableDesc<'_> => D3D11_SHADER_VARIABLE_DESC {
        name            => Name,
        start_offset    => StartOffset,
        size            => Size,
        flags           => uFlags,
        default_value   => DefaultValue,
        start_texture   => StartTexture,
        texture_size    => TextureSize,
        start_sampler   => StartSampler,
        sampler_size    => SamplerSize,
    }
}

//#cpp2rust D3D11_SHADER_VARIABLE_DESC              = d3d11::ShaderVariableDesc
