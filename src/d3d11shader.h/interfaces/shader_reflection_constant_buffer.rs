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

impl<'r> ShaderReflectionConstantBuffer<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionconstantbuffer-getdesc)\]
    /// ID3D11ShaderReflectionConstantBuffer::GetDesc
    ///
    /// Get a constant-buffer description.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_desc_raw(&self) -> Result<D3D11_SHADER_BUFFER_DESC, Error> {
        let mut desc = unsafe { std::mem::zeroed::<D3D11_SHADER_BUFFER_DESC>() };
        let hr = unsafe { self.ptr.as_ref().GetDesc(&mut desc) };
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
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_variable_by_index(&self, index: u32) -> Option<ShaderReflectionVariable<'r>> {
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
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_variable_by_name(&self, name: impl TryIntoAsCStr) -> Option<ShaderReflectionVariable<'r>> {
        let name = name.try_into().ok()?;
        let ptr = unsafe { self.ptr.as_ref().GetVariableByName(name.as_cstr()) };
        unsafe { ShaderReflectionVariable::from_raw(self.phantom, ptr) }
    }
}
