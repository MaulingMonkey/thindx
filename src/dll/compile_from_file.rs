use crate::*;
use crate::traits::Raw;

use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)\]
    /// D3DCompileFromFile
    ///
    /// ### Arguments
    /// *   `file_name`     - The shader file to compile
    /// *   `defines`       - An optional array of defines.  Use `()` if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use `()` if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [D3DCOMPILE_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants) constants.
    /// *   `flags2`        - [D3DCOMPILE_EFFECT_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants) constants.  Ignored except for `fx_*` targets.
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn compile_from_file<'s>(
        &self,
        file_name:      impl AsRef<Path>,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl Into<Option<&'s str>>,
        target:         impl Into<Option<&'s str>>,
        flags1:         u32, // TODO: type
        flags2:         u32, // TODO: type
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
