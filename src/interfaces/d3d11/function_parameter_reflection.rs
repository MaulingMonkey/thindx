use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionparameterreflection)\]
/// ID3D11FunctionParameterReflection
///
/// A function-parameter-reflection interface accesses function-parameter info.
///
/// ### See Also
/// *   [ShaderReflection::]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionParameterReflection<'r> {
    ptr:        NonNull<ID3D11FunctionParameterReflection>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> FunctionParameterReflection<'r> {
    pub(crate) unsafe fn from_raw(_: &'r ShaderReflection, fpr: *mut ID3D11FunctionParameterReflection) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(fpr)?,
            phantom:    PhantomData,
        })
    }
}

// TODO: methods
