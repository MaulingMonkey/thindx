use crate::*;
use crate::d3d11::*;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dloadmodule)\]
    /// D3DLoadModule
    ///
    /// Creates a shader module interface from source data for the shader module.
    ///
    /// > **Note:** This function is part of the HLSL shader linking technology that you can use on all Direct3D 11
    /// > platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders
    /// > at run time.
    ///
    /// ### Arguments
    /// *   `src_data`          - The data to load a [Module] from.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_4?.dll` and earlier
    /// *   Ok([d3d11::Module])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// // TODO
    /// ```
    pub fn load_module(&self, data: &[u8]) -> Result<Module, Error> {
        let f = self.D3DLoadModule.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut module = null_mut();
        let hr = unsafe { f(data.as_ptr().cast(), data.len(), &mut module) };
        Error::check("D3DLoadModule", hr)?;
        Ok(unsafe { Module::from_raw(module) })
    }
}
