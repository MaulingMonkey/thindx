#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcompiler::*;
type D3DCOMPILE = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants)\]
/// DWORD / D3DCOMPILE_*
///
/// Flags controlling how HLSL shaders are compiled to bytecode.
///
/// ### See Also
/// *   [D3DCompiler::compile_from_file]
/// *   [D3DCompiler::compile]
/// *   [D3DCompiler::compile2]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Compile(D3DCOMPILE);

flags! {
    Compile => D3DCOMPILE;
    None, Debug, SkipValidation, SkipOptimization, PackMatrixRowMajor, PackMatrixColumnMajor, PartialPrecision,
    ForceVsSoftwareNoOpt, ForcePsSoftwareNoOpt, NoPreshader, AvoidFlowControl, PreferFlowControl, EnableStrictness,
    EnableBackwardsCompatibility, IeeeStrictness,
    OptimizationLevel2, // Level0 | Level3
    OptimizationLevel0,
    //OptimizationLevel1, // 0
    OptimizationLevel3,
    WarningsAreErrors, ResourcesMayAlias, EnableUnboundedDescriptorTables, AllResourcesBound,
}

#[allow(non_upper_case_globals)] impl Compile { // These are enum-like
    pub const None                                  : Compile = Compile(0);
    pub const Debug                                 : Compile = Compile(D3DCOMPILE_DEBUG);
    pub const SkipValidation                        : Compile = Compile(D3DCOMPILE_SKIP_VALIDATION);
    pub const SkipOptimization                      : Compile = Compile(D3DCOMPILE_SKIP_OPTIMIZATION);
    pub const PackMatrixRowMajor                    : Compile = Compile(D3DCOMPILE_PACK_MATRIX_ROW_MAJOR);
    pub const PackMatrixColumnMajor                 : Compile = Compile(D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR);
    pub const PartialPrecision                      : Compile = Compile(D3DCOMPILE_PARTIAL_PRECISION);
    pub const ForceVsSoftwareNoOpt                  : Compile = Compile(D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT);
    pub const ForcePsSoftwareNoOpt                  : Compile = Compile(D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT);
    pub const NoPreshader                           : Compile = Compile(D3DCOMPILE_NO_PRESHADER);
    pub const AvoidFlowControl                      : Compile = Compile(D3DCOMPILE_AVOID_FLOW_CONTROL);
    pub const PreferFlowControl                     : Compile = Compile(D3DCOMPILE_PREFER_FLOW_CONTROL);
    pub const EnableStrictness                      : Compile = Compile(D3DCOMPILE_ENABLE_STRICTNESS);
    pub const EnableBackwardsCompatibility          : Compile = Compile(D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY);
    pub const IeeeStrictness                        : Compile = Compile(D3DCOMPILE_IEEE_STRICTNESS);
    pub const OptimizationLevel0                    : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL0);
    pub const OptimizationLevel1                    : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL1);
    pub const OptimizationLevel2                    : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL2);
    pub const OptimizationLevel3                    : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL3);
    pub const WarningsAreErrors                     : Compile = Compile(D3DCOMPILE_WARNINGS_ARE_ERRORS);
    pub const ResourcesMayAlias                     : Compile = Compile(D3DCOMPILE_RESOURCES_MAY_ALIAS);
    pub const EnableUnboundedDescriptorTables       : Compile = Compile(D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES);
    pub const AllResourcesBound                     : Compile = Compile(D3DCOMPILE_ALL_RESOURCES_BOUND);
}

#[doc(hidden)] impl Compile { // Ctrl+C Ctrl+V support
    pub const NONE                                  : Compile = Compile(0);
    pub const DEBUG                                 : Compile = Compile(D3DCOMPILE_DEBUG);
    pub const SKIP_VALIDATION                       : Compile = Compile(D3DCOMPILE_SKIP_VALIDATION);
    pub const SKIP_OPTIMIZATION                     : Compile = Compile(D3DCOMPILE_SKIP_OPTIMIZATION);
    pub const PACK_MATRIX_ROW_MAJOR                 : Compile = Compile(D3DCOMPILE_PACK_MATRIX_ROW_MAJOR);
    pub const PACK_MATRIX_COLUMN_MAJOR              : Compile = Compile(D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR);
    pub const PARTIAL_PRECISION                     : Compile = Compile(D3DCOMPILE_PARTIAL_PRECISION);
    pub const FORCE_VS_SOFTWARE_NO_OPT              : Compile = Compile(D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT);
    pub const FORCE_PS_SOFTWARE_NO_OPT              : Compile = Compile(D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT);
    pub const NO_PRESHADER                          : Compile = Compile(D3DCOMPILE_NO_PRESHADER);
    pub const AVOID_FLOW_CONTROL                    : Compile = Compile(D3DCOMPILE_AVOID_FLOW_CONTROL);
    pub const PREFER_FLOW_CONTROL                   : Compile = Compile(D3DCOMPILE_PREFER_FLOW_CONTROL);
    pub const ENABLE_STRICTNESS                     : Compile = Compile(D3DCOMPILE_ENABLE_STRICTNESS);
    pub const ENABLE_BACKWARDS_COMPATIBILITY        : Compile = Compile(D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY);
    pub const IEEE_STRICTNESS                       : Compile = Compile(D3DCOMPILE_IEEE_STRICTNESS);
    pub const OPTIMIZATION_LEVEL0                   : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL0);
    pub const OPTIMIZATION_LEVEL1                   : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL1);
    pub const OPTIMIZATION_LEVEL2                   : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL2);
    pub const OPTIMIZATION_LEVEL3                   : Compile = Compile(D3DCOMPILE_OPTIMIZATION_LEVEL3);
    pub const WARNINGS_ARE_ERRORS                   : Compile = Compile(D3DCOMPILE_WARNINGS_ARE_ERRORS);
    pub const RESOURCES_MAY_ALIAS                   : Compile = Compile(D3DCOMPILE_RESOURCES_MAY_ALIAS);
    pub const ENABLE_UNBOUNDED_DESCRIPTOR_TABLES    : Compile = Compile(D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES);
    pub const ALL_RESOURCES_BOUND                   : Compile = Compile(D3DCOMPILE_ALL_RESOURCES_BOUND);
}

impl Default for Compile {
    fn default() -> Self { Compile::None }
}
