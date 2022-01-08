use crate::*;
use crate::d3d::*;

use std::ptr::*;

/// <h1 id="d3d11" class="section-header"><a href="#d3d11">D3D11 Factories & APIs</a></h1>
impl Compiler {
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let d3dc = d3d::Compiler::new(47).unwrap();
    /// let flg: d3d11::FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub fn create_function_linking_graph(&self, flags: Option<void::Void>) -> Result<d3d11::FunctionLinkingGraph, MethodError> {
        let f = self.D3DCreateFunctionLinkingGraph.ok_or(MethodError::new("D3DCreateFunctionLinkingGraph", THINERR::MISSING_DLL_EXPORT))?;
        let _ = flags; let flags = 0;
        let mut flg = null_mut();
        let hr = unsafe { f(flags, &mut flg) };
        MethodError::check("D3DCreateFunctionLinkingGraph", hr)?;
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let d3dc = d3d::Compiler::new(47).unwrap();
    /// let linker : d3d11::Linker = d3dc.create_linker().unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub fn create_linker(&self) -> Result<d3d11::Linker, MethodError> {
        let f = self.D3DCreateLinker.ok_or(MethodError::new("D3DCreateFunctionLinkingGraph", THINERR::MISSING_DLL_EXPORT))?;
        let mut linker = null_mut();
        let hr = unsafe { f(&mut linker) };
        MethodError::check("D3DCreateFunctionLinkingGraph", hr)?;

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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let source = "export float4 example(float4 i) { return 2*i; }";
    /// let CompileResult { shader: code_blob, errors: error_blob } = d3dc.compile(
    ///     source.as_bytes(), "example.hlsl",
    ///     None, None, (), "lib_5_0",
    ///     Compile::OptimizationLevel3, CompileEffect::None,
    /// ).unwrap();
    /// let shader_library = d3dc.load_module(&code_blob).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub fn load_module(&self, src_data: &Bytecode) -> Result<d3d11::Module, MethodError> {
        let f = self.D3DLoadModule.ok_or(MethodError::new("D3DLoadModule", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();
        let mut module = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut module) };
        MethodError::check("D3DLoadModule", hr)?;
        Ok(unsafe { d3d11::Module::from_raw(module) })
    }
}
