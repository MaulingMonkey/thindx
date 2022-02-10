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
    /// ### ⚠️ Safety ⚠️
    /// *   `ptr` must be a valid `ID3D11ShaderReflectionVariable` for the entire lifetime of `_`
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11ShaderReflectionVariable) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("ShaderReflectionVariable should never be null"),
            phantom:    PhantomData,
        }
    }
}

impl<'r> ShaderReflectionVariable<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-getbuffer)\]
    /// ID3D11ShaderReflectionVariable::GetBuffer
    ///
    /// This method returns the buffer of the current [d3d11::ShaderReflectionVariable].
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_buffer(&self) -> ShaderReflectionConstantBuffer<'r> {
        fn_context!(d3d11::ShaderReflectionVariable::get_buffer => ID3D11ShaderReflectionVariable::GetBuffer);
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
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_desc(&self) -> Result<ShaderVariableDesc<'r>, MethodError> {
        fn_context!(d3d11::ShaderReflectionVariable::get_desc => ID3D11ShaderReflectionVariable::GetDesc);
        let mut desc = ShaderVariableDesc::default();
        fn_check_hr!(unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) })?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-getinterfaceslot)\]
    /// ID3D11ShaderReflectionVariable::GetInterfaceSlot
    ///
    /// Gets the corresponding interface slot for a variable that represents an interface pointer.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    //#allow_missing_argument_docs
    pub fn get_interface_slot(&self, array_index: u32) -> u32 {
        fn_context!(d3d11::ShaderReflectionVariable::get_interface_slot => ID3D11ShaderReflectionVariable::GetInterfaceSlot);
        unsafe { self.ptr.as_ref().GetInterfaceSlot(array_index) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectionvariable-gettype)\]
    /// ID3D11ShaderReflectionVariable::GetType
    ///
    /// Get a shader-variable type.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_type(&self) -> ShaderReflectionType<'r> {
        fn_context!(d3d11::ShaderReflectionVariable::get_type => ID3D11ShaderReflectionVariable::GetType);
        let ptr = unsafe { self.ptr.as_ref().GetType() };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }
}

//#cpp2rust ID3D11ShaderReflectionVariable                      = d3d11::ShaderReflectionVariable
