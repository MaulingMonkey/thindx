#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype)\]
/// D3DRENDERSTATETYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RenderStateType(D3DRENDERSTATETYPE);
pub type RS = RenderStateType;

impl RenderStateType {
    /// Convert a raw [D3DRENDERSTATETYPE] value into a [RenderStateType].  This is *probably* safe... probably....
    ///
    /// [D3DRENDERSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype
    pub const fn from_unchecked(renderstatetype: D3DRENDERSTATETYPE) -> Self { Self(renderstatetype) }

    /// Convert a [RenderStateType] into a raw [D3DRENDERSTATETYPE].
    ///
    /// [D3DRENDERSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype
    pub const fn into(self) -> D3DRENDERSTATETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl RenderStateType {
    pub const ZEnable                       : RenderStateType = RenderStateType(D3DRS_ZENABLE);
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

impl Debug for RenderStateType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            RenderStateType::ZEnable                    => write!(f, "RenderStateType::ZEnable"),
            RenderStateType::FillMode                   => write!(f, "RenderStateType::FillMode"),
            RenderStateType::ShadeMode                  => write!(f, "RenderStateType::ShadeMode"),
            RenderStateType::ZWriteEnable               => write!(f, "RenderStateType::ZWriteEnable"),
            RenderStateType::AlphaTestEnable            => write!(f, "RenderStateType::AlphaTestEnable"),
            RenderStateType::LastPixel                  => write!(f, "RenderStateType::LastPixel"),
            RenderStateType::SrcBlend                   => write!(f, "RenderStateType::SrcBlend"),
            RenderStateType::DestBlend                  => write!(f, "RenderStateType::DestBlend"),
            RenderStateType::CullMode                   => write!(f, "RenderStateType::CullMode"),
            RenderStateType::ZFunc                      => write!(f, "RenderStateType::ZFunc"),
            RenderStateType::AlphaRef                   => write!(f, "RenderStateType::AlphaRef"),
            RenderStateType::AlphaFunc                  => write!(f, "RenderStateType::AlphaFunc"),
            RenderStateType::DitherEnable               => write!(f, "RenderStateType::DitherEnable"),
            RenderStateType::AlphaBlendEnable           => write!(f, "RenderStateType::AlphaBlendEnable"),
            RenderStateType::FogEnable                  => write!(f, "RenderStateType::FogEnable"),
            RenderStateType::SpecularEnable             => write!(f, "RenderStateType::SpecularEnable"),
            RenderStateType::FogColor                   => write!(f, "RenderStateType::FogColor"),
            RenderStateType::FogTableMode               => write!(f, "RenderStateType::FogTableMode"),
            RenderStateType::FogStart                   => write!(f, "RenderStateType::FogStart"),
            RenderStateType::FogEnd                     => write!(f, "RenderStateType::FogEnd"),
            RenderStateType::FogDensity                 => write!(f, "RenderStateType::FogDensity"),
            RenderStateType::RangeFogEnable             => write!(f, "RenderStateType::RangeFogEnable"),
            RenderStateType::StencilEnable              => write!(f, "RenderStateType::StencilEnable"),
            RenderStateType::StencilFail                => write!(f, "RenderStateType::StencilFail"),
            RenderStateType::StencilZFail               => write!(f, "RenderStateType::StencilZFail"),
            RenderStateType::StencilPass                => write!(f, "RenderStateType::StencilPass"),
            RenderStateType::StencilFunc                => write!(f, "RenderStateType::StencilFunc"),
            RenderStateType::StencilRef                 => write!(f, "RenderStateType::StencilRef"),
            RenderStateType::StencilMask                => write!(f, "RenderStateType::StencilMask"),
            RenderStateType::StencilWriteMask           => write!(f, "RenderStateType::StencilWriteMask"),
            RenderStateType::TextureFactor              => write!(f, "RenderStateType::TextureFactor"),
            RenderStateType::Wrap0                      => write!(f, "RenderStateType::Wrap0"),
            RenderStateType::Wrap1                      => write!(f, "RenderStateType::Wrap1"),
            RenderStateType::Wrap2                      => write!(f, "RenderStateType::Wrap2"),
            RenderStateType::Wrap3                      => write!(f, "RenderStateType::Wrap3"),
            RenderStateType::Wrap4                      => write!(f, "RenderStateType::Wrap4"),
            RenderStateType::Wrap5                      => write!(f, "RenderStateType::Wrap5"),
            RenderStateType::Wrap6                      => write!(f, "RenderStateType::Wrap6"),
            RenderStateType::Wrap7                      => write!(f, "RenderStateType::Wrap7"),
            RenderStateType::Clipping                   => write!(f, "RenderStateType::Clipping"),
            RenderStateType::Lighting                   => write!(f, "RenderStateType::Lighting"),
            RenderStateType::Ambient                    => write!(f, "RenderStateType::Ambient"),
            RenderStateType::FogVertexMode              => write!(f, "RenderStateType::FogVertexMode"),
            RenderStateType::ColorVertex                => write!(f, "RenderStateType::ColorVertex"),
            RenderStateType::LocalViewer                => write!(f, "RenderStateType::LocalViewer"),
            RenderStateType::NormalizeNormals           => write!(f, "RenderStateType::NormalizeNormals"),
            RenderStateType::DiffuseMaterialSource      => write!(f, "RenderStateType::DiffuseMaterialSource"),
            RenderStateType::SpecularMaterialSource     => write!(f, "RenderStateType::SpecularMaterialSource"),
            RenderStateType::AmbientMaterialSource      => write!(f, "RenderStateType::AmbientMaterialSource"),
            RenderStateType::EmissiveMaterialSource     => write!(f, "RenderStateType::EmissiveMaterialSource"),
            RenderStateType::VertexBlend                => write!(f, "RenderStateType::VertexBlend"),
            RenderStateType::ClipPlaneEnable            => write!(f, "RenderStateType::ClipPlaneEnable"),
            RenderStateType::PointSize                  => write!(f, "RenderStateType::PointSize"),
            RenderStateType::PointSizeMin               => write!(f, "RenderStateType::PointSizeMin"),
            RenderStateType::PointSpriteEnable          => write!(f, "RenderStateType::PointSpriteEnable"),
            RenderStateType::PointScaleEnable           => write!(f, "RenderStateType::PointScaleEnable"),
            RenderStateType::PointScaleA                => write!(f, "RenderStateType::PointScaleA"),
            RenderStateType::PointScaleB                => write!(f, "RenderStateType::PointScaleB"),
            RenderStateType::PointScaleC                => write!(f, "RenderStateType::PointScaleC"),
            RenderStateType::MultiSampleAntiAlias       => write!(f, "RenderStateType::MultiSampleAntiAlias"),
            RenderStateType::MultiSampleMask            => write!(f, "RenderStateType::MultiSampleMask"),
            RenderStateType::PatchEdgeStyle             => write!(f, "RenderStateType::PatchEdgeStyle"),
            RenderStateType::DebugMonitorToken          => write!(f, "RenderStateType::DebugMonitorToken"),
            RenderStateType::PointSizeMax               => write!(f, "RenderStateType::PointSizeMax"),
            RenderStateType::IndexedVertexBlendEnable   => write!(f, "RenderStateType::IndexedVertexBlendEnable"),
            RenderStateType::ColorWriteEnable           => write!(f, "RenderStateType::ColorWriteEnable"),
            RenderStateType::TweenFactor                => write!(f, "RenderStateType::TweenFactor"),
            RenderStateType::BlendOp                    => write!(f, "RenderStateType::BlendOp"),
            RenderStateType::PositionDegree             => write!(f, "RenderStateType::PositionDegree"),
            RenderStateType::NormalDegree               => write!(f, "RenderStateType::NormalDegree"),
            RenderStateType::ScissorTestEnable          => write!(f, "RenderStateType::ScissorTestEnable"),
            RenderStateType::SlopeScaleDepthBias        => write!(f, "RenderStateType::SlopeScaleDepthBias"),
            RenderStateType::AntiAliasedLineEnable      => write!(f, "RenderStateType::AntiAliasedLineEnable"),
            RenderStateType::MinTessellationLevel       => write!(f, "RenderStateType::MinTessellationLevel"),
            RenderStateType::MaxTessellationLevel       => write!(f, "RenderStateType::MaxTessellationLevel"),
            RenderStateType::AdaptiveTessX              => write!(f, "RenderStateType::AdaptiveTessX"),
            RenderStateType::AdaptiveTessY              => write!(f, "RenderStateType::AdaptiveTessY"),
            RenderStateType::AdaptiveTessZ              => write!(f, "RenderStateType::AdaptiveTessZ"),
            RenderStateType::AdaptiveTessW              => write!(f, "RenderStateType::AdaptiveTessW"),
            RenderStateType::EnableAdaptiveTessellation => write!(f, "RenderStateType::EnableAdaptiveTessellation"),
            RenderStateType::TwoSidedStencilMode        => write!(f, "RenderStateType::TwoSidedStencilMode"),
            RenderStateType::CcwStencilFail             => write!(f, "RenderStateType::CcwStencilFail"),
            RenderStateType::CcwStencilZFail            => write!(f, "RenderStateType::CcwStencilZFail"),
            RenderStateType::CcwStencilPass             => write!(f, "RenderStateType::CcwStencilPass"),
            RenderStateType::CcwStencilFunc             => write!(f, "RenderStateType::CcwStencilFunc"),
            RenderStateType::ColorWriteEnable1          => write!(f, "RenderStateType::ColorWriteEnable1"),
            RenderStateType::ColorWriteEnable2          => write!(f, "RenderStateType::ColorWriteEnable2"),
            RenderStateType::ColorWriteEnable3          => write!(f, "RenderStateType::ColorWriteEnable3"),
            RenderStateType::BlendFactor                => write!(f, "RenderStateType::BlendFactor"),
            RenderStateType::SRGBWriteEnable            => write!(f, "RenderStateType::SRGBWriteEnable"),
            RenderStateType::DepthBias                  => write!(f, "RenderStateType::DepthBias"),
            RenderStateType::Wrap8                      => write!(f, "RenderStateType::Wrap8"),
            RenderStateType::Wrap9                      => write!(f, "RenderStateType::Wrap9"),
            RenderStateType::Wrap10                     => write!(f, "RenderStateType::Wrap10"),
            RenderStateType::Wrap11                     => write!(f, "RenderStateType::Wrap11"),
            RenderStateType::Wrap12                     => write!(f, "RenderStateType::Wrap12"),
            RenderStateType::Wrap13                     => write!(f, "RenderStateType::Wrap13"),
            RenderStateType::Wrap14                     => write!(f, "RenderStateType::Wrap14"),
            RenderStateType::Wrap15                     => write!(f, "RenderStateType::Wrap15"),
            RenderStateType::SeparateAlphaBlendEnable   => write!(f, "RenderStateType::SeparateAlphaBlendEnable"),
            RenderStateType::SrcBlendAlpha              => write!(f, "RenderStateType::SrcBlendAlpha"),
            RenderStateType::DestBlendAlpha             => write!(f, "RenderStateType::DestBlendAlpha"),
            RenderStateType::BlendOpAlpha               => write!(f, "RenderStateType::BlendOpAlpha"),
            other                                       => write!(f, "RenderStateType({})", other.0),
        }
    }
}

impl From<RenderStateType> for D3DRENDERSTATETYPE {
    fn from(value: RenderStateType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DRENDERSTATETYPE> for RenderStateType {
    fn from(value: D3DRENDERSTATETYPE) -> Self { Self(value) }
}
