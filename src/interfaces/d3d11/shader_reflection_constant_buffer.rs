use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionconstantbuffer)\]
/// ID3D11ShaderReflectionConstantBuffer
///
/// This shader-reflection interface provides access to a constant buffer.
///
/// ### See Also
/// *   [ShaderReflection::]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflectionConstantBuffer<'r> {
    ptr:        NonNull<ID3D11ShaderReflectionConstantBuffer>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> ShaderReflectionConstantBuffer<'r> {
    pub(crate) unsafe fn from_raw(_: &'r ShaderReflection, fpr: *mut ID3D11ShaderReflectionConstantBuffer) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(fpr)?,
            phantom:    PhantomData,
        })
    }
}

// TODO: methods
