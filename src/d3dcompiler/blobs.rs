use crate::*;

use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;

/// <h1 id="blobs" class="section-header"><a href="#blobs">ReadOnlyBlob Utilities</a></h1>
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
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let blob = compiler.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// assert_eq!(blob.get_buffer_size(),  4           );
    /// assert_eq!(blob.get_buffer(),       &[1,2,3,4]  );
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
    /// assert!(compiler.read_file_to_blob(r"test\data\nonexistant").is_err(), "shouldn't exist");
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
