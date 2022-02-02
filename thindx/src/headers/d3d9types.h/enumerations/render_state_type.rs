#[allow(unused_imports)] use crate::*;
use bytemuck::*;
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
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct RenderStateType(D3DRENDERSTATETYPE);
pub use RenderStateType as RS;

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

impl RenderStateType {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DRENDERSTATETYPE                  = d3d::RenderStateType
//#cpp2rust D3DRS_ZENABLE                       = d3d::RS::ZEnable
//#cpp2rust D3DRS_FILLMODE                      = d3d::RS::FillMode
//#cpp2rust D3DRS_SHADEMODE                     = d3d::RS::ShadeMode
//#cpp2rust D3DRS_ZWRITEENABLE                  = d3d::RS::ZWriteEnable
//#cpp2rust D3DRS_ALPHATESTENABLE               = d3d::RS::AlphaTestEnable
//#cpp2rust D3DRS_LASTPIXEL                     = d3d::RS::LastPixel
//#cpp2rust D3DRS_SRCBLEND                      = d3d::RS::SrcBlend
//#cpp2rust D3DRS_DESTBLEND                     = d3d::RS::DestBlend
//#cpp2rust D3DRS_CULLMODE                      = d3d::RS::CullMode
//#cpp2rust D3DRS_ZFUNC                         = d3d::RS::ZFunc
//#cpp2rust D3DRS_ALPHAREF                      = d3d::RS::AlphaRef
//#cpp2rust D3DRS_ALPHAFUNC                     = d3d::RS::AlphaFunc
//#cpp2rust D3DRS_DITHERENABLE                  = d3d::RS::DitherEnable
//#cpp2rust D3DRS_ALPHABLENDENABLE              = d3d::RS::AlphaBlendEnable
//#cpp2rust D3DRS_FOGENABLE                     = d3d::RS::FogEnable
//#cpp2rust D3DRS_SPECULARENABLE                = d3d::RS::SpecularEnable
//#cpp2rust D3DRS_FOGCOLOR                      = d3d::RS::FogColor
//#cpp2rust D3DRS_FOGTABLEMODE                  = d3d::RS::FogTableMode
//#cpp2rust D3DRS_FOGSTART                      = d3d::RS::FogStart
//#cpp2rust D3DRS_FOGEND                        = d3d::RS::FogEnd
//#cpp2rust D3DRS_FOGDENSITY                    = d3d::RS::FogDensity
//#cpp2rust D3DRS_RANGEFOGENABLE                = d3d::RS::RangeFogEnable
//#cpp2rust D3DRS_STENCILENABLE                 = d3d::RS::StencilEnable
//#cpp2rust D3DRS_STENCILFAIL                   = d3d::RS::StencilFail
//#cpp2rust D3DRS_STENCILZFAIL                  = d3d::RS::StencilZFail
//#cpp2rust D3DRS_STENCILPASS                   = d3d::RS::StencilPass
//#cpp2rust D3DRS_STENCILFUNC                   = d3d::RS::StencilFunc
//#cpp2rust D3DRS_STENCILREF                    = d3d::RS::StencilRef
//#cpp2rust D3DRS_STENCILMASK                   = d3d::RS::StencilMask
//#cpp2rust D3DRS_STENCILWRITEMASK              = d3d::RS::StencilWriteMask
//#cpp2rust D3DRS_TEXTUREFACTOR                 = d3d::RS::TextureFactor
//#cpp2rust D3DRS_WRAP0                         = d3d::RS::Wrap0
//#cpp2rust D3DRS_WRAP1                         = d3d::RS::Wrap1
//#cpp2rust D3DRS_WRAP2                         = d3d::RS::Wrap2
//#cpp2rust D3DRS_WRAP3                         = d3d::RS::Wrap3
//#cpp2rust D3DRS_WRAP4                         = d3d::RS::Wrap4
//#cpp2rust D3DRS_WRAP5                         = d3d::RS::Wrap5
//#cpp2rust D3DRS_WRAP6                         = d3d::RS::Wrap6
//#cpp2rust D3DRS_WRAP7                         = d3d::RS::Wrap7
//#cpp2rust D3DRS_CLIPPING                      = d3d::RS::Clipping
//#cpp2rust D3DRS_LIGHTING                      = d3d::RS::Lighting
//#cpp2rust D3DRS_AMBIENT                       = d3d::RS::Ambient
//#cpp2rust D3DRS_FOGVERTEXMODE                 = d3d::RS::FogVertexMode
//#cpp2rust D3DRS_COLORVERTEX                   = d3d::RS::ColorVertex
//#cpp2rust D3DRS_LOCALVIEWER                   = d3d::RS::LocalViewer
//#cpp2rust D3DRS_NORMALIZENORMALS              = d3d::RS::NormalizeNormals
//#cpp2rust D3DRS_DIFFUSEMATERIALSOURCE         = d3d::RS::DiffuseMaterialSource
//#cpp2rust D3DRS_SPECULARMATERIALSOURCE        = d3d::RS::SpecularMaterialSource
//#cpp2rust D3DRS_AMBIENTMATERIALSOURCE         = d3d::RS::AmbientMaterialSource
//#cpp2rust D3DRS_EMISSIVEMATERIALSOURCE        = d3d::RS::EmissiveMaterialSource
//#cpp2rust D3DRS_VERTEXBLEND                   = d3d::RS::VertexBlend
//#cpp2rust D3DRS_CLIPPLANEENABLE               = d3d::RS::ClipPlaneEnable
//#cpp2rust D3DRS_POINTSIZE                     = d3d::RS::PointSize
//#cpp2rust D3DRS_POINTSIZE_MIN                 = d3d::RS::PointSizeMin
//#cpp2rust D3DRS_POINTSPRITEENABLE             = d3d::RS::PointSpriteEnable
//#cpp2rust D3DRS_POINTSCALEENABLE              = d3d::RS::PointScaleEnable
//#cpp2rust D3DRS_POINTSCALE_A                  = d3d::RS::PointScaleA
//#cpp2rust D3DRS_POINTSCALE_B                  = d3d::RS::PointScaleB
//#cpp2rust D3DRS_POINTSCALE_C                  = d3d::RS::PointScaleC
//#cpp2rust D3DRS_MULTISAMPLEANTIALIAS          = d3d::RS::MultiSampleAntiAlias
//#cpp2rust D3DRS_MULTISAMPLEMASK               = d3d::RS::MultiSampleMask
//#cpp2rust D3DRS_PATCHEDGESTYLE                = d3d::RS::PatchEdgeStyle
//#cpp2rust D3DRS_DEBUGMONITORTOKEN             = d3d::RS::DebugMonitorToken
//#cpp2rust D3DRS_POINTSIZE_MAX                 = d3d::RS::PointSizeMax
//#cpp2rust D3DRS_INDEXEDVERTEXBLENDENABLE      = d3d::RS::IndexedVertexBlendEnable
//#cpp2rust D3DRS_COLORWRITEENABLE              = d3d::RS::ColorWriteEnable
//#cpp2rust D3DRS_TWEENFACTOR                   = d3d::RS::TweenFactor
//#cpp2rust D3DRS_BLENDOP                       = d3d::RS::BlendOp
//#cpp2rust D3DRS_POSITIONDEGREE                = d3d::RS::PositionDegree
//#cpp2rust D3DRS_NORMALDEGREE                  = d3d::RS::NormalDegree
//#cpp2rust D3DRS_SCISSORTESTENABLE             = d3d::RS::ScissorTestEnable
//#cpp2rust D3DRS_SLOPESCALEDEPTHBIAS           = d3d::RS::SlopeScaleDepthBias
//#cpp2rust D3DRS_ANTIALIASEDLINEENABLE         = d3d::RS::AntiAliasedLineEnable
//#cpp2rust D3DRS_MINTESSELLATIONLEVEL          = d3d::RS::MinTessellationLevel
//#cpp2rust D3DRS_MAXTESSELLATIONLEVEL          = d3d::RS::MaxTessellationLevel
//#cpp2rust D3DRS_ADAPTIVETESS_X                = d3d::RS::AdaptiveTessX
//#cpp2rust D3DRS_ADAPTIVETESS_Y                = d3d::RS::AdaptiveTessY
//#cpp2rust D3DRS_ADAPTIVETESS_Z                = d3d::RS::AdaptiveTessZ
//#cpp2rust D3DRS_ADAPTIVETESS_W                = d3d::RS::AdaptiveTessW
//#cpp2rust D3DRS_ENABLEADAPTIVETESSELLATION    = d3d::RS::EnableAdaptiveTessellation
//#cpp2rust D3DRS_TWOSIDEDSTENCILMODE           = d3d::RS::TwoSidedStencilMode
//#cpp2rust D3DRS_CCW_STENCILFAIL               = d3d::RS::CcwStencilFail
//#cpp2rust D3DRS_CCW_STENCILZFAIL              = d3d::RS::CcwStencilZFail
//#cpp2rust D3DRS_CCW_STENCILPASS               = d3d::RS::CcwStencilPass
//#cpp2rust D3DRS_CCW_STENCILFUNC               = d3d::RS::CcwStencilFunc
//#cpp2rust D3DRS_COLORWRITEENABLE1             = d3d::RS::ColorWriteEnable1
//#cpp2rust D3DRS_COLORWRITEENABLE2             = d3d::RS::ColorWriteEnable2
//#cpp2rust D3DRS_COLORWRITEENABLE3             = d3d::RS::ColorWriteEnable3
//#cpp2rust D3DRS_BLENDFACTOR                   = d3d::RS::BlendFactor
//#cpp2rust D3DRS_SRGBWRITEENABLE               = d3d::RS::SRGBWriteEnable
//#cpp2rust D3DRS_DEPTHBIAS                     = d3d::RS::DepthBias
//#cpp2rust D3DRS_WRAP8                         = d3d::RS::Wrap8
//#cpp2rust D3DRS_WRAP9                         = d3d::RS::Wrap9
//#cpp2rust D3DRS_WRAP10                        = d3d::RS::Wrap10
//#cpp2rust D3DRS_WRAP11                        = d3d::RS::Wrap11
//#cpp2rust D3DRS_WRAP12                        = d3d::RS::Wrap12
//#cpp2rust D3DRS_WRAP13                        = d3d::RS::Wrap13
//#cpp2rust D3DRS_WRAP14                        = d3d::RS::Wrap14
//#cpp2rust D3DRS_WRAP15                        = d3d::RS::Wrap15
//#cpp2rust D3DRS_SEPARATEALPHABLENDENABLE      = d3d::RS::SeparateAlphaBlendEnable
//#cpp2rust D3DRS_SRCBLENDALPHA                 = d3d::RS::SrcBlendAlpha
//#cpp2rust D3DRS_DESTBLENDALPHA                = d3d::RS::DestBlendAlpha
//#cpp2rust D3DRS_BLENDOPALPHA                  = d3d::RS::BlendOpAlpha
