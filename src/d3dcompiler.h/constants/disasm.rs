#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::*;
use winapi::um::d3dcompiler::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassembleregion#parameters)\]
/// UINT / D3D_DISASM_\*
///
/// Flags controlling how [D3DCompiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Disasm(UINT);

flags! {
    Disasm => UINT;
    None, EnableColorCode, EnableDefaultValuePrints, EnableInstructionNumbering, EnableInstructionCycle, DisableDebugInfo,
    EnableInstructionOffset, InstructionOnly,
}

#[allow(non_upper_case_globals)] impl Disasm { // These are enum-like
    pub const None                              : Disasm = Disasm(0);

    /// Enable the output of color codes.
    pub const EnableColorCode                   : Disasm = Disasm(D3D_DISASM_ENABLE_COLOR_CODE);

    /// Enable the output of default values.
    pub const EnableDefaultValuePrints          : Disasm = Disasm(D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS);

    /// Enable instruction numbering.
    pub const EnableInstructionNumbering        : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING);

    /// No effect.
    pub const EnableInstructionCycle            : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_CYCLE);

    /// Disable the output of debug information.
    pub const DisableDebugInfo                  : Disasm = Disasm(D3D_DISASM_DISABLE_DEBUG_INFO);

    /// Enable the output of instruction offsets.
    pub const EnableInstructionOffset           : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_OFFSET);

    /// This flag has no effect in [D3DCompiler::disassemble_region].
    /// Cycle information comes from the trace; therefore, cycle information is available only in [D3DDisassemble11Trace]'s trace disassembly.
    ///
    /// [D3DDisassemble11Trace]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d11shadertracing/nf-d3d11shadertracing-d3ddisassemble11trace
    pub const InstructionOnly                   : Disasm = Disasm(D3D_DISASM_INSTRUCTION_ONLY);
}

#[doc(hidden)] impl Disasm { // Ctrl+C Ctrl+V support
    pub const NONE                              : Disasm = Disasm(0);

    /// Enable the output of color codes.
    pub const ENABLE_COLOR_CODE                 : Disasm = Disasm(D3D_DISASM_ENABLE_COLOR_CODE);

    /// Enable the output of default values.
    pub const ENABLE_DEFAULT_VALUE_PRINTS       : Disasm = Disasm(D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS);

    /// Enable instruction numbering.
    pub const ENABLE_INSTRUCTION_NUMBERING      : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING);

    /// No effect.
    pub const ENABLE_INSTRUCTION_CYCLE          : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_CYCLE);

    /// Disable the output of debug information.
    pub const DISABLE_DEBUG_INFO                : Disasm = Disasm(D3D_DISASM_DISABLE_DEBUG_INFO);

    /// Enable the output of instruction offsets.
    pub const ENABLE_INSTRUCTION_OFFSET         : Disasm = Disasm(D3D_DISASM_ENABLE_INSTRUCTION_OFFSET);

    /// This flag has no effect in [D3DCompiler::disassemble_region].
    /// Cycle information comes from the trace; therefore, cycle information is available only in [D3DDisassemble11Trace]'s trace disassembly.
    ///
    /// [D3DDisassemble11Trace]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d11shadertracing/nf-d3d11shadertracing-d3ddisassemble11trace
    pub const INSTRUCTION_ONLY                  : Disasm = Disasm(D3D_DISASM_INSTRUCTION_ONLY);
}

impl Default for Disasm {
    fn default() -> Self { Disasm::None }
}
