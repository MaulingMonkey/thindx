use crate::*;

use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dpreprocess)\]
    /// D3DPreprocess
    ///
    /// Preprocesses uncompiled HLSL code.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use `()` if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use `()` if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_39.dll` and earlier
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let ps = compiler.preprocess(&basic_hlsl, r"test\data\basic.hlsl", (), None).unwrap();
    /// println!("{}", String::from_utf8_lossy(ps.shader.get_buffer()));
    /// ```
    ///
    /// ### Output
    /// ```hlsl
    /// #line 1 "C:\\local\\thin3dcompiler\\test\\data\\basic.hlsl"
    ///
    ///
    /// struct Vertex {
    ///     float4 position : POSITION0 ;
    ///     float4 color : COLOR0 ;
    /// } ;
    ///
    /// struct VsToPs {
    /// ...
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_40.dll"))]
    pub fn preprocess<'s>(
        &self,
        src_data:       &[u8],
        source_name:    impl Into<Option<&'s str>>,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DPreprocess.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name.into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let source_name = source_name.as_ref().map_or(null(), |s| s.as_ptr().cast());
        let include     = include.as_id3dinclude();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), source_name, defines, include, &mut shader, &mut errors)};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }
}
