use crate::*;

use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreadfiletoblob)\]
    /// D3DReadFileToBlob
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    ///
    /// Reads a file into memory.
    ///
    /// ### Arguments
    /// *   `file_name`         - The path to read the blob from.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_43.dll` and earlier
    /// *   Err(`e`)                                                        - if the file doesn't exist, can't be read, etc.
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let blob : ReadOnlyBlob = compiler.read_file_to_blob(r"test\data\basic.hlsl").unwrap();
    ///
    /// assert!(compiler.read_file_to_blob(r"test\data\nonexistant").is_err(), "file shouldn't exist");
    /// assert!(compiler.read_file_to_blob(r"test\data").is_err(), "can't read folder");
    /// ```
    ///
    /// <div class="note"><b>Note:</b>  The D3dcompiler_44.dll or later version of the file contains the D3DReadFileToBlob compiler function.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::read_file_to_blob wasn't added until d3dcompiler_44.dll"))]
    pub fn read_file_to_blob<'s>(&self, file_name: impl AsRef<Path>) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DReadFileToBlob.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        let mut blob = null_mut();
        let hr = unsafe { f(file_name.as_ptr(), &mut blob) };
        Error::check("D3DReadFileToBlob", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
