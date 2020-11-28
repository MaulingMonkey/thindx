use crate::*;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetdebuginfo)\]
    /// D3DGetDebugInfo
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    ///
    /// Gets shader debug information.
    ///
    /// ### Arguments
    /// *   `src_data`  Either uncompiled or compiled HLSL code.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_39.dll` and earlier
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust,no_run
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader_src = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let compiled_shader = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// // TODO: This doesn't seem to work?
    ///
    /// let debug_info = compiler.get_debug_info(&shader_src).unwrap();
    /// println!("{:?}", debug_info.get_buffer());
    ///
    /// let debug_info = compiler.get_debug_info(compiled_shader.get_buffer()).unwrap();
    /// println!("{:?}", debug_info.get_buffer());
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_40.dll"))]
    pub fn get_debug_info(&self, src_data: &[u8]) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DGetDebugInfo.ok_or(Error::new("D3DGetDebugInfo", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetDebugInfo", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
