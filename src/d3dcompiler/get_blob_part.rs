use crate::*;

use std::ptr::*;



impl D3DCompiler {
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
    /// ### Returns
    /// *   Ok([ReadOnlyBlob])                  - on success
    /// *   Err([Error]) with `error.kind()` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT] - on `d3dcompiler_42.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]           - on data that wasn't compiled shader code.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader : ReadOnlyBlob = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let shader2 = compiler.set_blob_part(
    ///     shader.get_buffer(), Blob::PrivateData, (), b"testing 123"
    /// ).unwrap();
    ///
    /// assert_eq!(b"testing 123", compiler.get_blob_part(
    ///     shader2.get_buffer(), Blob::PrivateData, None
    /// ).unwrap().get_buffer());
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="43"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_43.dll"))]
    pub fn get_blob_part(&self, src_data: &[u8], part: impl Into<BlobPart>, flags: Option<void::Void>) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DGetBlobPart.ok_or(Error::new("D3DGetBlobPart", ErrorKind::MISSING_DLL_EXPORT))?;
        let part = part.into().into();
        let _ = flags; let flags = 0;
        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), part, flags, &mut blob) };
        Error::check("D3DGetBlobPart", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
