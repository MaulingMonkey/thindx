use crate::*;

use std::ptr::*;

/// <h1 id="d3d11" class="section-header"><a href="#d3d11">D3D11 Factories & APIs</a></h1>
impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreatefunctionlinkinggraph)\]
    /// D3DCreateFunctionLinkingGraph
    ///
    /// Creates a function-linking-graph interface.
    ///
    /// > **Note:** This function is part of the HLSL shader linking technology that you can use on all Direct3D 11 platforms
    /// > to create precompiled HLSL functions, package them into libraries, and link them into full shaders at run time.
    ///
    /// ### Arguments
    /// *   `flags`         - Reserved, initialize with `None`
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_4?.dll` and earlier
    /// *   Ok([d3d11::FunctionLinkingGraph])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let flg: d3d11::FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// ```
    pub fn create_function_linking_graph(&self, flags: Option<void::Void>) -> Result<d3d11::FunctionLinkingGraph, ErrorKind> {
        // Early outs
        let f = self.D3DCreateFunctionLinkingGraph.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let _ = flags; let flags = 0;

        let mut flg = null_mut();
        let hr = unsafe { f(flags, &mut flg) };
        ErrorKind::check(hr)?;
        Ok(unsafe { d3d11::FunctionLinkingGraph::from_raw(flg) })
    }

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
    /// *   `src_data`          - The data to load a [d3d11::Module] from.
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
    pub fn load_module(&self, data: &[u8]) -> Result<d3d11::Module, Error> {
        let f = self.D3DLoadModule.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut module = null_mut();
        let hr = unsafe { f(data.as_ptr().cast(), data.len(), &mut module) };
        Error::check("D3DLoadModule", hr)?;
        Ok(unsafe { d3d11::Module::from_raw(module) })
    }
}
