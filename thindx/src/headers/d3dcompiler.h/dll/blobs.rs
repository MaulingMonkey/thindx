use crate::*;
use crate::d3d::*;

use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;

/// <h1 id="blobs" class="section-header"><a href="#blobs">ReadOnlyBlob Utilities</a></h1>
impl Compiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreateblob)\]
    /// D3DCreateBlob
    ///
    /// Compresses a set of shaders into a more compact form.
    ///
    /// ### Arguments
    /// *   `data`          - The data to create a [ReadOnlyBlob] out of.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_42.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::new(47).unwrap();
    /// let blob = d3dc.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// assert_eq!(blob.get_buffer_size(),  4           );
    /// assert_eq!(blob.get_buffer(),       &[1,2,3,4]  );
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn create_read_only_blob(&self, data: &[u8]) -> Result<ReadOnlyBlob, MethodError> {
        let f = self.D3DCreateBlob.ok_or(MethodError::new("D3DCreateBlob", THINERR::MISSING_DLL_EXPORT))?;

        let mut blob = null_mut();
        let hr = unsafe { f(data.len(), &mut blob) };
        MethodError::check("D3DCreateBlob", hr)?;

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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    /// *   [ERROR::FILE_NOT_FOUND]         - `file_name` not found
    /// *   [ERROR::ACCESS_DENIED]          - Access denied (`file_name` not a file? bad perms? ...?)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*};
    /// # use winapi::shared::winerror::*;
    /// # let d3dc = Compiler::new(47).unwrap();
    /// let blob : ReadOnlyBlob = d3dc.read_file_to_blob(r"test\data\basic.hlsl").unwrap();
    ///
    /// let err = d3dc.read_file_to_blob(r"test\data\nonexistant").err().unwrap();
    /// assert_eq!(err.kind(), ErrorKind::from_win32(ERROR_FILE_NOT_FOUND));
    ///
    /// let err = d3dc.read_file_to_blob(r"test\data").err().unwrap();
    /// assert_eq!(err.kind(), ErrorKind::from_win32(ERROR_ACCESS_DENIED));
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn read_file_to_blob(&self, file_name: impl AsRef<Path>) -> Result<ReadOnlyBlob, MethodError> {
        let f = self.D3DReadFileToBlob.ok_or(MethodError::new("D3DReadFileToBlob", THINERR::MISSING_DLL_EXPORT))?;
        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        let mut blob = null_mut();
        let hr = unsafe { f(file_name.as_ptr(), &mut blob) };
        MethodError::check("D3DReadFileToBlob", hr)?;
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
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    /// *   [ERROR::PATH_NOT_FOUND]         - Path not found (missing filename in `file_name`)
    /// *   [ERROR::ACCESS_DENIED]          - Access denied (target `file_name` is a directory?)
    /// *   [ERROR::FILE_EXISTS]            - `file_name` already exists (if `!overwrite`)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*};
    /// # use winapi::shared::winerror::*;
    /// # let d3dc = Compiler::new(47).unwrap();
    /// let blob = d3dc.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// d3dc.write_blob_to_file(&blob, r"..\target\1234.bin", true).unwrap();
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target\1234.bin", false).unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::from_win32(ERROR_FILE_EXISTS));
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target\", false).unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::from_win32(ERROR_PATH_NOT_FOUND));
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target", false).unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::from_win32(ERROR_ACCESS_DENIED));
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn write_blob_to_file(
        &self,
        blob:       &ReadOnlyBlob,
        file_name:  impl AsRef<Path>,
        overwrite:  bool,
    ) -> Result<(), MethodError> {
        let f = self.D3DWriteBlobToFile.ok_or(MethodError::new("D3DWriteBlobToFile", THINERR::MISSING_DLL_EXPORT))?;
        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
        let hr = unsafe { f(blob.as_raw(), file_name.as_ptr(), overwrite.into()) };
        MethodError::check("D3DWriteBlobToFile", hr)
    }
}
