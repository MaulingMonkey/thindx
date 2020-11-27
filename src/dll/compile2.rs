use crate::*;
use crate::traits::Raw;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2)\]
    /// D3DCompile2
    ///
    /// The difference between [compile2](Self::compile2) and [compile](Self::compile) is that [compile2](Self::compile2)
    /// takes some optional parameters (`secondary_data_flags` and `secondary_data`) that can be used to control some
    /// aspects of how bytecode is generated.  Refer to the descriptions of these parameters for more details. There is
    /// no difference otherwise to the efficiency of the bytecode generated between [compile2](Self::compile2) and
    /// [compile](Self::compile).
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use `()` if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use `()` if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("vs_main")`.  Ignored if `target` is `"fx_*"`.
    /// *   `target`        - A target shader profile such as `Some("ps_3_0")`, `Some("vs_5_0")`, `Some("fx_4_0")`, etc.
    /// *   `flags1`        - [D3DCOMPILE_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants) constants.
    /// *   `flags2`        - [D3DCOMPILE_EFFECT_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants) constants.  Ignored except for `fx_*` targets.
    /// *   `secondary_data_flags`  - D3DCOMPILE_SECDATA_* constants.
    /// *   `secondary_data`    - A pointer to secondary data. If you don't pass secondary data, set to [None].
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn compile2<'s>(
        &self,
        src_data:               &[u8],
        source_name:            impl Into<Option<&'s str>>,
        defines:                impl AsShaderMacros,
        include:                impl AsID3DInclude,
        entrypoint:             impl Into<Option<&'s str>>,
        target:                 impl Into<Option<&'s str>>,
        flags1:                 u32, // TODO: type
        flags2:                 u32, // TODO: type
        secondary_data_flags:   u32, // TODO: type
        secondary_data:         impl Into<Option<&'s [u8]>>,
    ) -> Result<CompileResult, ErrorKind> {
        // Early outs
        let defines     = defines.as_shader_macros()?;

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let entrypoint  = entrypoint    .into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let target      = target        .into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let source_name = source_name   .as_ref().map_or(null(), |s| s.as_ptr().cast());
        let entrypoint  = entrypoint    .as_ref().map_or(null(), |s| s.as_ptr().cast());
        let target      = target        .as_ref().map_or(null(), |s| s.as_ptr().cast());

        let include     = include.as_id3dinclude();

        let secondary_data      = secondary_data.into();
        let secondary_data_len  = secondary_data.map_or(0, |sd| sd.len());
        let secondary_data      = secondary_data.map_or(null(), |sd| sd.as_ptr()).cast();

        let mut code   = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { (self.D3DCompile2)(
            src_data.as_ptr().cast(), src_data.len(),
            source_name, defines, include, entrypoint, target,
            flags1, flags2, secondary_data_flags, secondary_data, secondary_data_len,
            &mut code, &mut errors,
        )};
        ErrorKind::check(hr)?;

        Ok(CompileResult {
            code:   unsafe { ReadOnlyBlob::from_raw(code as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }
}
