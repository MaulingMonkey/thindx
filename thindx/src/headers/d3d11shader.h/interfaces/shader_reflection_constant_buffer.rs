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
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11ShaderReflectionConstantBuffer) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("ShaderReflectionConstantBuffer should never be null"),
            phantom:    PhantomData,
        }
    }
}

impl<'r> ShaderReflectionConstantBuffer<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getdesc)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetDesc
    ///
    /// Get a constant-buffer description.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_desc(&self) -> Result<ShaderBufferDesc<'r>, Error> {
        let mut desc = ShaderBufferDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflectionConstantBuffer::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getvariablebyindex)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetVariableByIndex
    ///
    /// Get a shader-reflection variable by index.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_variable_by_index(&self, index: u32) -> ShaderReflectionVariable<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetVariableByIndex(index) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getvariablebyname)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetVariableByName
    ///
    /// Get a shader-reflection variable by name.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionVariable<'r> {
        let name = name.try_into().ok();
        let name = name.map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetVariableByName(name) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }
}
