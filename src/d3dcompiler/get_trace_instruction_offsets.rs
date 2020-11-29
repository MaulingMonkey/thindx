use crate::*;

use std::ptr::*;



impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the number of byte offsets for instructions within a section of shader code.
    ///
    /// ### Returns
    /// *   Ok([usize])                             - the number of available trace instruction offsets in that range
    /// *   Err([Error]) where `error.kind()` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// println!("{}", compiler.get_trace_instruction_offsets_count(
    ///     shader.get_buffer(), GetInstOffsets::None, 0, std::usize::MAX
    /// ).unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// 2
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_44.dll"))]
    pub fn get_trace_instruction_offsets_count(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<usize, Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", ErrorKind::MISSING_DLL_EXPORT))?;
        let mut n = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags.into().into(), start_inst_index, num_insts, null_mut(), &mut n) };
        Error::check("D3DGetTraceInstructionOffsets", hr)?;
        Ok(n)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the byte offsets for instructions within a section of shader code.
    ///
    /// ### Returns
    /// *   Ok(&amp;\[[usize]\])                    - the trace instruction offsets in `offsets` that were read
    /// *   Err([Error]) where `error.kind()` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]     - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let mut offsets = [0; 128];
    /// let offsets : &[usize] = compiler.get_trace_instruction_offsets_inplace(
    ///     shader.get_buffer(), GetInstOffsets::None, 0, &mut offsets
    /// ).unwrap();
    /// println!("{:?}", offsets);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [32, 52]
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_44.dll"))]
    pub fn get_trace_instruction_offsets_inplace<'o>(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        offsets:            &'o mut [usize],
    ) -> Result<&'o [usize], Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", ErrorKind::MISSING_DLL_EXPORT))?;
        let mut n = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags.into().into(), start_inst_index, offsets.len(), offsets.as_mut_ptr(), &mut n) };
        Error::check("D3DGetTraceInstructionOffsets", hr)?;
        Ok(&offsets[..n])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the byte offsets for instructions within a section of shader code.
    ///
    /// ### Returns
    /// *   Ok([Vec]&lt;[usize]&gt;)                    - trace instruction offsets
    /// *   Err([Error]) where `error.kind()` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]         - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// let offsets : Vec<usize> = compiler.get_trace_instruction_offsets(
    ///     shader.get_buffer(), GetInstOffsets::None, 0, std::usize::MAX
    /// ).unwrap();
    /// println!("{:?}", offsets);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [32, 52]
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_44.dll"))]
    pub fn get_trace_instruction_offsets(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<Vec<usize>, Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", ErrorKind::MISSING_DLL_EXPORT))?;
        let flags = flags.into().into();

        let mut n = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags, start_inst_index, num_insts, null_mut(), &mut n) };
        Error::check("D3DGetTraceInstructionOffsets", hr)?;

        let mut buffer = Vec::new();
        buffer.resize(n, 0usize);
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags, start_inst_index, buffer.len(), buffer.as_mut_ptr(), &mut n) };
        Error::check("D3DGetTraceInstructionOffsets", hr)?;
        debug_assert_eq!(n, buffer.len(), "number of instructions shouldn't have changed between calls");

        Ok(buffer)
    }
}
