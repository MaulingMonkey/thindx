use crate::*;
use crate::d3d::*;

use std::path::*;
use std::ptr::*;

/// <h1 id="blobs" class="section-header"><a href="#blobs">ReadOnlyBlob Utilities</a></h1>
impl Compiler {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreateblob)\]
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
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// let blob = d3dc.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// assert_eq!(blob.get_buffer_size(),  4           );
    /// assert_eq!(blob.get_buffer(),       &[1,2,3,4]  );
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn create_read_only_blob(&self, data: &[u8]) -> Result<ReadOnlyBlob, Error> {
        fn_context_dll!(d3d::Compiler::create_read_only_blob => self.D3DCreateBlob);

        // SAFETY: ❌ needs fuzz testing against ~4GB `data` to attempt to induce alloc overflow bugs
        //  * `f`           should be valid/sound like all `self.*`
        //  * `blob`        is a trivial out param.
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DCreateBlob(data.len(), &mut blob) })?;

        if !blob.is_null() {
            // SAFETY: ⚠️ see earlier safety comments about ~4GB overflow concerns
            //  * `T`       is `u8`, no lifetime/drop concerns etc.
            //  * `blob`    is non-null, non-dangling, and should be a valid ID3DBlob
            //  * read      is within `data`'s range
            //  * write     *should* be within `blob`'s range, unless `D3DCreateBlob` earlier has overflow bugs
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), (*blob).GetBufferPointer().cast(), data.len()) };
        }

        // SAFETY: ✔️
        //  * `blob`            should be null (panics) or a valid pointer.
        //  * `ReadOnlyBlob`    imposes no restrictions on `blob`'s contents.
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreadfiletoblob)\]
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
    /// *   [HResultError::from_win32](winresult::HResultError::from_win32):
    ///     *   [ERROR::FILE_NOT_FOUND]         - `file_name` not found
    ///     *   [ERROR::ACCESS_DENIED]          - Access denied (`file_name` not a file? bad perms? ...?)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*};
    /// # use winresult::*;
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// let blob : ReadOnlyBlob = d3dc.read_file_to_blob(r"test\data\basic.hlsl").unwrap();
    ///
    /// let err = d3dc.read_file_to_blob(r"test\data\nonexistant");
    /// assert_eq!(err, HResultError::from_win32(ERROR::FILE_NOT_FOUND));
    ///
    /// let err = d3dc.read_file_to_blob(r"test\data");
    /// assert_eq!(err, HResultError::from_win32(ERROR::ACCESS_DENIED));
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn read_file_to_blob(&self, file_name: impl AsRef<Path>) -> Result<ReadOnlyBlob, Error> {
        fn_context_dll!(d3d::Compiler::read_file_to_blob => self.D3DReadFileToBlob);
        let file_name = file_name.to_wcstr().map_err(|e| fn_param_error!(file_name, e))?;

        // SAFETY: ❌ needs fuzz testing against ~4GB files to attempt to induce alloc overflow bugs
        //  * `f`           should be valid/sound like all `self.*`
        //  * `file_name`   is `\0` terminated per `to_wcstr`.
        //  * `blob`        is a trivial out param.
        let mut blob = null_mut();
        fn_check_hr!(unsafe { D3DReadFileToBlob(file_name.as_ptr(), &mut blob) })?;

        // SAFETY: ✔️
        //  * `blob`            should be null (panics) or non-dangling, and should be a valid ID3DBlob
        //  * `ReadOnlyBlob`    imposes no restrictions on `blob`'s contents.
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dwriteblobtofile)\]
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    /// *   [HResultError::from_win32](winresult::HResultError::from_win32):
    ///     *   [ERROR::PATH_NOT_FOUND]         - Path not found (missing filename in `file_name`)
    ///     *   [ERROR::ACCESS_DENIED]          - Access denied (target `file_name` is a directory?)
    ///     *   [ERROR::FILE_EXISTS]            - `file_name` already exists (if `!overwrite`)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*};
    /// # use winresult::*;
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// let blob = d3dc.create_read_only_blob(&[1,2,3,4]).unwrap();
    /// d3dc.write_blob_to_file(&blob, r"..\target\1234.bin", true).unwrap();
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target\1234.bin", false);
    /// assert_eq!(err, HResultError::from_win32(ERROR::FILE_EXISTS));
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target\", false);
    /// assert_eq!(err, HResultError::from_win32(ERROR::PATH_NOT_FOUND));
    ///
    /// let err = d3dc.write_blob_to_file(&blob, r"..\target", false);
    /// assert_eq!(err, HResultError::from_win32(ERROR::ACCESS_DENIED));
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn write_blob_to_file(
        &self,
        blob:       &ReadOnlyBlob,
        file_name:  impl AsRef<Path>,
        overwrite:  bool,
    ) -> Result<(), Error> {
        fn_context_dll!(d3d::Compiler::write_blob_to_file => self.D3DWriteBlobToFile);
        let file_name = file_name.to_wcstr().map_err(|e| fn_param_error!(file_name, e))?;

        // SAFETY: ✔️
        //  * `f`           should be valid/sound like all `self.*`
        //  * `file_name`   is `\0` terminated per `to_wcstr`.
        //  * `overwrite`   is a trivial well tested bool.
        fn_check_hr!(unsafe { D3DWriteBlobToFile(blob.as_raw(), file_name.as_ptr(), overwrite.into()) })
    }
}

//TODO:     D3DCreateBlob                           = d3d::Compiler::create_blob
//#cpp2rust D3DCreateBlob                           = d3d::Compiler::create_read_only_blob
//#cpp2rust D3DReadFileToBlob                       = d3d::Compiler::read_file_to_blob
//#cpp2rust D3DWriteBlobToFile                      = d3d::Compiler::write_blob_to_file
