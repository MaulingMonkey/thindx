use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use std::ptr::*;



/// { disassembly: [TextBlob], finish_byte_offset: [usize] }
///
/// ### See Also
/// *   [d3d::Compiler::disassemble_region]
#[derive(Clone, Debug)]
pub struct DisassembledRegion {
    /// Human-readable commented disassembly
    ///
    /// ### Example
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
    pub disassembly:        TextBlob,

    /// The number of bytes offset into the compiled shader data where [d3d::Compiler::disassemble_region] finishes the disassembly.
    pub finish_byte_offset: usize,
}



/// <h1 id="debugging" class="section-header"><a href="#debugging">Bytecode Debugging</a></h1>
impl Compiler {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassemble)\]
    /// D3DDisassemble
    ///
    /// Disassembles compiled HLSL code.
    ///
    /// ### Arguments
    /// *   `src_data`          - Compiled HLSL code.
    /// *   `flags`             - [Disasm] flags controlling how the compiled shader data is disassembled.
    /// *   `comments`          - Optional string at the top of the shader that identifies the shader constants and variables.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - on `d3dcompiler_39.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let dis = d3dc.disassemble(&shader, Disasm::None, "// example comment\n").unwrap();
    /// println!("{}", dis.to_utf8_lossy());
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
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.
    pub fn disassemble(
        &self,
        src_data:           &Bytecode,
        flags:              impl Into<Disasm>,
        comments:           impl TryIntoAsOptCStr,
    ) -> Result<TextBlob, Error> {
        fn_context_dll!(d3d::Compiler::disassemble => self.D3DDisassemble);
        let src_data = src_data.as_bytes();
        let flags = flags.into().into();
        let comments = fn_param_try_into!(comments)?;
        let comments = comments.as_opt_cstr();
        let mut disassembly = null_mut();

        // SAFETY: ❌ alloc not fuzzed for overflow, flags unfuzzed
        //  * `f`               ✔️ should be valid, like all of `self.*`
        //  * `src_data`        ❌ could cause alloc for `disassembly` to overflow
        //  * `src_data`        ✔️ should be valid bytecode, as implied by it's type, `Bytecode`
        //  * `flags`           ⚠️ could be invalid?
        //  * `comments`        ❌ could cause alloc for `disassembly` to overflow
        //  * `comments`        ✔️ should be a valid `\0` terminated C-string without interior `\0`s
        fn_check_hr!(unsafe { D3DDisassemble(src_data.as_ptr().cast(), src_data.len(), flags, comments, &mut disassembly) })?;

        // SAFETY: ✔️
        //  * `disassembly`     should be null (panics) or a non-dangling, valid ID3DBlob
        //  * `ReadOnlyBlob`    imposes no restrictions on the contents of `disassembly`
        Ok(TextBlob::new(unsafe { ReadOnlyBlob::from_raw(disassembly) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassembleregion)\]
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
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - on `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let dr = d3dc.disassemble_region(
    ///     &shader, Disasm::None, "// example comment\n", 0, std::usize::MAX
    /// ).unwrap();
    /// println!("finish_byte_offset: {}", dr.finish_byte_offset);
    /// println!("{}", dr.disassembly.to_utf8_lossy());
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
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub fn disassemble_region(
        &self,
        src_data:           &Bytecode,
        flags:              impl Into<Disasm>,
        comments:           impl TryIntoAsOptCStr,
        start_byte_offset:  usize,
        num_insts:          usize,
    ) -> Result<DisassembledRegion, Error> {
        fn_context_dll!(d3d::Compiler::disassemble_region => self.D3DDisassembleRegion);
        let src_data = src_data.as_bytes();
        let flags = flags.into().into();
        let comments = fn_param_try_into!(comments)?;
        let comments = comments.as_opt_cstr();
        let mut disassembly = null_mut();
        let mut finish_byte_offset = 0;

        // SAFETY: ❌ alloc not fuzzed for overflow, flags unfuzzed
        //  * `f`                   ✔️ should be valid, like all of `self.*`
        //  * `src_data`            ❌ could cause alloc for `disassembly` to overflow
        //  * `src_data`            ✔️ should be valid bytecode, as implied by it's type, `Bytecode`
        //  * `flags`               ⚠️ could be invalid?
        //  * `comments`            ❌ could cause alloc for `disassembly` to overflow
        //  * `comments`            ✔️ should be a valid `\0` terminated C-string without interior `\0`s
        //  * `start_byte_offset`   ❌ could be out-of-bounds
        //  * `num_insts`           ❌ could be out-of-bounds
        //  * `finish_byte_offset`  ✔️ is a trivial out param
        //  * `disassembly`         ✔️ is a trivial out param
        fn_check_hr!(unsafe { D3DDisassembleRegion(src_data.as_ptr().cast(), src_data.len(), flags, comments, start_byte_offset, num_insts, &mut finish_byte_offset, &mut disassembly) })?;
        Ok(DisassembledRegion {
            // SAFETY: ✔️
            //  * `disassembly`     should be null (panics) or a non-dangling, valid ID3DBlob
            //  * `ReadOnlyBlob`    imposes no restrictions on the contents of `disassembly`
            disassembly: TextBlob::new(unsafe { ReadOnlyBlob::from_raw(disassembly) }),
            finish_byte_offset,
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the number of byte offsets for instructions within a section of shader code.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// println!("{}", d3dc.get_trace_instruction_offsets_count(
    ///     &shader, GetInstOffsets::None, 0, std::usize::MAX
    /// ).unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// 2
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    //#allow_missing_argument_docs
    pub fn get_trace_instruction_offsets_count(
        &self,
        src_data:           &Bytecode,
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<usize, Error> {
        fn_context_dll!(d3d::Compiler::get_trace_instruction_offsets_count => self.D3DGetTraceInstructionOffsets);
        let src_data = src_data.as_bytes();
        let mut n = 0;

        // SAFETY: ❌ indicies could be out of bounds, flags unfuzzed
        //  * `f`                   ✔️ should be valid, like all of `self.*`
        //  * `src_data`            ✔️ should be valid bytecode, as implied by it's type, `Bytecode`
        //  * `flags`               ⚠️ could be invalid?
        //  * `start_inst_index`    ❌ could be out-of-bounds
        //  * `num_insts`           ❌ could be out-of-bounds
        //  * `pOffsets`            ✔️ null is valid (we're just getting the count)
        //  * `pTotalInsts`         ✔️ is a trivial out param
        fn_check_hr!(unsafe { D3DGetTraceInstructionOffsets(src_data.as_ptr().cast(), src_data.len(), flags.into().into(), start_inst_index, num_insts, null_mut(), &mut n) })?;
        Ok(n)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the byte offsets for instructions within a section of shader code.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let mut offsets = [0; 128];
    /// let offsets : &[usize] = d3dc.get_trace_instruction_offsets_inplace(
    ///     &shader, GetInstOffsets::None, 0, &mut offsets
    /// ).unwrap();
    /// println!("{:?}", offsets);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [32, 52]
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    //#allow_missing_argument_docs
    pub fn get_trace_instruction_offsets_inplace<'o>(
        &self,
        src_data:           &Bytecode,
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        offsets:            &'o mut [usize],
    ) -> Result<&'o [usize], Error> {
        fn_context_dll!(d3d::Compiler::get_trace_instruction_offsets_inplace => self.D3DGetTraceInstructionOffsets);
        let src_data = src_data.as_bytes();
        let mut n = 0;

        // SAFETY: ❌ indicies could be out of bounds, flags unfuzzed
        //  * `f`                   ✔️ should be valid, like all of `self.*`
        //  * `src_data`            ✔️ should be valid bytecode, as implied by it's type, `Bytecode`
        //  * `flags`               ⚠️ could be invalid?
        //  * `start_inst_index`    ❌ could be out-of-bounds
        //  * `NumInsts`            ✔️ should be valid.  Even if `D3DGetTraceInstructionOffsets` truncates, offsets is initialized, so a short read should be harmless.
        //  * `pOffsets`            ✔️ is valid for offsets.len() elements.
        //  * `pTotalInsts`         ✔️ is a trivial out param
        fn_check_hr!(unsafe { D3DGetTraceInstructionOffsets(src_data.as_ptr().cast(), src_data.len(), flags.into().into(), start_inst_index, offsets.len(), offsets.as_mut_ptr(), &mut n) })?;
        Ok(&offsets[..n])
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets)\]
    /// D3DGetTraceInstructionOffsets
    ///
    /// Retrieves the byte offsets for instructions within a section of shader code.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_43.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let offsets : Vec<usize> = d3dc.get_trace_instruction_offsets(
    ///     &shader, GetInstOffsets::None, 0, std::usize::MAX
    /// ).unwrap();
    /// println!("{:?}", offsets);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [32, 52]
    /// ```
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    //#allow_missing_argument_docs
    pub fn get_trace_instruction_offsets(
        &self,
        src_data:           &Bytecode,
        flags:              impl Into<GetInstOffsets>,
        start_inst_index:   usize,
        num_insts:          usize,
    ) -> Result<Vec<usize>, Error> {
        fn_context!(d3d::Compiler::get_trace_instruction_offsets => D3DGetTraceInstructionOffsets);
        let flags = flags.into();
        let n = self.get_trace_instruction_offsets_count(src_data, flags, start_inst_index, num_insts)?;
        let mut buffer = vec![0usize; n];
        let n2 = self.get_trace_instruction_offsets_inplace(src_data, flags, start_inst_index, &mut buffer)?.len();
        debug_assert_eq!(n, n2, "number of instructions shouldn't have changed between calls");
        buffer.shrink_to(n2);
        Ok(buffer)
    }
}

//TODO:     D3DDisassemble10Effect                  = d3d::Compiler::disassemble_10_effect
