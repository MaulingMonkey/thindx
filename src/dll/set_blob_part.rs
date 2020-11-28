use crate::*;

use std::ptr::*;



impl D3DCompiler {
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
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_43.dll` and earlier
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader : ReadOnlyBlob = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// compiler.set_blob_part(shader.get_buffer(), Blob::PrivateData, (), "testing 123".as_bytes()).unwrap();
    /// ```
    ///
    /// <div class="note"><b>Note:</b>  The D3dcompiler_44.dll or later version of the file contains the D3DSetBlobPart compiler function.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::set_blob_part wasn't added until d3dcompiler_44.dll"))]
    pub fn set_blob_part<'s>(
        &self,
        src_data:           &[u8],
        part:               impl Into<BlobPart>,
        flags:              (),
        part_data:          &[u8],
    ) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DSetBlobPart.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let _ = flags; let flags = 0;
        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), part.into().into(), flags, part_data.as_ptr().cast(), part_data.len(), &mut blob) };
        Error::check("D3DSetBlobPart", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
