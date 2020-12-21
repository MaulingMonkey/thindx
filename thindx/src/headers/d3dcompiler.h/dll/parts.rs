use crate::*;
use crate::d3d::*;

use std::ptr::*;



/// <h1 id="parts" class="section-header"><a href="#parts">Manipulate Bytecode by BlobPart</a></h1>
impl Compiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetblobpart)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let shader2 = d3dc.set_blob_part(
    ///     &shader, Blob::PrivateData, (), b"testing 123"
    /// ).unwrap();
    ///
    /// assert_eq!(b"testing 123", d3dc.get_blob_part(
    ///     &shader2, Blob::PrivateData, None
    /// ).unwrap().as_bytes());
    /// ```
    #[requires(d3dcompiler=43)]
    pub fn get_blob_part(&self, src_data: &Bytecode, part: impl Into<BlobPart>, flags: Option<void::Void>) -> Result<BytesBlob, Error> {
        let f = self.D3DGetBlobPart.ok_or(Error::new("D3DGetBlobPart", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();
        let part = part.into().into();
        let _ = flags; let flags = 0;
        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), part, flags, &mut blob) };
        Error::check("D3DGetBlobPart", hr)?;
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetdebuginfo)\]
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
    /// let d3dc = d3d::Compiler::new(43).unwrap();
    /// # let shader_src = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// let debug_info : d3d::BytesBlob = d3dc.get_debug_info(&shader).unwrap();
    /// assert!(debug_info.len() > 0);
    /// #
    /// # // This is not legal, despite MSDN suggesting that src_data can contain "either uncompiled or compiled HLSL code" (didn't sound right)
    /// # assert_eq!(d3dc.get_debug_info(unsafe { d3d::Bytecode::from_unchecked(&shader_src) }).err().map(|err| err.kind()), Some(E::FAIL), "uncompiled shader code isn't actually legal");
    /// #
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::None, d3d::CompileEffect::None).unwrap();
    /// # assert_eq!(d3dc.get_debug_info(&shader).err().map(|err| err.kind()), Some(E::FAIL), "shader was compiled without debug info");
    /// #
    /// # let d3dc = d3d::Compiler::new(47).unwrap();
    /// # let shader = d3dc.compile(&shader_src, cstr!(r"test\data\basic.hlsl"), None, None, "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
    /// # assert_eq!(d3dc.get_debug_info(&shader).err().map(|err| err.kind()), Some(E::FAIL), "shader was compiled with d3dcompiler_47.dll");
    /// ```
    ///
    /// ### Output
    /// ```text
    /// BytesBlob(2625 bytes)
    /// ```
    #[requires(d3dcompiler=40)]
    pub fn get_debug_info(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        let f = self.D3DGetDebugInfo.ok_or(Error::new("D3DGetDebugInfo", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetDebugInfo", hr)?;
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetinputandoutputsignatureblob)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
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
    #[xallow(missing_argument_docs)]
    pub fn get_input_and_output_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        let f = self.D3DGetInputAndOutputSignatureBlob.ok_or(Error::new("D3DGetInputAndOutputSignatureBlob", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetInputAndOutputSignatureBlob", hr)?;
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetinputsignatureblob)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
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
    #[xallow(missing_argument_docs)]
    pub fn get_input_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        let f = self.D3DGetInputSignatureBlob.ok_or(Error::new("D3DGetInputSignatureBlob", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetInputSignatureBlob", hr)?;
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetoutputsignatureblob)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
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
    #[xallow(missing_argument_docs)]
    pub fn get_output_signature_blob(&self, src_data: &Bytecode) -> Result<BytesBlob, Error> {
        let f = self.D3DGetOutputSignatureBlob.ok_or(Error::new("D3DGetOutputSignatureBlob", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetOutputSignatureBlob", hr)?;
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dsetblobpart)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let shader2 = d3dc.set_blob_part(
    ///     &shader, Blob::PrivateData, (), b"testing 123"
    /// ).unwrap();
    ///
    /// assert_eq!(b"testing 123", d3dc.get_blob_part(
    ///     &shader2, Blob::PrivateData, None
    /// ).unwrap().as_bytes());
    /// ```
    #[requires(d3dcompiler=44)]
    pub fn set_blob_part<'s>(
        &self,
        src_data:           &Bytecode,
        part:               impl Into<BlobPart>,
        flags:              (),
        part_data:          &[u8],
    ) -> Result<CodeBlob, Error> {
        let f = self.D3DSetBlobPart.ok_or(Error::new("D3DSetBlobPart", THINERR::MISSING_DLL_EXPORT))?;
        let src_data = src_data.as_bytes();

        let _ = flags; let flags = 0;
        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), part.into().into(), flags, part_data.as_ptr().cast(), part_data.len(), &mut blob) };
        Error::check("D3DSetBlobPart", hr)?;
        Ok(unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(blob)) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dstripshader)\]
    /// D3DStripShader
    ///
    /// Removes unwanted blobs from a compilation result.
    ///
    /// ### Arguments
    /// *   `shader_bytecode`   - The original shader bytecode.
    /// *   `strip_flags`       - The [CompilerStripFlags] to use.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_39.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let shader = d3dc.strip_shader(
    ///     &shader, CompilerStripFlags::DebugInfo
    /// ).unwrap();
    /// ```
    #[requires(d3dcompiler=40)]
    pub fn strip_shader<'s>(
        &self,
        shader_bytecode:    &[u8],
        strip_flags:        impl Into<CompilerStripFlags>,
    ) -> Result<CodeBlob, Error> {
        let f = self.D3DStripShader.ok_or(Error::new("D3DStripShader", THINERR::MISSING_DLL_EXPORT))?;
        let mut blob = null_mut();
        let hr = unsafe { f(shader_bytecode.as_ptr().cast(), shader_bytecode.len(), strip_flags.into().into(), &mut blob) };
        Error::check("D3DStripShader", hr)?;
        Ok(unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(blob)) })
    }
}
