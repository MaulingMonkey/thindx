use crate::*;
use crate::d3d::*;

use std::ptr::*;



/// <h1 id="parts" class="section-header"><a href="#parts">Manipulate Bytecode by BlobPart</a></h1>
impl Compiler {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetblobpart)\]
    /// D3DGetBlobPart
    ///
    /// Retrieves a specific part from a compilation result.
    ///
    /// ### Arguments
    /// *   `src_data`      - Compiled HLSL code.
    /// *   `part`          - The [BlobPart] to retrieve
    /// *   `flags`         - Reserved (pass [None])
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - on `d3dcompiler_42.dll` and earlier
    /// *   [D3DERR::INVALIDCALL]           - on data that wasn't compiled shader code.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let shader2 = d3dc.set_blob_part(
    ///     &shader, Blob::PrivateData, (), b"testing 123"
    /// ).unwrap();
    ///
    /// assert_eq!(b"testing 123", d3dc.get_blob_part(
    ///     &shader2, Blob::PrivateData, None
    /// ).unwrap().as_bytes());
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn get_blob_part(&self, src_data: &Bytecode, part: impl Into<BlobPart>, flags: Option<std::convert::Infallible>) -> Result<BytesBlob, Error> {
        fn_context_dll!(d3d::Compiler::get_blob_part => self.D3DGetBlobPart);
        let src_data = src_data.as_bytes();
        let part = part.into().into();
        let _ = flags;
        let mut blob = null_mut();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `part`        ⚠️ could be invalid
        //  * `flags`       ✔️ are reserved/0
        //  * `blob`        ✔️ is a simple out-param
        fn_check_hr!(unsafe { D3DGetBlobPart(src_data.as_ptr().cast(), src_data.len(), part, 0, &mut blob) })?;

        // SAFETY: ✔️ `blob` is null (from_raw panics) or a valid non-dangling pointer (from_raw takes ownership).  ReadOnlyBlob imposes no content requirements.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetdebuginfo)\]
    /// D3DGetDebugInfo
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    ///
    /// Gets shader debug information.
    ///
    /// ### Arguments
    /// *   `src_data`  - Shader bytecode
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_39.dll` and earlier
    /// *   [E::FAIL]                       - `src_data` wasn't compiled with [d3d::Compile::Debug]
    /// *   [E::FAIL]                       - `src_data`'s debug info is too modern? (cannot load debug info for shaders compiled with `d3dcompiler_47.dll`)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # if d3d::Compiler::load_system(43).is_err() { return; }
    /// let d3dc = d3d::Compiler::load_system(43).unwrap();
    /// # let shader_src = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// let debug_info : d3d::BytesBlob = d3dc.get_debug_info(&shader).unwrap();
    /// assert!(debug_info.len() > 0);
    /// #
    /// # // This is not legal, despite MSDN suggesting that src_data can contain "either uncompiled or compiled HLSL code" (didn't sound right)
    /// # assert_eq!(E::FAIL, d3dc.get_debug_info(unsafe { d3d::Bytecode::from_unchecked(&shader_src) }), "uncompiled shader code isn't actually legal");
    /// #
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::None, d3d::CompileEffect::None).unwrap();
    /// # assert_eq!(E::FAIL, d3dc.get_debug_info(&shader), "shader was compiled without debug info");
    /// #
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # assert_eq!(E::FAIL, d3dc.get_debug_info(&shader), "shader was compiled with d3dcompiler_47.dll");
    /// ```
    ///
    /// ### Output
    /// ```text
    /// BytesBlob(2625 bytes)
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.
    pub fn get_debug_info(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        fn_context_dll!(d3d::Compiler::get_debug_info => self.D3DGetDebugInfo);
        let src_data = src_data.as_bytes();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `blob`        ✔️ is a simple out-param
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DGetDebugInfo(src_data.as_ptr().cast(), src_data.len(), &mut blob) })?;

        // SAFETY: ✔️ `blob` is null (from_raw panics) or a valid non-dangling pointer (from_raw takes ownership).  ReadOnlyBlob imposes no content requirements.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetinputandoutputsignatureblob)\]
    /// D3DGetInputAndOutputSignatureBlob
    ///
    /// > **Note:** [get_input_and_output_signature_blob](Self::get_input_and_output_signature_blob) may be altered or
    /// > unavailable for releases after Windows 8.1. Instead use [get_glob_part](Self::get_blob_part) with the
    /// > [Blob::InputAndOutputSignatureBlob] value.
    ///
    /// Gets the input and output signatures from a compilation result.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let signature = d3dc.get_input_and_output_signature_blob(&shader).unwrap();
    /// println!("{:?}", &signature);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [68, 88, 66, 67, 97, ...
    /// ```
    // #[requires(d3dcompiler=33)] // or earlier?
    //#allow_missing_argument_docs
    pub fn get_input_and_output_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        fn_context_dll!(d3d::Compiler::get_input_and_output_signature_blob => self.D3DGetInputAndOutputSignatureBlob);
        let src_data = src_data.as_bytes();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `blob`        ✔️ is a simple out-param
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DGetInputAndOutputSignatureBlob(src_data.as_ptr().cast(), src_data.len(), &mut blob) })?;

        // SAFETY: ✔️ `blob` is null (from_raw panics) or a valid non-dangling pointer (from_raw takes ownership).  ReadOnlyBlob imposes no content requirements.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetinputsignatureblob)\]
    /// D3DGetInputSignatureBlob
    ///
    /// > **Note:** [get_input_signature_blob](Self::get_input_signature_blob) may be altered or
    /// > unavailable for releases after Windows 8.1. Instead use [get_glob_part](Self::get_blob_part) with the
    /// > [Blob::InputSignatureBlob] value.
    ///
    /// Gets the input signature from a compilation result.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let signature = d3dc.get_input_signature_blob(&shader).unwrap();
    /// println!("{:?}", &signature);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [68, 88, 66, 67, 53, ...
    /// ```
    // #[requires(d3dcompiler=33)] // or earlier?
    //#allow_missing_argument_docs
    pub fn get_input_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        fn_context_dll!(d3d::Compiler::get_input_signature_blob => self.D3DGetInputSignatureBlob);
        let src_data = src_data.as_bytes();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `blob`        ✔️ is a simple out-param
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DGetInputSignatureBlob(src_data.as_ptr().cast(), src_data.len(), &mut blob) })?;

        // SAFETY: ✔️ `blob` is null (from_raw panics) or a valid non-dangling pointer (from_raw takes ownership).  ReadOnlyBlob imposes no content requirements.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetoutputsignatureblob)\]
    /// D3DGetOutputSignatureBlob
    ///
    /// > **Note:** [get_output_signature_blob](Self::get_output_signature_blob) may be altered or
    /// > unavailable for releases after Windows 8.1. Instead use [get_glob_part](Self::get_blob_part) with the
    /// > [Blob::OutputSignatureBlob] value.
    ///
    /// Gets the output signature from a compilation result.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let signature = d3dc.get_output_signature_blob(&shader).unwrap();
    /// println!("{:?}", &signature);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [68, 88, 66, 67, 210, ...
    /// ```
    // #[requires(d3dcompiler=33)] // or earlier?
    //#allow_missing_argument_docs
    pub fn get_output_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        fn_context_dll!(d3d::Compiler::get_output_signature_blob => self.D3DGetOutputSignatureBlob);
        let src_data = src_data.as_bytes();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `blob`        ✔️ is a simple out-param
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DGetOutputSignatureBlob(src_data.as_ptr().cast(), src_data.len(), &mut blob) })?;

        // SAFETY: ✔️ `blob` is null (from_raw panics) or a valid non-dangling pointer (from_raw takes ownership).  ReadOnlyBlob imposes no content requirements.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dsetblobpart)\]
    /// D3DSetBlobPart
    ///
    /// Sets information in a compilation result.
    ///
    /// ### Arguments
    /// *   `src_data`  - The original compiled shader data.
    /// *   `part`      - What part to replace (currently only [Blob::PrivateData] is supported.)
    /// *   `flags`     - Resereved.  Pass `()`.
    /// *   `part_data` - The part data to set.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let shader2 = d3dc.set_blob_part(
    ///     &shader, Blob::PrivateData, (), b"testing 123"
    /// ).unwrap();
    ///
    /// assert_eq!(b"testing 123", d3dc.get_blob_part(
    ///     &shader2, Blob::PrivateData, None
    /// ).unwrap().as_bytes());
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn set_blob_part(
        &self,
        src_data:           &Bytecode,
        part:               impl Into<BlobPart>,
        flags:              (),
        part_data:          &[u8],
    ) -> Result<CodeBlob, Error> {
        fn_context_dll!(d3d::Compiler::set_blob_part => self.D3DSetBlobPart);
        let src_data = src_data.as_bytes();
        let _ = flags;

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `blob`        ✔️ is a simple out-param
        //  * `part`        ⚠️ could be invalid
        //  * `part_data`   ❌ could be invalid for `part`
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DSetBlobPart(src_data.as_ptr().cast(), src_data.len(), part.into().into(), 0, part_data.as_ptr().cast(), part_data.len(), &mut blob) })?;

        // SAFETY: ⚠️
        //  * `blob`        ✔️ should be null (from_raw panics) or a valid non-dangling blob (from_raw takes ownership)
        //  * `blob`        ⚠️ should be a shader blob, but invalid params above could violate CodeBlob's precondition (of the blob being valid DXBC or DXIL bytecode)
        Ok(unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(blob)) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dstripshader)\]
    /// D3DStripShader
    ///
    /// Removes unwanted blobs from a compilation result.
    ///
    /// ### Arguments
    /// *   `src_data`          - The original shader bytecode.
    /// *   `strip_flags`       - The [CompilerStripFlags] to use.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_39.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let shader = d3dc.strip_shader(
    ///     &shader, CompilerStripFlags::DebugInfo
    /// ).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.
    pub fn strip_shader(
        &self,
        src_data:           &Bytecode,
        strip_flags:        impl Into<CompilerStripFlags>,
    ) -> Result<CodeBlob, Error> {
        fn_context_dll!(d3d::Compiler::strip_shader => self.D3DStripShader);
        let mut blob = null_mut();

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           ✔️ should be valid/sound like all `self.*`
        //  * `src_data`    ❌ needs fuzz testing against ~4GB data to attempt to induce alloc overflow bugs
        //  * `src_data`    ✔️ should be valid bytecode as implied by [`Bytecode`]
        //  * `strip_flags` ⚠️ could be invalid
        //  * `blob`        ✔️ is a simple out-param
        fn_check_hr!(unsafe { D3DStripShader(src_data.as_ptr().cast(), src_data.len(), strip_flags.into().into(), &mut blob) })?;

        // SAFETY: ⚠️
        //  * `blob`        ✔️ should be null (from_raw panics) or a valid non-dangling blob (from_raw takes ownership)
        //  * `blob`        ⚠️ should be a shader blob, but invalid params above could violate CodeBlob's precondition (of the blob being valid DXBC or DXIL bytecode)
        Ok(unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(blob)) })
    }
}
