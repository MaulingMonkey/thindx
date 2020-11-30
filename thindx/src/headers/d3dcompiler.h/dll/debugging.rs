use crate::*;

use std::ptr::*;



/// { disassembly: [ReadOnlyBlob], finish_byte_offset: [usize] }
pub struct DisassembledRegion {
    pub disassembly:        ReadOnlyBlob,
    pub finish_byte_offset: usize,
}



/// <h1 id="debugging" class="section-header"><a href="#debugging">Bytecode Debugging</a></h1>
impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassemble)\]
    /// D3DDisassemble
    ///
    /// Disassembles compiled HLSL code.
    ///
    /// ### Arguments
    /// *   `src_data`          - Compiled HLSL code.
    /// *   `flags`             - [Disasm] flags controlling how the compiled shader data is disassembled.
    /// *   `comments`          - Optional string at the top of the shader that identifies the shader constants and variables.
    ///
    /// ### Returns
    /// *   Ok([ReadOnlyBlob])      - the disassembly
    /// *   Err([Error]) with `error.kind()` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT] - on `d3dcompiler_39.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// # let shader = shader.get_buffer();
    /// let dis = compiler.disassemble(shader, Disasm::None, "// example comment\n").unwrap();
    /// println!("{}", String::from_utf8_lossy(dis.get_buffer()));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// //
    /// // Generated by Microsoft (R) HLSL Shader Compiler 10.1
    /// //
    /// // example comment
    /// //
    /// //
    /// // Input signature:
    /// //
    /// // Name                 Index   Mask Register SysValue  Format   Used
    /// // -------------------- ----- ------ -------- -------- ------- ------
    /// // COLOR                    0   xyzw        0     NONE   float   xyzw
    /// // SV_POSITION              0   xyzw        1      POS   float
    /// //
    /// //
    /// // Output signature:
    /// //
    /// // Name                 Index   Mask Register SysValue  Format   Used
    /// // -------------------- ----- ------ -------- -------- ------- ------
    /// // SV_TARGET                0   xyzw        0   TARGET   float   xyzw
    /// //
    /// ps_4_0
    /// dcl_input_ps linear v0.xyzw
    /// dcl_output o0.xyzw
    /// //
    /// // Initial variable locations:
    /// //   v0.x <- i.color.x; v0.y <- i.color.y; v0.z <- i.color.z; v0.w <- i.color.w;
    /// //   v1.x <- i.position.x; v1.y <- i.position.y; v1.z <- i.position.z; v1.w <- i.position.w;
    /// //   o0.x <- <ps_main return value>.color.x; o0.y <- <ps_main return value>.color.y; o0.z <- <ps_main return value>.color.z; o0.w <- <ps_main return value>.color.w    
    /// //
    /// #line 27 "C:\local\thindx\test\data\basic.hlsl"
    /// mov o0.xyzw, v0.xyzw
    /// ret
    /// // Approximately 2 instruction slots used
    /// ```
    #[requires(d3dcompiler=40)]
    pub fn disassemble<'s>(
        &self,
        src_data:           &[u8],
        flags:              impl Into<Disasm>,
        comments:           impl TryIntoAsOptCStr,
    ) -> Result<ReadOnlyBlob, Error> {
        let f = self.D3DDisassemble.ok_or(Error::new("D3DDisassemble", THINERR::MISSING_DLL_EXPORT))?;
        let flags = flags.into().into();
        let comments = comments.try_into().map_err(|e| Error::new("D3DDisassemble", e))?;
        let comments = comments.as_opt_cstr();
        let mut disassembly = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags, comments, &mut disassembly) };
        Error::check("D3DDisassemble", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(disassembly) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassembleregion)\]
    /// D3DDisassembleRegion
    ///
    /// Disassembles a specific region of compiled Microsoft High Level Shader Language (HLSL) code.
    ///
    /// ### Arguments
    /// *   `src_data`          - Compiled HLSL code.
    /// *   `flags`             - [Disasm] flags controlling how the compiled shader data is disassembled.
    /// *   `comments`          - Optional string at the top of the shader that identifies the shader constants and variables.
    /// *   `start_byte_offset` - The number of bytes offset into the compiled shader data where [disassemble_region](Self::disassemble_region) starts the disassembly.
    /// *   `num_insts`         - The number of instructions to disassemble.
    ///
    /// ### Returns
    /// *   Ok([DisassembledRegion] { disassembly: [ReadOnlyBlob], finish_byte_offset: [usize] })  - the disassembly
    /// *   Err([Error]) with `error.kind()` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT] - on `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let shader = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap().shader;
    /// # let shader = shader.get_buffer();
    /// let dr = compiler.disassemble_region(
    ///     shader, Disasm::None, "// example comment\n", 0, std::usize::MAX
    /// ).unwrap();
    /// println!("finish_byte_offset: {}", dr.finish_byte_offset);
    /// println!("{}", String::from_utf8_lossy(dr.disassembly.get_buffer()));
    /// ```
    ///
    /// ### Output
    /// ```text
    /// finish_byte_offset: 56
    /// //
    /// // Generated by Microsoft (R) HLSL Shader Compiler 10.1
    /// //
    /// // example comment
    /// //
    /// //
    /// // Input signature:
    /// //
    /// // Name                 Index   Mask Register SysValue  Format   Used
    /// // -------------------- ----- ------ -------- -------- ------- ------
    /// // COLOR                    0   xyzw        0     NONE   float   xyzw
    /// // SV_POSITION              0   xyzw        1      POS   float
    /// //
    /// //
    /// // Output signature:
    /// //
    /// // Name                 Index   Mask Register SysValue  Format   Used
    /// // -------------------- ----- ------ -------- -------- ------- ------
    /// // SV_TARGET                0   xyzw        0   TARGET   float   xyzw
    /// //
    /// ps_4_0
    /// dcl_input_ps linear v0.xyzw
    /// dcl_output o0.xyzw
    /// //
    /// // Initial variable locations:
    /// //   v0.x <- i.color.x; v0.y <- i.color.y; v0.z <- i.color.z; v0.w <- i.color.w;
    /// //   v1.x <- i.position.x; v1.y <- i.position.y; v1.z <- i.position.z; v1.w <- i.position.w;
    /// //   o0.x <- <ps_main return value>.color.x; o0.y <- <ps_main return value>.color.y; o0.z <- <ps_main return value>.color.z; o0.w <- <ps_main return value>.color.w    
    /// //
    /// #line 27 "C:\local\thindx\test\data\basic.hlsl"
    /// mov o0.xyzw, v0.xyzw
    /// ret
    /// // Approximately 2 instruction slots used
    /// ```
    #[requires(d3dcompiler=44)]
    pub fn disassemble_region<'s>(
        &self,
        src_data:           &[u8],
        flags:              impl Into<Disasm>,
        comments:           impl TryIntoAsOptCStr,
        start_byte_offset:  usize,
        num_insts:          usize,
    ) -> Result<DisassembledRegion, Error> {
        let f = self.D3DDisassembleRegion.ok_or(Error::new("D3DDisassembleRegion", THINERR::MISSING_DLL_EXPORT))?;
        let flags = flags.into().into();
        let comments = comments.try_into().map_err(|e| Error::new("D3DDisassembleRegion", e))?;
        let comments = comments.as_opt_cstr();
        let mut disassembly = null_mut();
        let mut finish_byte_offset = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), flags, comments, start_byte_offset, num_insts, &mut finish_byte_offset, &mut disassembly) };
        Error::check("D3DDisassembleRegion", hr)?;
        Ok(DisassembledRegion {
            disassembly: unsafe { ReadOnlyBlob::from_raw(disassembly) },
            finish_byte_offset,
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the number of byte offsets for instructions within a section of shader code.
    ///
    /// ### Returns
    /// *   Ok([usize])                             - the number of available trace instruction offsets in that range
    /// *   Err([Error]) where `error.kind()` ==
    ///     *   [THINERR::MISSING_DLL_EXPORT]    - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
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
    #[requires(d3dcompiler=44)]
    pub fn get_trace_instruction_offsets_count(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<usize, Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", THINERR::MISSING_DLL_EXPORT))?;
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
    ///     *   [THINERR::MISSING_DLL_EXPORT]     - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
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
    #[requires(d3dcompiler=44)]
    pub fn get_trace_instruction_offsets_inplace<'o>(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        offsets:            &'o mut [usize],
    ) -> Result<&'o [usize], Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", THINERR::MISSING_DLL_EXPORT))?;
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
    ///     *   [THINERR::MISSING_DLL_EXPORT]         - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
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
    #[requires(d3dcompiler=44)]
    pub fn get_trace_instruction_offsets(
        &self,
        src_data:           &[u8],
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<Vec<usize>, Error> {
        let f = self.D3DGetTraceInstructionOffsets.ok_or(Error::new("D3DGetTraceInstructionOffsets", THINERR::MISSING_DLL_EXPORT))?;
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
