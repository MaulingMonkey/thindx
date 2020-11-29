use crate::*;

use std::ptr::*;

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
    /// *   `flags`         - Reserved, initialize with `()`
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_4?.dll` and earlier
    /// *   Ok([d3d11::FunctionLinkingGraph])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let flg: d3d11::FunctionLinkingGraph = compiler.create_function_linking_graph(()).unwrap();
    /// ```
    pub fn create_function_linking_graph(&self, flags: ()) -> Result<d3d11::FunctionLinkingGraph, ErrorKind> {
        // Early outs
        let f = self.D3DCreateFunctionLinkingGraph.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let _ = flags; let flags = 0;

        let mut flg = null_mut();
        let hr = unsafe { f(flags, &mut flg) };
        ErrorKind::check(hr)?;
        Ok(unsafe { d3d11::FunctionLinkingGraph::from_raw(flg) })
    }
}
