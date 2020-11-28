use crate::*;

use winapi::Interface;

use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflectlibrary)\]
    /// D3DReflectLibrary
    ///
    /// Creates a library-reflection interface from source data that contains an HLSL library of functions.
    ///
    /// > **Note:** This function is part of the HLSL shader linking technology that you can use on all Direct3D 11
    /// > platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders
    /// > at run time.
    ///
    /// ### Arguments
    /// *   `src_data`  - An HLSL library of functions.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust,no_run
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// // TODO: figure out what reflect_library actually takes and demo that.
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect_library::<d3d11::LibraryReflection>(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    pub fn reflect_library<C: Raw>(&self, src_data: &[u8]) -> Result<C, Error> where C::Raw : Interface {
        let f = self.D3DReflectLibrary.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &C::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflectLibrary", hr)?;
        Ok(unsafe { C::from_raw(reflector.cast()) })
    }
}
