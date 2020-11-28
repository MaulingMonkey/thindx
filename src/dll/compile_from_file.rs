use crate::*;
use crate::traits::Raw;

use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)\]
    /// D3DCompileFromFile
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// > Refer to the section, "Compiling shaders for UWP", in the remarks for [compile2](Self::compile2).
    ///
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `file_name`     - The shader file to compile
    /// *   `defines`       - An optional array of defines.  Use `()` if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use `()` if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_4?.dll` and earlier
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    ///
    /// <div class="note"><b>Note:</b> You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.</div>
    pub fn compile_from_file<'s>(
        &self,
        file_name:      impl AsRef<Path>,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl Into<Option<&'s str>>,
        target:         impl Into<Option<&'s str>>,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, ErrorKind> {
        // Early outs
        let f           = self.D3DCompileFromFile.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let entrypoint  = entrypoint    .into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let target      = target        .into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let entrypoint  = entrypoint    .as_ref().map_or(null(), |s| s.as_ptr().cast());
        let target      = target        .as_ref().map_or(null(), |s| s.as_ptr().cast());

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();

        let mut code   = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            file_name.as_ptr(),
            defines, include, entrypoint, target,
            flags1, flags2, &mut code, &mut errors,
        )};
        ErrorKind::check(hr)?;

        Ok(CompileResult {
            code:   unsafe { ReadOnlyBlob::from_raw(code as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }
}
