#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype)\]
/// D3DRENDERSTATETYPE
///
/// Render states define set-up states for all kinds of vertex and pixel processing.
/// Some render states set up vertex processing, and some set up pixel processing (see [Render States (Direct3D 9)]).
/// Render states can be saved and restored using stateblocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [Render States (Direct3D 9)]:                       https://docs.microsoft.com/en-us/windows/win32/direct3d9/render-states
/// [State Blocks Save and Restore State (Direct3D 9)]: https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RenderStateType(D3DRENDERSTATETYPE);
pub type RS = RenderStateType;

enumish! {
    RS => D3DRENDERSTATETYPE;
    ZEnable, FillMode, ShadeMode, ZWriteEnable, AlphaTestEnable, LastPixel, SrcBlend, DestBlend,
    CullMode, ZFunc, AlphaRef, AlphaFunc, DitherEnable, AlphaBlendEnable, FogEnable, SpecularEnable,
    FogColor, FogTableMode, FogStart, FogEnd, FogDensity, RangeFogEnable,
    StencilEnable, StencilFail, StencilZFail, StencilPass, StencilFunc, StencilRef, StencilMask, StencilWriteMask, TextureFactor,
    Wrap0, Wrap1, Wrap2, Wrap3, Wrap4, Wrap5, Wrap6, Wrap7,
    Clipping, Lighting, Ambient, FogVertexMode, ColorVertex, LocalViewer, NormalizeNormals,
    DiffuseMaterialSource, SpecularMaterialSource, AmbientMaterialSource, EmissiveMaterialSource, VertexBlend, ClipPlaneEnable,
    PointSize, PointSizeMin, PointSpriteEnable, PointScaleEnable, PointScaleA, PointScaleB, PointScaleC,
    MultiSampleAntiAlias, MultiSampleMask, PatchEdgeStyle, DebugMonitorToken, PointSizeMax, IndexedVertexBlendEnable,
    ColorWriteEnable, TweenFactor, BlendOp, PositionDegree, NormalDegree, ScissorTestEnable, SlopeScaleDepthBias,
    AntiAliasedLineEnable, MinTessellationLevel, MaxTessellationLevel, AdaptiveTessX, AdaptiveTessY, AdaptiveTessZ, AdaptiveTessW,
    EnableAdaptiveTessellation, TwoSidedStencilMode, CcwStencilFail, CcwStencilZFail, CcwStencilPass, CcwStencilFunc,
    ColorWriteEnable1, ColorWriteEnable2, ColorWriteEnable3, BlendFactor, SRGBWriteEnable, DepthBias,
    Wrap8, Wrap9, Wrap10, Wrap11, Wrap12, Wrap13, Wrap14, Wrap15,
    SeparateAlphaBlendEnable, SrcBlendAlpha, DestBlendAlpha, BlendOpAlpha,
}

