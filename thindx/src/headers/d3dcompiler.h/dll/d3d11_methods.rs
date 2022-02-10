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
    /// # use thindx::*; let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// let flg: d3d11::FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub fn create_function_linking_graph(&self, flags: Option<std::convert::Infallible>) -> Result<d3d11::FunctionLinkingGraph, MethodError> {
        fn_context_dll!(d3d::Compiler::create_function_linking_graph => self.D3DCreateFunctionLinkingGraph);
        let _ = flags;

        // SAFETY: ✔️
        //  * `f` should be a valid/sound fn, like all of `self.*`
        //  * `0` for reserved flags of `D3DCreateFunctionLinkingGraph` is expected
        //  * `flg` is just an output `**ID3D11FunctionLinkingGraph`
        let mut flg = null_mut();
        let hr = unsafe { D3DCreateFunctionLinkingGraph(0, &mut flg) };
        fn_check_hr!(hr)?;

        // SAFETY: ✔️ `flg` is null (from_raw panics) or valid
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
    /// # use thindx::*; let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// let linker : d3d11::Linker = d3dc.create_linker().unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub fn create_linker(&self) -> Result<d3d11::Linker, MethodError> {
        fn_context_dll!(d3d::Compiler::create_linker => self.D3DCreateLinker);

        // SAFETY: ✔️
        //  * `f` should be a valid/sound fn, like all of `self.*`
        //  * `linker` is just an output `**ID3D11Linker`
        let mut linker = null_mut();
        let hr = unsafe { D3DCreateLinker(&mut linker) };
        fn_check_hr!(hr)?;

        // SAFETY: ✔️ `linker` is null (from_raw panics) or valid
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
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
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
        fn_context_dll!(d3d::Compiler::load_module => self.D3DLoadModule);
        let src_data = src_data.as_bytes();

        // SAFETY: ✔️
        //  * `f` should be a valid/sound fn, like all of `self.*`
        //  * `D3DLoadModule` may require `src_data` be valid bytecode, but this is implied by `Bytecode`
        //  * `module` is just an output `**ID3D11Module`
        let mut module = null_mut();
        let hr = unsafe { D3DLoadModule(src_data.as_ptr().cast(), src_data.len(), &mut module) };
        fn_check_hr!(hr)?;

        // SAFETY: ✔️ `module` is null (from_raw panics) or valid
        Ok(unsafe { d3d11::Module::from_raw(module) })
    }
}
