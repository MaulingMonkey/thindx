use crate::*;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreatelinker)\]
    /// D3DCreateLinker
    ///
    /// Creates a linker interface.
    ///
    /// > **Note:**  This function is part of the HLSL shader linking technology that you can use on all Direct3D 11
    /// > platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders
    /// > at run time.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_4?.dll` and earlier
    /// *   Ok([d3d11::Linker])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let linker : d3d11::Linker = compiler.create_linker().unwrap();
    /// ```
    pub fn create_linker(&self) -> Result<d3d11::Linker, ErrorKind> {
        // Early outs
        let f           = self.D3DCreateLinker.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut linker = null_mut();
        let hr = unsafe { f(&mut linker) };
        ErrorKind::check(hr)?;

        Ok(unsafe { d3d11::Linker::from_raw(linker) })
    }
}