#[allow(non_upper_case_globals)] // These are enum-like
impl RenderStateType {
    pub const ZEnable                       : RenderStateType = RenderStateType(D3DRS_ZENABLE); // 7
    pub const FillMode                      : RenderStateType = RenderStateType(D3DRS_FILLMODE);
    pub const ShadeMode                     : RenderStateType = RenderStateType(D3DRS_SHADEMODE);
    pub const ZWriteEnable                  : RenderStateType = RenderStateType(D3DRS_ZWRITEENABLE);
    pub const AlphaTestEnable               : RenderStateType = RenderStateType(D3DRS_ALPHATESTENABLE);
    pub const LastPixel                     : RenderStateType = RenderStateType(D3DRS_LASTPIXEL);
    pub const SrcBlend                      : RenderStateType = RenderStateType(D3DRS_SRCBLEND);
    pub const DestBlend                     : RenderStateType = RenderStateType(D3DRS_DESTBLEND);
    pub const CullMode                      : RenderStateType = RenderStateType(D3DRS_CULLMODE);
    pub const ZFunc                         : RenderStateType = RenderStateType(D3DRS_ZFUNC);
    pub const AlphaRef                      : RenderStateType = RenderStateType(D3DRS_ALPHAREF);
    pub const AlphaFunc                     : RenderStateType = RenderStateType(D3DRS_ALPHAFUNC);
    pub const DitherEnable                  : RenderStateType = RenderStateType(D3DRS_DITHERENABLE);
    pub const AlphaBlendEnable              : RenderStateType = RenderStateType(D3DRS_ALPHABLENDENABLE);
    pub const FogEnable                     : RenderStateType = RenderStateType(D3DRS_FOGENABLE);
    pub const SpecularEnable                : RenderStateType = RenderStateType(D3DRS_SPECULARENABLE);
    pub const FogColor                      : RenderStateType = RenderStateType(D3DRS_FOGCOLOR);
    pub const FogTableMode                  : RenderStateType = RenderStateType(D3DRS_FOGTABLEMODE);
    pub const FogStart                      : RenderStateType = RenderStateType(D3DRS_FOGSTART);
    pub const FogEnd                        : RenderStateType = RenderStateType(D3DRS_FOGEND);
    pub const FogDensity                    : RenderStateType = RenderStateType(D3DRS_FOGDENSITY);
    pub const RangeFogEnable                : RenderStateType = RenderStateType(D3DRS_RANGEFOGENABLE);
    pub const StencilEnable                 : RenderStateType = RenderStateType(D3DRS_STENCILENABLE);
    pub const StencilFail                   : RenderStateType = RenderStateType(D3DRS_STENCILFAIL);
    pub const StencilZFail                  : RenderStateType = RenderStateType(D3DRS_STENCILZFAIL);
    pub const StencilPass                   : RenderStateType = RenderStateType(D3DRS_STENCILPASS);
    pub const StencilFunc                   : RenderStateType = RenderStateType(D3DRS_STENCILFUNC);
    pub const StencilRef                    : RenderStateType = RenderStateType(D3DRS_STENCILREF);
    pub const StencilMask                   : RenderStateType = RenderStateType(D3DRS_STENCILMASK);
    pub const StencilWriteMask              : RenderStateType = RenderStateType(D3DRS_STENCILWRITEMASK);
    pub const TextureFactor                 : RenderStateType = RenderStateType(D3DRS_TEXTUREFACTOR);
    pub const Wrap0                         : RenderStateType = RenderStateType(D3DRS_WRAP0);
    pub const Wrap1                         : RenderStateType = RenderStateType(D3DRS_WRAP1);
    pub const Wrap2                         : RenderStateType = RenderStateType(D3DRS_WRAP2);
    pub const Wrap3                         : RenderStateType = RenderStateType(D3DRS_WRAP3);
    pub const Wrap4                         : RenderStateType = RenderStateType(D3DRS_WRAP4);
    pub const Wrap5                         : RenderStateType = RenderStateType(D3DRS_WRAP5);
    pub const Wrap6                         : RenderStateType = RenderStateType(D3DRS_WRAP6);
    pub const Wrap7                         : RenderStateType = RenderStateType(D3DRS_WRAP7);
    pub const Clipping                      : RenderStateType = RenderStateType(D3DRS_CLIPPING);
    pub const Lighting                      : RenderStateType = RenderStateType(D3DRS_LIGHTING);
    pub const Ambient                       : RenderStateType = RenderStateType(D3DRS_AMBIENT);
    pub const FogVertexMode                 : RenderStateType = RenderStateType(D3DRS_FOGVERTEXMODE);
    pub const ColorVertex                   : RenderStateType = RenderStateType(D3DRS_COLORVERTEX);
    pub const LocalViewer                   : RenderStateType = RenderStateType(D3DRS_LOCALVIEWER);
    pub const NormalizeNormals              : RenderStateType = RenderStateType(D3DRS_NORMALIZENORMALS);
    pub const DiffuseMaterialSource         : RenderStateType = RenderStateType(D3DRS_DIFFUSEMATERIALSOURCE);
    pub const SpecularMaterialSource        : RenderStateType = RenderStateType(D3DRS_SPECULARMATERIALSOURCE);
    pub const AmbientMaterialSource         : RenderStateType = RenderStateType(D3DRS_AMBIENTMATERIALSOURCE);
    pub const EmissiveMaterialSource        : RenderStateType = RenderStateType(D3DRS_EMISSIVEMATERIALSOURCE);
    pub const VertexBlend                   : RenderStateType = RenderStateType(D3DRS_VERTEXBLEND);
    pub const ClipPlaneEnable               : RenderStateType = RenderStateType(D3DRS_CLIPPLANEENABLE);
    pub const PointSize                     : RenderStateType = RenderStateType(D3DRS_POINTSIZE);
    pub const PointSizeMin                  : RenderStateType = RenderStateType(D3DRS_POINTSIZE_MIN);
    pub const PointSpriteEnable             : RenderStateType = RenderStateType(D3DRS_POINTSPRITEENABLE);
    pub const PointScaleEnable              : RenderStateType = RenderStateType(D3DRS_POINTSCALEENABLE);
    pub const PointScaleA                   : RenderStateType = RenderStateType(D3DRS_POINTSCALE_A);
    pub const PointScaleB                   : RenderStateType = RenderStateType(D3DRS_POINTSCALE_B);
    pub const PointScaleC                   : RenderStateType = RenderStateType(D3DRS_POINTSCALE_C);
    pub const MultiSampleAntiAlias          : RenderStateType = RenderStateType(D3DRS_MULTISAMPLEANTIALIAS);
    pub const MultiSampleMask               : RenderStateType = RenderStateType(D3DRS_MULTISAMPLEMASK);
    pub const PatchEdgeStyle                : RenderStateType = RenderStateType(D3DRS_PATCHEDGESTYLE);
    pub const DebugMonitorToken             : RenderStateType = RenderStateType(D3DRS_DEBUGMONITORTOKEN);
    pub const PointSizeMax                  : RenderStateType = RenderStateType(D3DRS_POINTSIZE_MAX);
    pub const IndexedVertexBlendEnable      : RenderStateType = RenderStateType(D3DRS_INDEXEDVERTEXBLENDENABLE);
    pub const ColorWriteEnable              : RenderStateType = RenderStateType(D3DRS_COLORWRITEENABLE);
    pub const TweenFactor                   : RenderStateType = RenderStateType(D3DRS_TWEENFACTOR);
    pub const BlendOp                       : RenderStateType = RenderStateType(D3DRS_BLENDOP);
    pub const PositionDegree                : RenderStateType = RenderStateType(D3DRS_POSITIONDEGREE);
    pub const NormalDegree                  : RenderStateType = RenderStateType(D3DRS_NORMALDEGREE);
    pub const ScissorTestEnable             : RenderStateType = RenderStateType(D3DRS_SCISSORTESTENABLE);
    pub const SlopeScaleDepthBias           : RenderStateType = RenderStateType(D3DRS_SLOPESCALEDEPTHBIAS);
    pub const AntiAliasedLineEnable         : RenderStateType = RenderStateType(D3DRS_ANTIALIASEDLINEENABLE);
    pub const MinTessellationLevel          : RenderStateType = RenderStateType(D3DRS_MINTESSELLATIONLEVEL);
    pub const MaxTessellationLevel          : RenderStateType = RenderStateType(D3DRS_MAXTESSELLATIONLEVEL);
    pub const AdaptiveTessX                 : RenderStateType = RenderStateType(D3DRS_ADAPTIVETESS_X);
    pub const AdaptiveTessY                 : RenderStateType = RenderStateType(D3DRS_ADAPTIVETESS_Y);
    pub const AdaptiveTessZ                 : RenderStateType = RenderStateType(D3DRS_ADAPTIVETESS_Z);
    pub const AdaptiveTessW                 : RenderStateType = RenderStateType(D3DRS_ADAPTIVETESS_W);
    pub const EnableAdaptiveTessellation    : RenderStateType = RenderStateType(D3DRS_ENABLEADAPTIVETESSELLATION);
    pub const TwoSidedStencilMode           : RenderStateType = RenderStateType(D3DRS_TWOSIDEDSTENCILMODE);
    pub const CcwStencilFail                : RenderStateType = RenderStateType(D3DRS_CCW_STENCILFAIL);
    pub const CcwStencilZFail               : RenderStateType = RenderStateType(D3DRS_CCW_STENCILZFAIL);
    pub const CcwStencilPass                : RenderStateType = RenderStateType(D3DRS_CCW_STENCILPASS);
    pub const CcwStencilFunc                : RenderStateType = RenderStateType(D3DRS_CCW_STENCILFUNC);
    pub const ColorWriteEnable1             : RenderStateType = RenderStateType(D3DRS_COLORWRITEENABLE1);
    pub const ColorWriteEnable2             : RenderStateType = RenderStateType(D3DRS_COLORWRITEENABLE2);
    pub const ColorWriteEnable3             : RenderStateType = RenderStateType(D3DRS_COLORWRITEENABLE3);
    pub const BlendFactor                   : RenderStateType = RenderStateType(D3DRS_BLENDFACTOR);
    pub const SRGBWriteEnable               : RenderStateType = RenderStateType(D3DRS_SRGBWRITEENABLE);
    pub const DepthBias                     : RenderStateType = RenderStateType(D3DRS_DEPTHBIAS);
    pub const Wrap8                         : RenderStateType = RenderStateType(D3DRS_WRAP8);
    pub const Wrap9                         : RenderStateType = RenderStateType(D3DRS_WRAP9);
    pub const Wrap10                        : RenderStateType = RenderStateType(D3DRS_WRAP10);
    pub const Wrap11                        : RenderStateType = RenderStateType(D3DRS_WRAP11);
    pub const Wrap12                        : RenderStateType = RenderStateType(D3DRS_WRAP12);
    pub const Wrap13                        : RenderStateType = RenderStateType(D3DRS_WRAP13);
    pub const Wrap14                        : RenderStateType = RenderStateType(D3DRS_WRAP14);
    pub const Wrap15                        : RenderStateType = RenderStateType(D3DRS_WRAP15);
    pub const SeparateAlphaBlendEnable      : RenderStateType = RenderStateType(D3DRS_SEPARATEALPHABLENDENABLE);
    pub const SrcBlendAlpha                 : RenderStateType = RenderStateType(D3DRS_SRCBLENDALPHA);
    pub const DestBlendAlpha                : RenderStateType = RenderStateType(D3DRS_DESTBLENDALPHA);
    pub const BlendOpAlpha                  : RenderStateType = RenderStateType(D3DRS_BLENDOPALPHA);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for RenderStateType {
    fn default() -> Self { RenderStateType(0) }
}
