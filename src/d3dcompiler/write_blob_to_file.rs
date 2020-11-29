use crate::*;

use std::os::windows::ffi::*;
use std::path::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dwriteblobtofile)\]
    /// D3DWriteBlobToFile
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    ///
    /// Writes a memory blob to a file on disk.
    ///
    /// ### Arguments
    /// *   `blob`          - The blob of data to write to disk.
    /// *   `file_name`     - The path to write it to.
    /// *   `overwrite`     - Overwrite any existing file.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_43.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == ???                                - file exists, cannot be written, etc.
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let blob = compiler.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// compiler.write_blob_to_file(&blob, r"target\1234.bin", true).unwrap();
    /// compiler.write_blob_to_file(&blob, r"target\1234.bin", false).unwrap_err();
    /// ```
    ///
    /// <div class="note"><b>Note:</b>  The D3dcompiler_44.dll or later version of the file contains the D3DWriteBlobToFile compiler function.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::write_blob_to_file wasn't added until d3dcompiler_44.dll"))]
    pub fn write_blob_to_file<'s>(
        &self,
        blob:       &ReadOnlyBlob,
        file_name:  impl AsRef<Path>,
        overwrite:  bool,
    ) -> Result<(), Error> {
        let f = self.D3DWriteBlobToFile.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
        let hr = unsafe { f(blob.as_raw(), file_name.as_ptr(), overwrite.into()) };
        Error::check("D3DWriteBlobToFile", hr)
    }
}
