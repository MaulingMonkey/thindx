use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use std::borrow::Borrow;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::windows::ffi::*;
use std::ops::Deref;
use std::path::*;
use std::ptr::*;



/// { code: [ReadOnlyBlob], errors: [TextBlob] } returned by [Compiler::compile] & friends.
#[derive(Clone, Debug)]
pub struct CompileResult {
    /// The shader bytecode
    pub shader:     CodeBlob,

    /// Any diagnostics, warnings, or non-fatal errors generated while compiling the shader.
    pub errors:     TextBlob,
}

impl AsRef <[u8]> for CompileResult { fn as_ref(&self) -> &[u8] { self.shader.as_bytes() } }
impl Borrow<[u8]> for CompileResult { fn borrow(&self) -> &[u8] { self.shader.as_bytes() } }
impl AsRef <Bytecode> for CompileResult { fn as_ref(&self) -> &Bytecode { self.shader.as_bytecode() } }
impl Borrow<Bytecode> for CompileResult { fn borrow(&self) -> &Bytecode { self.shader.as_bytecode() } }
impl Deref for CompileResult { fn deref(&self) -> &Bytecode { self.shader.as_bytecode() } type Target = Bytecode; }

pub use CompileResult as LinkResult;



/// { code: [TextBlob], errors: [TextBlob] } returned by [Compiler::preprocess]
#[derive(Clone, Debug)]
pub struct PreprocessResult {
    /// The preprocessed HLSL
    pub shader:     TextBlob,

    /// Any diagnostics, warnings, or non-fatal errors generated while preprocessing the shader.
    pub errors:     TextBlob,
}



/// { kind: [ErrorKind], shader: Option&lt;[ReadOnlyBlob]&gt;, errors: [TextBlob] }
#[derive(Clone)]
pub struct CompileError {
    /// The [ErrorKind] / HRESULT generated when compiling the shader.
    pub kind:       ErrorKind,

    /// The method that generated the error in question.
    pub method:     Option<&'static str>,

    /// Any shader bytecode that may have resulted despite compilation failing.
    ///
    /// This might be always [None]?
    pub shader:     Option<CodeBlob>,

    /// More detailed errors/diagnostics beyond the basic error code.
    ///
    /// May be blank for basic API parameter errors, but should be populated for errors in the HLSL code that was being compiled.
    pub errors:     TextBlob,
}

impl From<MethodError> for CompileError {
    fn from(e: MethodError) -> Self {
        Self { kind: e.kind(), method: Some(e.method()), errors: Default::default(), shader: Default::default() }
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



/// { kind: [ErrorKind], shader: Option&lt;[ReadOnlyBlob]&gt;, errors: [TextBlob] }
#[derive(Clone)]
pub struct PreprocessError {
    /// The [ErrorKind] / HRESULT generated when preprocessing the shader.
    pub kind:       ErrorKind,

    /// The method that generated the error in question.
    pub method:     Option<&'static str>,

    /// Any preprocessed HLSL that may have been generated.
    ///
    /// This might be always empty?
    pub shader:     TextBlob,

    /// More detailed errors/diagnostics beyond the basic error code.
    ///
    /// May be blank for basic API parameter errors, but should be populated for errors in the HLSL code that was being preprocessed.
    pub errors:     TextBlob,
}

impl From<MethodError> for PreprocessError {
    fn from(e: MethodError) -> Self {
        Self { kind: e.kind(), method: Some(e.method()), errors: Default::default(), shader: Default::default() }
    }
}

impl std::error::Error for PreprocessError {}

impl Debug for PreprocessError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("PreprocessError")
            .field("kind", &self.kind)
            .field("shader", &self.shader.to_utf8_lossy())
            .field("errors", &self.errors.to_utf8_lossy())
            .finish()
    }
}

