use crate::*;
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
/// *   [FunctionReflection::get_function_parameter]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionParameterReflection<'r> {
    ptr:        NonNull<ID3D11FunctionParameterReflection>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> FunctionParameterReflection<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11FunctionParameterReflection) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("FunctionParameterReflection should never be null"),
            phantom:    PhantomData,
        }
    }
}

impl<'r> FunctionParameterReflection<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionparameterreflection-getdesc)\]
    /// ID3D11FunctionParameterReflection::GetDesc
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_desc(&self) -> Result<ParameterDesc<'r>, Error> {
        let mut desc = ParameterDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(&mut desc as *mut _ as *mut _) };
        Error::check("ID3D11FunctionParameterReflection::GetDesc", hr)?;
        Ok(desc)
    }
}
