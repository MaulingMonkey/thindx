#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcompiler::*;
#[allow(non_camel_case_types)] type D3DCOMPILE_EFFECT = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants)\]
/// DWORD / D3DCOMPILE_EFFECT_*
///
/// These constants direct how the compiler compiles an effect file or how the runtime processes the effect file.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CompileEffect(D3DCOMPILE_EFFECT);

flags! {
    CompileEffect => D3DCOMPILE_EFFECT;
    ChildEffect,
    AllowSlowOps,
}

#[allow(non_upper_case_globals)] impl CompileEffect { // These are enum-like
    pub const None                                  : CompileEffect = CompileEffect(0);
    pub const ChildEffect                           : CompileEffect = CompileEffect(D3DCOMPILE_EFFECT_CHILD_EFFECT);
    pub const AllowSlowOps                          : CompileEffect = CompileEffect(D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS);
}

#[doc(hidden)] impl CompileEffect { // Ctrl+C Ctrl+V support
    pub const NONE                                  : CompileEffect = CompileEffect(0);
    pub const CHILD_EFFECT                          : CompileEffect = CompileEffect(D3DCOMPILE_EFFECT_CHILD_EFFECT);
    pub const ALLOW_SLOW_OPS                        : CompileEffect = CompileEffect(D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS);
}

impl Default for CompileEffect {
    fn default() -> Self { CompileEffect::None }
}
