#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;
use winapi::um::d3dcompiler::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2#parameters)\]
/// UINT / D3DCOMPILE_SECDATA_*
///
/// [D3DCompiler::compile2] `secondary_data_flags`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CompileSecData(UINT);
#[doc(hidden)] pub use CompileSecData as CompileSecdata;

flags! { CompileSecData => UINT; None, MergeUavSlots, PreserveTemplateSlots, RequireTemplateMatch }

#[allow(non_upper_case_globals)] impl CompileSecData { // These are enum-like
    pub const None                      : CompileSecData = CompileSecData(0);

    /// Merge unordered access view (UAV) slots in the secondary data that the pSecondaryData parameter points to.
    pub const MergeUavSlots             : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS);

    /// Preserve template slots in the secondary data that the pSecondaryData parameter points to.
    pub const PreserveTemplateSlots     : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS);

    /// Require that templates in the secondary data that the pSecondaryData parameter points to match when the compiler compiles the HLSL code.
    pub const RequireTemplateMatch      : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH);
}

#[doc(hidden)] impl CompileSecData { // Ctrl+C Ctrl+V support
    pub const NONE                      : CompileSecData = CompileSecData(0);

    /// Merge unordered access view (UAV) slots in the secondary data that the pSecondaryData parameter points to.
    pub const MERGE_UAV_SLOTS           : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS);

    /// Preserve template slots in the secondary data that the pSecondaryData parameter points to.
    pub const PRESERVE_TEMPLATE_SLOTS   : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS);

    /// Require that templates in the secondary data that the pSecondaryData parameter points to match when the compiler compiles the HLSL code.
    pub const REQUIRE_TEMPLATE_MATCH    : CompileSecData = CompileSecData(D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH);
}

impl Default for CompileSecData {
    fn default() -> Self { CompileSecData::None }
}