impl Display for PreprocessError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Error preprocessing shader: {:?}", self.kind)?;
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
    /// *   [D3DERR::INVALIDCALL]           - on invalid parameters such as nonexistant `target`s
    /// *   [E::FAIL]                       - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let pixel_shader = d3dc.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = d3dc.compile_from_file(
    ///     r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// // TODO: defines/includes examples
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_02_compile]
    ///
    /// ### Remarks
    /// *   You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    // /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions. // ?
    pub fn compile_from_file(
        &self,
        file_name:      impl AsRef<Path>,
        defines:        impl AsShaderMacros,
        include:        impl AsInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompileFromFile.ok_or(MethodError("D3DCompileFromFile", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| MethodError::new("D3DCompileFromFile", e))?;

        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        let entrypoint  = entrypoint.try_into().map_err(|e| MethodError::new("D3DCompileFromFile", e))?;
        let target      = target    .try_into().map_err(|e| MethodError::new("D3DCompileFromFile", e))?;
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
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _).map(|blob| CodeBlob::from_unchecked(blob)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(shader as *mut _)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_39.dll` and earlier
    /// *   [D3DERR::INVALIDCALL]           - on invalid parameters such as nonexistant `target`s
    /// *   [E::FAIL]                       - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = d3dc.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = d3dc.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_02_compile]
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.
    pub fn compile(
        &self,
        src_data:       impl AsRef<[u8]>,
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile.ok_or(MethodError("D3DCompile", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| MethodError::new("D3DCompile", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| MethodError::new("D3DCompile", e))?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| MethodError::new("D3DCompile", e))?;
        let target      = target        .try_into().map_err(|e| MethodError::new("D3DCompile", e))?;
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
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _).map(|blob| CodeBlob::from_unchecked(blob)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(shader as *mut _)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
    /// *   [D3DERR::INVALIDCALL]           - on invalid parameters such as nonexistant `target`s
    /// *   [E::FAIL]                       - if the shader failed to compile
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = d3dc.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    ///
    /// let vertex_shader = d3dc.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_02_compile]
    ///
    /// ### Remarks
    /// The difference between [compile2](Self::compile2) and [compile](Self::compile) is that [compile2](Self::compile2)
    /// takes some optional parameters (`secondary_data_flags` and `secondary_data`) that can be used to control some
    /// aspects of how bytecode is generated.  Refer to the descriptions of these parameters for more details. There is
    /// no difference otherwise to the efficiency of the bytecode generated between [compile2](Self::compile2) and
    /// [compile](Self::compile).
    // #[requires(d3dcompiler=47)] // ?
    pub fn compile2<'s>(
        &self,
        src_data:               impl AsRef<[u8]>,
        source_name:            impl TryIntoAsOptCStr,
        defines:                impl AsShaderMacros,
        include:                impl AsInclude,
        entrypoint:             impl TryIntoAsOptCStr,
        target:                 impl TryIntoAsCStr,
        flags1:                 impl Into<Compile>,
        flags2:                 impl Into<CompileEffect>,
        secondary_data_flags:   impl Into<CompileSecData>,
        secondary_data:         impl Into<Option<&'s [u8]>>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile2.ok_or(MethodError("D3DCompile2", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| MethodError::new("D3DCompile2", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| MethodError::new("D3DCompile2", e))?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| MethodError::new("D3DCompile2", e))?;
        let target      = target        .try_into().map_err(|e| MethodError::new("D3DCompile2", e))?;
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
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _).map(|blob| CodeBlob::from_unchecked(blob)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(CompileResult {
            shader: unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(shader as *mut _)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_39.dll` and earlier
    /// *   [E::FAIL]                       - if the shader failed to preprocess
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let ps = d3dc.preprocess(&basic_hlsl, r"test\data\basic.hlsl", (), None).unwrap();
    /// println!("{}", ps.shader.to_utf8_lossy());
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
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_02_compile]
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.
    pub fn preprocess(
        &self,
        src_data:       impl AsRef<[u8]>,
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsInclude,
    ) -> Result<PreprocessResult, PreprocessError> {
        // Early outs
        let f           = self.D3DPreprocess.ok_or(MethodError("D3DPreprocess", THINERR::MISSING_DLL_EXPORT))?;
        let defines     = defines.as_shader_macros().map_err(|e| MethodError::new("D3DPreprocess", e))?;

        let src_data    = src_data.as_ref();
        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name.try_into().map_err(|e| MethodError::new("D3DPreprocess", e))?;
        let source_name = source_name.as_opt_cstr();
        let include     = include.as_id3dinclude();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), source_name, defines, include, &mut shader, &mut errors)};
        ErrorKind::check(hr).map_err(|kind| PreprocessError {
            kind,
            method: Some("D3DPreprocess"),
            shader: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) }),
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })?;

        Ok(PreprocessResult {
            shader: TextBlob::new(unsafe { ReadOnlyBlob::from_raw(shader as *mut _) }),
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) }),
        })
    }
}
