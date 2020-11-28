#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;
use winapi::um::d3dcompiler::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets#parameters)\]
/// UINT / D3D_GET_INST_OFFSETS_*
///
/// [D3DCompiler::get_trace_instruction_offsets] `flags`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct GetInstOffsets(UINT);

flags! { GetInstOffsets => UINT; None, IncludeNonExecutable }

#[allow(non_upper_case_globals)] impl GetInstOffsets { // These are enum-like
    pub const None                      : GetInstOffsets = GetInstOffsets(0);

    /// Include non-executable code in the retrieved information.
    pub const IncludeNonExecutable      : GetInstOffsets = GetInstOffsets(D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE);
}

#[doc(hidden)] impl GetInstOffsets { // Ctrl+C Ctrl+V support
    pub const NONE                      : GetInstOffsets = GetInstOffsets(0);

    /// Include non-executable code in the retrieved information.
    pub const INCLUDE_NON_EXECUTABLE    : GetInstOffsets = GetInstOffsets(D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE);
}

impl Default for GetInstOffsets {
    fn default() -> Self { GetInstOffsets::None }
}
