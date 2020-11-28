use crate::*;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreateblob)\]
    /// D3DCreateBlob
    ///
    /// Compresses a set of shaders into a more compact form.
    ///
    /// ### Arguments
    /// *   `data`          - The data to create a [ReadOnlyBlob] out of.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_42.dll` and earlier
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="43"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_43.dll"))]
    pub fn create_read_only_blob(&self, data: &[u8]) -> Result<ReadOnlyBlob, ErrorKind> {
        // Early outs
        let f           = self.D3DCreateBlob.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut blob = null_mut();
        let hr = unsafe { f(data.len(), &mut blob) };
        ErrorKind::check(hr)?;

        if !blob.is_null() {
            let dst = unsafe { (*blob).GetBufferPointer() };
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), dst.cast(), data.len()) };
        }

        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
