use crate::*;

use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dstripshader)\]
    /// D3DStripShader
    ///
    /// Removes unwanted blobs from a compilation result.
    ///
    /// ### Arguments
    /// *   `shader_bytecode`   - The original shader bytecode.
    /// *   `strip_flags`       - The [CompilerStripFlags] to use.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_39.dll` and earlier
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let ps : CompileResult = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let ps : ReadOnlyBlob = compiler.strip_shader(ps.shader.get_buffer(), CompilerStripFlags::DebugInfo).unwrap();
    /// ```
    ///
    /// <div class="note"><b>Note:</b>  The D3dcompiler_40.dll or later version of the file contains the D3DStripShader compiler function.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::strip_shader wasn't added until d3dcompiler_40.dll"))]
    pub fn strip_shader<'s>(
        &self,
        shader_bytecode:    &[u8],
        strip_flags:        impl Into<CompilerStripFlags>,
    ) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DStripShader.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let mut blob = null_mut();
        let hr = unsafe { f(shader_bytecode.as_ptr().cast(), shader_bytecode.len(), strip_flags.into().into(), &mut blob) };
        Error::check("D3DStripShader", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
