use crate::*;
use crate::d3d11::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11libraryreflection)\]
/// ID3D11LibraryReflection
///
/// A library-reflection interface accesses library info.
#[derive(Clone)] #[repr(transparent)]
pub struct LibraryReflection(pub(crate) mcom::Rc<ID3D11LibraryReflection>);

convert!(unsafe LibraryReflection => Unknown, ID3D11LibraryReflection);

impl LibraryReflection {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11libraryreflection-getdesc)\]
    /// ID3D11LibraryReflection::GetDesc
    ///
    /// Get a library descriptor structure for the library reflection.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_desc_raw(&self) -> Result<D3D11_LIBRARY_DESC, Error> {
        let mut desc = unsafe { std::mem::zeroed::<D3D11_LIBRARY_DESC>() }; // TODO: structify?
        let hr = unsafe { self.0.GetDesc(&mut desc) };
        Error::check("ID3D11LibraryReflection::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11libraryreflection-getfunctionbyindex)\]
    /// ID3D11LibraryReflection::GetFunctionByIndex
    ///
    /// Gets the function reflector.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn get_function_by_index(&self, function_index: i32) -> Option<FunctionReflection> {
        let ptr = unsafe { self.0.GetFunctionByIndex(function_index) };
        unsafe { FunctionReflection::from_raw(self, ptr) }
    }
}
