use crate::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionvariable)\]
/// ID3D11ShaderReflectionVariable
///
/// This shader-reflection interface provides access to a variable.
///
/// ### See Also
/// *   [ShaderReflection::get_variable_by_name]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflectionVariable<'r> {
    ptr:        NonNull<ID3D11ShaderReflectionVariable>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> ShaderReflectionVariable<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, fpr: *mut ID3D11ShaderReflectionVariable) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(fpr)?,
            phantom:    PhantomData,
        })
    }
}

// TODO: methods
