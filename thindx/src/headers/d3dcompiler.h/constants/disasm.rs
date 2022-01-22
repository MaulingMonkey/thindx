#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::*;
use winapi::um::d3dcompiler::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassemble#parameters)\]
/// UINT / D3D_DISASM_\*
///
/// Flags controlling how [d3d::Compiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Disasm(UINT);

flags! {
    Disasm => UINT;
    None, EnableColorCode, EnableDefaultValuePrints, EnableInstructionNumbering, EnableInstructionCycle, DisableDebugInfo,
    EnableInstructionOffset, InstructionOnly,
}

#[allow(non_upper_case_globals)] impl Disasm { // These are enum-like
    #[doc=""]
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

    /// This flag has no effect in [d3d::Compiler::disassemble_region].
    /// Cycle information comes from the trace; therefore, cycle information is available only in [D3DDisassemble11Trace]'s trace disassembly.
    ///
    /// [D3DDisassemble11Trace]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d11shadertracing/nf-d3d11shadertracing-d3ddisassemble11trace
    pub const InstructionOnly                   : Disasm = Disasm(D3D_DISASM_INSTRUCTION_ONLY);

    /// Use hex symbols in disassemblies.
    pub const PrintHexLiterals                  : Disasm = Disasm(D3D_DISASM_PRINT_HEX_LITERALS);
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

    /// This flag has no effect in [Compiler::disassemble_region].
    /// Cycle information comes from the trace; therefore, cycle information is available only in [D3DDisassemble11Trace]'s trace disassembly.
    ///
    /// [D3DDisassemble11Trace]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d11shadertracing/nf-d3d11shadertracing-d3ddisassemble11trace
    pub const INSTRUCTION_ONLY                  : Disasm = Disasm(D3D_DISASM_INSTRUCTION_ONLY);

    /// Use hex symbols in disassemblies.
    pub const PRINT_HEX_LITERALS                : Disasm = Disasm(D3D_DISASM_PRINT_HEX_LITERALS);
}

impl Default for Disasm {
    fn default() -> Self { Disasm::None }
}

//#cpp2rust D3D_DISASM_ENABLE_COLOR_CODE            = d3d::Disasm::EnableColorCode
//#cpp2rust D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS  = d3d::Disasm::EnableDefaultValuePrints
//#cpp2rust D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING = d3d::Disasm::EnableInstructionNumbering
//#cpp2rust D3D_DISASM_ENABLE_INSTRUCTION_CYCLE     = d3d::Disasm::EnableInstructionCycle
//#cpp2rust D3D_DISASM_DISABLE_DEBUG_INFO           = d3d::Disasm::DisableDebugInfo
//#cpp2rust D3D_DISASM_ENABLE_INSTRUCTION_OFFSET    = d3d::Disasm::EnableInstructionOffset
//#cpp2rust D3D_DISASM_INSTRUCTION_ONLY             = d3d::Disasm::InstructionOnly
//#cpp2rust D3D_DISASM_PRINT_HEX_LITERALS           = d3d::Disasm::PrintHexLiterals
