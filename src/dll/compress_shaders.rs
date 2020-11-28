use crate::*;
use crate::traits::Raw;

use std::convert::TryInto;
use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders)\]
    /// D3DCompressShaders
    ///
    /// Compresses a set of shaders into a more compact form.
    ///
    /// ### Arguments
    /// *   `shaders`       - The compiled shader(s) to compress.
    /// *   `flags`         - [CompressShader] flags controlling compression.
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
    /// <div class="note"><b>Note:</b> You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.</div>
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="43"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_43.dll"))]
    pub fn compress_shaders(
        &self,
        shaders:                &[ShaderData],
        flags:                  impl Into<CompressShader>,
    ) -> Result<ReadOnlyBlob, ErrorKind> {
        // Early outs
        let f           = self.D3DCompressShaders.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let num_shaders = shaders.len().try_into().map_err(|_| ErrorKind::MISSING_DLL_EXPORT)?;

        let shader_data = shaders.as_ptr() as *mut _; // TODO: Is the `mut` sane?
        let flags       = flags.into().into();

        let mut compressed_data = null_mut();
        let hr = unsafe { f(num_shaders, shader_data, flags, &mut compressed_data) };
        ErrorKind::check(hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(compressed_data) })
        // TODO: Wait, this takes multiple shaders.  Does this also *return* an array of blobs, perhaps?
    }
}
