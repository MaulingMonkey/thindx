#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::minwindef::UINT;
use winapi::um::d3dcompiler::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets#parameters)\]
/// UINT / D3D_GET_INST_OFFSETS_*
///
/// [d3d::Compiler::get_trace_instruction_offsets] `flags`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct GetInstOffsets(UINT);

flags! { GetInstOffsets => UINT; None, IncludeNonExecutable }

#[allow(non_upper_case_globals)] impl GetInstOffsets { // These are enum-like
    #[doc=""]
    pub const None                      : GetInstOffsets = GetInstOffsets(0);

    /// Include non-executable code in the retrieved information.
    pub const IncludeNonExecutable      : GetInstOffsets = GetInstOffsets(D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE);
}



//#cpp2rust D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE     = d3d::GetInstOffsets::IncludeNonExecutable
