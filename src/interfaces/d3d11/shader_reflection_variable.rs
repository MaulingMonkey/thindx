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

impl<'r> ShaderReflectionVariable<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-getbuffer)\]
    /// ID3D11ShaderReflectionVariable::GetBuffer
    ///
    /// This method returns the buffer of the current ID3D11ShaderReflectionVariable.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_buffer(&self) -> Option<ShaderReflectionConstantBuffer> {
        let ptr = unsafe { self.ptr.as_ref().GetBuffer() };
        unsafe { ShaderReflectionConstantBuffer::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-getdesc)\]
    /// ID3D11ShaderReflectionVariable::GetDesc
    ///
    /// Get a shader-variable description.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_desc_raw(&self) -> Result<D3D11_SHADER_VARIABLE_DESC, Error> {
        let mut desc = unsafe { std::mem::zeroed::<D3D11_SHADER_VARIABLE_DESC>() };
        let hr = unsafe { self.ptr.as_ref().GetDesc(&mut desc) };
        Error::check("ID3D11ShaderReflectionVariable::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-getinterfaceslot)\]
    /// ID3D11ShaderReflectionVariable::GetInterfaceSlot
    ///
    /// Gets the corresponding interface slot for a variable that represents an interface pointer.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_interface_slot(&self, array_index: u32) -> u32 {
        unsafe { self.ptr.as_ref().GetInterfaceSlot(array_index) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-gettype)\]
    /// ID3D11ShaderReflectionVariable::GetType
    ///
    /// Get a shader-variable type.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_type(&self) -> Option<ShaderReflectionType> {
        let ptr = unsafe { self.ptr.as_ref().GetType() };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }
}
