use crate::*;
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
/// *   [ShaderReflection::get_constant_buffer_by_index]
/// *   [ShaderReflection::get_constant_buffer_by_name]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflectionConstantBuffer<'r> {
    ptr:        NonNull<ID3D11ShaderReflectionConstantBuffer>,
    phantom:    PhantomData<&'r LibraryReflection>,
}

impl<'r> ShaderReflectionConstantBuffer<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, srcb: *mut ID3D11ShaderReflectionConstantBuffer) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(srcb)?,
            phantom:    PhantomData,
        })
    }
}

// TODO: methods
