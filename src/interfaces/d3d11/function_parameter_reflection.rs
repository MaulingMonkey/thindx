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
/// *   [ShaderReflection::]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionParameterReflection<'r> {
    ptr:        NonNull<ID3D11FunctionParameterReflection>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> FunctionParameterReflection<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, fpr: *mut ID3D11FunctionParameterReflection) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(fpr)?,
            phantom:    PhantomData,
        })
    }
}

impl<'r> FunctionParameterReflection<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionparameterreflection-getdesc)\]
    /// ID3D11FunctionParameterReflection::GetDesc
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_desc_raw(&self) -> Result<D3D11_PARAMETER_DESC, Error> {
        let mut desc = unsafe { std::mem::zeroed::<D3D11_PARAMETER_DESC>() };
        let hr = unsafe { self.ptr.as_ref().GetDesc(&mut desc) };
        Error::check("ID3D11FunctionParameterReflection::GetDesc", hr)?;
        Ok(desc)
    }
}
