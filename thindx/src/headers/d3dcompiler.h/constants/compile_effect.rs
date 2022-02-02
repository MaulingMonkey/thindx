#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcompiler::*;
#[allow(non_camel_case_types)] type D3DCOMPILE_EFFECT = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants)\]
/// UINT / D3DCOMPILE_EFFECT_*
///
/// Flags controlling how HLSL **effects** shaders are compiled.
///
/// ### See Also
/// *   [d3d::Compiler::compile_from_file]
/// *   [d3d::Compiler::compile]
/// *   [d3d::Compiler::compile2]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct CompileEffect(D3DCOMPILE_EFFECT);

flags! {
    CompileEffect => D3DCOMPILE_EFFECT;
    ChildEffect,
    AllowSlowOps,
}

#[allow(non_upper_case_globals)] impl CompileEffect { // These are enum-like
    #[doc=""]
    pub const None                                  : CompileEffect = CompileEffect(0);

    /// Compile the effects (.fx) file to a child effect. Child effects have no initializers for any shared values because these child effects are initialized in the master effect (the effect pool).
    ///
    /// > **Note:** Effect pools are supported by Effects 10 (FX10) but not by Effects 11 (FX11).
    /// > For more info about differences between effect pools in Direct3D 10 and effect groups in Direct3D 11, see [Effect Pools and Groups].
    ///
    /// [Effect Pools and Groups]:      https://docs.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-programming-guide-effects-differences
    pub const ChildEffect                           : CompileEffect = CompileEffect(D3DCOMPILE_EFFECT_CHILD_EFFECT);

    /// Disables performance mode and allows for mutable state objects.
    ///
    /// By default, performance mode is enabled. Performance mode disallows mutable state objects by preventing non-literal expressions from appearing in state object definitions.
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

//#cpp2rust D3DCOMPILE_EFFECT                   = d3d::CompileEffect
//#cpp2rust D3DCOMPILE_EFFECT_CHILD_EFFECT      = d3d::CompileEffect::ChildEffect
//#cpp2rust D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS    = d3d::CompileEffect::AllowSlowOps
