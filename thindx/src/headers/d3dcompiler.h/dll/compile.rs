use crate::*;
use crate::d3d::*;

use std::fmt::{self, Debug, Display, Formatter};
use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;



/// { code: [ReadOnlyBlob], errors: Option&lt;[ReadOnlyBlob]&gt; } returned by [Compiler::compile]/[compile2](Compiler::compile2)
pub struct CompileResult {
    pub shader:     ReadOnlyBlob,
    pub errors:     CompilerDiagnostics,
}



/// { kind: [ErrorKind], shader: Option&lt;[ReadOnlyBlob]&gt;, errors: Option&lt;[ReadOnlyBlob]&gt; }
pub struct CompileError {
    pub kind:       ErrorKind,
    pub method:     Option<&'static str>,
    pub shader:     Option<ReadOnlyBlob>, // may or may not have generated a shader blob despite errors
    pub errors:     CompilerDiagnostics,
}

impl From<Error> for CompileError {
    fn from(e: Error) -> Self {
        let Error { kind, method, errors } = e;
        Self { kind, method, errors, shader: None }
    }
}

impl std::error::Error for CompileError {}

impl Debug for CompileError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("CompileError")
            .field("kind", &self.kind)
            .field("shader", &self.shader.as_ref().map(|_| ..))
            .field("errors", &self.errors.to_utf8_lossy())
            .finish()
    }
}

impl Display for CompileError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Error compiling shader: {:?}", self.kind)?;
        if !self.errors.is_empty() {
            writeln!(fmt, "\n{}", self.errors.to_utf8_lossy())?;
        }
        Ok(())
    }
}



/// <h1 id="compile" class="section-header"><a href="#compile">Compile & Preprocess HLSL to Bytecode</a></h1>
impl Compiler {
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
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT]     - `d3dcompiler_4?.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let compiler = Compiler::new(47).unwrap();
    /// let pixel_shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// // TODO: defines/includes examples
    /// ```
    /// [_examples::d3dcompiler_02_compile]<br>
    #[requires(!store)]
    // #[requires(d3dcompiler=47)] // ?
    pub fn compile_from_file<'s>(
        &self,
        file_name:      impl AsRef<Path>,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompileFromFile.ok_or(Error::new("D3DCompileFromFile", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| Error::new("D3DCompileFromFile", e))?;

        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        let entrypoint  = entrypoint.try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompileFromFile"), shader: None, errors: Default::default() })?;
        let target      = target    .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompileFromFile"), shader: None, errors: Default::default() })?;
        let entrypoint  = entrypoint.as_opt_cstr();
        let target      = target    .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            file_name.as_ptr(),
            defines, include, entrypoint, target,
            flags1, flags2, &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            method: Some("D3DCompileFromFile"),
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile)\]
    /// D3DCompile
    ///
    /// Compile HLSL code or an effect file into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT]     - `d3dcompiler_39.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let compiler = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = compiler.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    /// ```
    /// [_examples::d3dcompiler_02_compile]<br>
    #[requires(d3dcompiler=40)]
    pub fn compile<'s>(
        &self,
        src_data:       impl AsRef<[u8]>,
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile.ok_or(Error::new("D3DCompile", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| Error::new("D3DCompile", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile"), shader: None, errors: Default::default() })?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile"), shader: None, errors: Default::default() })?;
        let target      = target        .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile"), shader: None, errors: Default::default() })?;
        let source_name = source_name   .as_opt_cstr();
        let entrypoint  = entrypoint    .as_opt_cstr();
        let target      = target        .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            src_data.as_ptr().cast(), src_data.len(),
            source_name, defines, include, entrypoint, target,
            flags1, flags2, &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            method: Some("D3DCompile"),
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2)\]
    /// D3DCompile2
    ///
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("vs_main")`.  Ignored if `target` is `"fx_*"`.
    /// *   `target`        - A target shader profile such as `Some("ps_3_0")`, `Some("vs_5_0")`, `Some("fx_4_0")`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    /// *   `secondary_data_flags`  - [CompileSecData]::\* constants.
    /// *   `secondary_data`    - A pointer to secondary data. If you don't pass secondary data, set to [None].
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT]     - `d3dcompiler_4?.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let compiler = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = compiler.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    /// ```
    /// [_examples::d3dcompiler_02_compile]<br>
    ///
    /// ### Remarks
    /// The difference between [compile2](Self::compile2) and [compile](Self::compile) is that [compile2](Self::compile2)
    /// takes some optional parameters (`secondary_data_flags` and `secondary_data`) that can be used to control some
    /// aspects of how bytecode is generated.  Refer to the descriptions of these parameters for more details. There is
    /// no difference otherwise to the efficiency of the bytecode generated between [compile2](Self::compile2) and
    /// [compile](Self::compile).
    //#[requires(d3dcompiler=47)] // ?
    pub fn compile2<'s>(
        &self,
        src_data:               impl AsRef<[u8]>,
        source_name:            impl TryIntoAsOptCStr,
        defines:                impl AsShaderMacros,
        include:                impl AsID3DInclude,
        entrypoint:             impl TryIntoAsOptCStr,
        target:                 impl TryIntoAsCStr,
        flags1:                 impl Into<Compile>,
        flags2:                 impl Into<CompileEffect>,
        secondary_data_flags:   impl Into<CompileSecData>,
        secondary_data:         impl Into<Option<&'s [u8]>>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile2.ok_or(Error::new("D3DCompile2", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| Error::new("D3DCompile2", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile2"), shader: None, errors: Default::default() })?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile2"), shader: None, errors: Default::default() })?;
        let target      = target        .try_into().map_err(|e| CompileError { kind: e, method: Some("D3DCompile2"), shader: None, errors: Default::default() })?;
        let source_name = source_name   .as_opt_cstr();
        let entrypoint  = entrypoint    .as_opt_cstr();
        let target      = target        .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();
        let secondary_data_flags = secondary_data_flags.into().into();

        let secondary_data      = secondary_data.into();
        let secondary_data_len  = secondary_data.map_or(0, |sd| sd.len());
        let secondary_data      = secondary_data.map_or(null(), |sd| sd.as_ptr()).cast();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            src_data.as_ptr().cast(), src_data.len(),
            source_name, defines, include, entrypoint, target,
            flags1, flags2, secondary_data_flags, secondary_data, secondary_data_len,
            &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            method: Some("D3DCompile2"),
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })
    }

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
    /// *   Err([THINERR::MISSING_DLL_EXPORT])    - `d3dcompiler_39.dll` and earlier
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let compiler = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let ps = compiler.preprocess(&basic_hlsl, r"test\data\basic.hlsl", (), None).unwrap();
    /// println!("{}", String::from_utf8_lossy(ps.shader.get_buffer()));
    /// ```
    ///
    /// ### Output
    /// ```hlsl
    /// #line 1 "C:\\local\\thindx\\test\\data\\basic.hlsl"
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
    #[requires(d3dcompiler=40)]
    pub fn preprocess<'s>(
        &self,
        src_data:       impl AsRef<[u8]>,
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DPreprocess.ok_or(Error::new("D3DPreprocess", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| Error::new("D3DPreprocess", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name.try_into().map_err(|e| CompileError { kind: e, method: Some("D3DPreprocess"), shader: None, errors: Default::default() })?;
        let source_name = source_name.as_opt_cstr();
        let include     = include.as_id3dinclude();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), source_name, defines, include, &mut shader, &mut errors)};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            method: Some("D3DPreprocess"),
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: CompilerDiagnostics::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })
    }
}
