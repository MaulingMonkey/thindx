use crate::*;

use std::ptr::*;

impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgetinputandoutputsignatureblob)\]
    /// D3DGetInputAndOutputSignatureBlob
    ///
    /// > **Note:** [get_input_and_output_signature_blob](Self::get_input_and_output_signature_blob) may be altered or
    /// > unavailable for releases after Windows 8.1. Instead use [get_glob_part](Self::get_blob_part) with the
    /// > [Blob::InputAndOutputSignatureBlob] value.
    ///
    /// Gets the input and output signatures from a compilation result.
    ///
    /// ### Returns
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let signature = compiler.get_input_and_output_signature_blob(shader.get_buffer()).unwrap();
    /// println!("{:?}", signature.get_buffer());
    /// ```
    ///
    /// ### Outputs
    /// ```text
    /// [68, 88, 66, 67, 97, ...
    /// ```
    pub fn get_input_and_output_signature_blob(&self, src_data: &[u8]) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DGetInputAndOutputSignatureBlob.ok_or(Error::new("D3DGetInputAndOutputSignatureBlob", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetInputAndOutputSignatureBlob", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
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
    /// ### Returns
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let signature = compiler.get_input_signature_blob(shader.get_buffer()).unwrap();
    /// println!("{:?}", signature.get_buffer());
    /// ```
    ///
    /// ### Outputs
    /// ```text
    /// [68, 88, 66, 67, 53, ...
    /// ```
    pub fn get_input_signature_blob(&self, src_data: &[u8]) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DGetInputSignatureBlob.ok_or(Error::new("D3DGetInputSignatureBlob", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetInputSignatureBlob", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
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
    /// ### Returns
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", (), (), "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let signature = compiler.get_output_signature_blob(shader.get_buffer()).unwrap();
    /// println!("{:?}", signature.get_buffer());
    /// ```
    ///
    /// ### Outputs
    /// ```text
    /// [68, 88, 66, 67, 210, ...
    /// ```
    pub fn get_output_signature_blob(&self, src_data: &[u8]) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DGetOutputSignatureBlob.ok_or(Error::new("D3DGetOutputSignatureBlob", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut blob = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &mut blob) };
        Error::check("D3DGetOutputSignatureBlob", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
    }
}
