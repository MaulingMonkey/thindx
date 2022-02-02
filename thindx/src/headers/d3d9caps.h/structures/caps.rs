use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;

// use std::fmt::{self, Debug, Formatter}; // TODO: a super awesome Debug impl
use std::ops::{Deref, DerefMut};

#[allow(non_camel_case_types)] type dword  = u32;
#[allow(non_camel_case_types)] type uint   = u32;
#[allow(non_camel_case_types)] type int    = i32;
#[allow(non_camel_case_types)] type float  = f32;
type AdapterOrdinal = uint; // FIXME: ???



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DCAPS9
///
/// Contains various information about an adapter/device's capabilities and limitations
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Caps {
    // Device Info

    /// Typically [DevType::HAL]
    pub device_type:                                    DevType,

    /// Typically `0`, but could be something higher on multi-screen / multi-GPU systems.
    pub adapter_ordinal:                                AdapterOrdinal, // UINT

    // Caps from DX7 Draw

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [Caps1]::{[ReadScanline](Caps1::ReadScanline)}
    /// | (other)   | [Caps1]::{[Overlay](Caps1::Overlay)}
    pub caps:                                           Caps1,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [Caps2]::{[CanAutoGenMipMap](Caps2::CanAutoGenMipMap) \|&nbsp;[CanShareResource](Caps2::CanShareResource) \|&nbsp;[DynamicTextures](Caps2::DynamicTextures) \|&nbsp;[FullScreenGamma](Caps2::FullScreenGamma)}
    /// | (other)   | [Caps2]::{[CanCalibrateGamma](Caps2::CanCalibrateGamma) \|&nbsp;[CanManageResource](Caps2::CanManageResource)}
    pub caps2:                                          Caps2,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [Caps3]::{[AlphaFullscreenFlipOrDiscard](Caps3::AlphaFullscreenFlipOrDiscard) \|&nbsp;[CopyToVidMem](Caps3::CopyToVidMem) \|&nbsp;[CopyToSystemMem](Caps3::CopyToSystemMem) \|&nbsp;[LinearToSrgbPresentation](Caps3::LinearToSrgbPresentation)}
    /// | (other)   | [Caps3]::{[DxvaHD](Caps3::DxvaHD)}
    pub caps3:                                          Caps3,

    /// Mask of available presentation swap intervals.
    ///
    /// Intervals documented by caps include: [Present]::{[IntervalImmediate](Present::IntervalImmediate) \|&nbsp;[IntervalOne](Present::IntervalOne) \|&nbsp;... \|&nbsp;[IntervalFour](Present::IntervalFour)}
    ///
    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [Present]::{[IntervalOne](Present::IntervalOne) \|&nbsp;... \|&nbsp;[IntervalFour](Present::IntervalFour) \|&nbsp;[IntervalImmediate](Present::IntervalImmediate)}
    /// | (other)   | None?
    pub presentation_intervals:                         Present,

    // Cursor Caps

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [CursorCaps]::{[Color](CursorCaps::Color)}
    /// | (other)   | [CursorCaps]::{[LowRes](CursorCaps::LowRes)}
    pub cursor_caps:                                    CursorCaps,

    // 3D Device Caps

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [DevCaps]::{[CanBltSysToNonLocal](DevCaps::CanBltSysToNonLocal) \|&nbsp;[CanRenderAfterFlip](DevCaps::CanRenderAfterFlip) \|&nbsp;[DrawPrimitives2](DevCaps::DrawPrimitives2) \|&nbsp;[DrawPrimitives2Ex](DevCaps::DrawPrimitives2Ex) \|&nbsp;[DrawPrimTlVertex](DevCaps::DrawPrimTlVertex) \|&nbsp;[ExecuteSystemMemory](DevCaps::ExecuteSystemMemory) \|&nbsp;[ExecuteVideoMemory](DevCaps::ExecuteVideoMemory) \|&nbsp;[HwRasterization](DevCaps::HwRasterization) \|&nbsp;[HwTransformAndLight](DevCaps::HwTransformAndLight) \|&nbsp;[PureDevice](DevCaps::PureDevice) \|&nbsp;[TextureNonLocalVidMem](DevCaps::TextureNonLocalVidMem) \|&nbsp;[TextureVideoMemory](DevCaps::TextureVideoMemory) \|&nbsp;[TlVertexSystemMemory](DevCaps::TlVertexSystemMemory) \|&nbsp;[TlVertexVideoMemory](DevCaps::TlVertexVideoMemory)}
    pub dev_caps:                                       DevCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PMiscCaps]::{[MaskZ](PMiscCaps::MaskZ) \|&nbsp;[CullNone](PMiscCaps::CullNone) \|&nbsp;[CullCW](PMiscCaps::CullCW) \|&nbsp;[CullCCW](PMiscCaps::CullCCW) \|&nbsp;[ColorWriteEnable](PMiscCaps::ColorWriteEnable) \|&nbsp;[TssArgTemp](PMiscCaps::TssArgTemp) \|&nbsp;[BlendOp](PMiscCaps::BlendOp) \|&nbsp;[IndependentWriteMasks](PMiscCaps::IndependentWriteMasks) \|&nbsp;[PerStageConstant](PMiscCaps::PerStageConstant) \|&nbsp;[PostBlendSrgbConvert](PMiscCaps::PostBlendSrgbConvert) \|&nbsp;[FogAndSpecularAlpha](PMiscCaps::FogAndSpecularAlpha) \|&nbsp;[SeparateAlphaBlend](PMiscCaps::SeparateAlphaBlend) \|&nbsp;[MrtIndependentBitDepths](PMiscCaps::MrtIndependentBitDepths) \|&nbsp;[MrtPostPixelShaderBlending](PMiscCaps::MrtPostPixelShaderBlending) \|&nbsp;[FogVertexClamped](PMiscCaps::FogVertexClamped)}
    pub primitive_misc_caps:                            PMiscCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PRasterCaps]::{[Anisotropy](PRasterCaps::Anisotropy) \|&nbsp;[ColorPerspective](PRasterCaps::ColorPerspective) \|&nbsp;[Dither](PRasterCaps::Dither) \|&nbsp;[DepthBias](PRasterCaps::DepthBias) \|&nbsp;[FogRange](PRasterCaps::FogRange) \|&nbsp;[FogTable](PRasterCaps::FogTable) \|&nbsp;[FogVertex](PRasterCaps::FogVertex) \|&nbsp;[MipMapLodBias](PRasterCaps::MipMapLodBias) \|&nbsp;[MultisampleToggle](PRasterCaps::MultisampleToggle) \|&nbsp;[ScissorTest](PRasterCaps::ScissorTest) \|&nbsp;[SlopeScaleDepthBias](PRasterCaps::SlopeScaleDepthBias) \|&nbsp;[WFog](PRasterCaps::WFog) \|&nbsp;[ZFog](PRasterCaps::ZFog) \|&nbsp;[ZTest](PRasterCaps::ZTest) \|&nbsp;[ZBias](PRasterCaps::ZBias)}
    pub raster_caps:                                    PRasterCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PCmpCaps]::{[Always](PCmpCaps::Always) \|&nbsp;[Equal](PCmpCaps::Equal) \|&nbsp;[Greater](PCmpCaps::Greater) \|&nbsp;[GreaterEqual](PCmpCaps::GreaterEqual) \|&nbsp;[Less](PCmpCaps::Less) \|&nbsp;[LessEqual](PCmpCaps::LessEqual) \|&nbsp;[Never](PCmpCaps::Never) \|&nbsp;[NotEqual](PCmpCaps::NotEqual)}
    pub z_cmp_caps:                                     PCmpCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PBlendCaps]::{[BlendFactor](PBlendCaps::BlendFactor) \|&nbsp;[BothInvSrcAlpha](PBlendCaps::BothInvSrcAlpha) \|&nbsp;[BothSrcAlpha](PBlendCaps::BothSrcAlpha) \|&nbsp;[DestAlpha](PBlendCaps::DestAlpha) \|&nbsp;[DestColor](PBlendCaps::DestColor) \|&nbsp;[InvDestAlpha](PBlendCaps::InvDestAlpha) \|&nbsp;[InvDestColor](PBlendCaps::InvDestColor) \|&nbsp;[InvSrcAlpha](PBlendCaps::InvSrcAlpha) \|&nbsp;[InvSrcColor](PBlendCaps::InvSrcColor) \|&nbsp;[One](PBlendCaps::One) \|&nbsp;[SrcAlpha](PBlendCaps::SrcAlpha) \|&nbsp;[SrcAlphaSat](PBlendCaps::SrcAlphaSat) \|&nbsp;[SrcColor](PBlendCaps::SrcColor) \|&nbsp;[Zero](PBlendCaps::Zero)}
    pub src_blend_caps:                                 PBlendCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PBlendCaps]::{[BlendFactor](PBlendCaps::BlendFactor) \|&nbsp;[BothInvSrcAlpha](PBlendCaps::BothInvSrcAlpha) \|&nbsp;[BothSrcAlpha](PBlendCaps::BothSrcAlpha) \|&nbsp;[DestAlpha](PBlendCaps::DestAlpha) \|&nbsp;[DestColor](PBlendCaps::DestColor) \|&nbsp;[InvDestAlpha](PBlendCaps::InvDestAlpha) \|&nbsp;[InvDestColor](PBlendCaps::InvDestColor) \|&nbsp;[InvSrcAlpha](PBlendCaps::InvSrcAlpha) \|&nbsp;[InvSrcColor](PBlendCaps::InvSrcColor) \|&nbsp;[One](PBlendCaps::One) \|&nbsp;[SrcAlpha](PBlendCaps::SrcAlpha) \|&nbsp;[SrcAlphaSat](PBlendCaps::SrcAlphaSat) \|&nbsp;[SrcColor](PBlendCaps::SrcColor) \|&nbsp;[Zero](PBlendCaps::Zero)}
    pub dest_blend_caps:                                PBlendCaps,

    /// Alpha-test comparison capabilities. This member can include the same capability flags defined for the ZCmpCaps member.
    /// If this member contains only the [PCmpCaps::Always] or only the [PCmpCaps::Never] capability, the driver does not support alpha tests.
    /// Otherwise, the flags identify the individual comparisons that are supported for alpha testing.
    ///
    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PCmpCaps]::{[Always](PCmpCaps::Always) \|&nbsp;[Equal](PCmpCaps::Equal) \|&nbsp;[Greater](PCmpCaps::Greater) \|&nbsp;[GreaterEqual](PCmpCaps::GreaterEqual) \|&nbsp;[Less](PCmpCaps::Less) \|&nbsp;[LessEqual](PCmpCaps::LessEqual) \|&nbsp;[Never](PCmpCaps::Never) \|&nbsp;[NotEqual](PCmpCaps::NotEqual)}
    pub alpha_cmp_caps:                                 PCmpCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PShadeCaps]::{[AlphaGouraudBlend](PShadeCaps::AlphaGouraudBlend) \|&nbsp;[ColorGouraudRgb](PShadeCaps::ColorGouraudRgb) \|&nbsp;[FogGouraud](PShadeCaps::FogGouraud) \|&nbsp;[SpecularGouraudRgb](PShadeCaps::SpecularGouraudRgb)}
    pub shade_caps:                                     PShadeCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTextureCaps]::{[Alpha](PTextureCaps::Alpha) \|&nbsp;[CubeMap](PTextureCaps::CubeMap) \|&nbsp;[MipCubeMap](PTextureCaps::MipCubeMap) \|&nbsp;[MipMap](PTextureCaps::MipMap) \|&nbsp;[MipVolumeMap](PTextureCaps::MipVolumeMap) \|&nbsp;[Perspective](PTextureCaps::Perspective) \|&nbsp;[Projected](PTextureCaps::Projected) \|&nbsp;[TexRepeatNotScaledBySize](PTextureCaps::TexRepeatNotScaledBySize) \|&nbsp;[VolumeMap](PTextureCaps::VolumeMap)}
    pub texture_caps:                                   PTextureCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTFilterCaps]::{[MagFPoint](PTFilterCaps::MagFPoint) \|&nbsp;[MagFLinear](PTFilterCaps::MagFLinear) \|&nbsp;[MagFAnisotropic](PTFilterCaps::MagFAnisotropic) \|&nbsp;[MinFPoint](PTFilterCaps::MinFPoint) \|&nbsp;[MinFLinear](PTFilterCaps::MinFLinear) \|&nbsp;[MinFAnisotropic](PTFilterCaps::MinFAnisotropic) \|&nbsp;[MipFPoint](PTFilterCaps::MipFPoint) \|&nbsp;[MipFLinear](PTFilterCaps::MipFLinear)}
    pub texture_filter_caps:                            PTFilterCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTFilterCaps]::{[MagFPoint](PTFilterCaps::MagFPoint) \|&nbsp;[MagFLinear](PTFilterCaps::MagFLinear) \|&nbsp;[MinFPoint](PTFilterCaps::MinFPoint) \|&nbsp;[MinFLinear](PTFilterCaps::MinFLinear) \|&nbsp;[MipFPoint](PTFilterCaps::MipFPoint) \|&nbsp;[MipFLinear](PTFilterCaps::MipFLinear)}
    pub cube_texture_filter_caps:                       PTFilterCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTFilterCaps]::{[MagFPoint](PTFilterCaps::MagFPoint) \|&nbsp;[MagFLinear](PTFilterCaps::MagFLinear) \|&nbsp;[MagFAnisotropic](PTFilterCaps::MagFAnisotropic) \|&nbsp;[MinFPoint](PTFilterCaps::MinFPoint) \|&nbsp;[MinFLinear](PTFilterCaps::MinFLinear) \|&nbsp;[MinFAnisotropic](PTFilterCaps::MinFAnisotropic) \|&nbsp;[MipFPoint](PTFilterCaps::MipFPoint) \|&nbsp;[MipFLinear](PTFilterCaps::MipFLinear)}
    pub volume_texture_filter_caps:                     PTFilterCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTAddressCaps]::{[Border](PTAddressCaps::Border) \|&nbsp;[Clamp](PTAddressCaps::Clamp) \|&nbsp;[IndependentUV](PTAddressCaps::IndependentUV) \|&nbsp;[Mirror](PTAddressCaps::Mirror) \|&nbsp;[MirrorOnce](PTAddressCaps::MirrorOnce) \|&nbsp;[Wrap](PTAddressCaps::Wrap)}
    pub texture_address_caps:                           PTAddressCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTAddressCaps]::{[Border](PTAddressCaps::Border) \|&nbsp;[Clamp](PTAddressCaps::Clamp) \|&nbsp;[IndependentUV](PTAddressCaps::IndependentUV) \|&nbsp;[Mirror](PTAddressCaps::Mirror) \|&nbsp;[MirrorOnce](PTAddressCaps::MirrorOnce) \|&nbsp;[Wrap](PTAddressCaps::Wrap)}
    pub volume_texture_address_caps:                    PTAddressCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [LineCaps]::{[AlphaCmp](LineCaps::AlphaCmp) \|&nbsp;[Blend](LineCaps::Blend) \|&nbsp;[Fog](LineCaps::Fog) \|&nbsp;[Texture](LineCaps::Texture) \|&nbsp;[ZTest](LineCaps::ZTest)}
    pub line_caps:                                      LineCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 16384
    pub max_texture_width:                              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 16384
    pub max_texture_height:                             dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8192
    pub max_volume_extent:                              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8192
    pub max_texture_repeat:                             dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8192
    pub max_texture_aspect_ratio:                       dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 16
    pub max_anisotropy:                                 dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 10000000000.0
    pub max_vertex_w:                                   float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | -32768.0
    pub guard_band_left:                                float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | -32768.0
    pub guard_band_top:                                 float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 32768.0
    pub guard_band_right:                               float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 32768.0
    pub guard_band_bottom:                              float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 0.0
    pub extents_adjust:                                 float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [StencilCaps]::{[Keep](StencilCaps::Keep) \|&nbsp;[Zero](StencilCaps::Zero) \|&nbsp;[Replace](StencilCaps::Replace) \|&nbsp;[IncrSat](StencilCaps::IncrSat) \|&nbsp;[DecrSat](StencilCaps::DecrSat) \|&nbsp;[Invert](StencilCaps::Invert) \|&nbsp;[Incr](StencilCaps::Incr) \|&nbsp;[Decr](StencilCaps::Decr) \|&nbsp;[TwoSided](StencilCaps::TwoSided)}
    pub stencil_caps:                                   StencilCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [FvfCaps]::{[PSize](FvfCaps::PSize) \|&nbsp;0x00000008}
    pub fvf_caps:                                       FvfCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [TexOpCaps]::{[Add](TexOpCaps::Add) \|&nbsp;[AddSigned](TexOpCaps::AddSigned) \|&nbsp;[AddSigned2x](TexOpCaps::AddSigned2x) \|&nbsp;[AddSmooth](TexOpCaps::AddSmooth) \|&nbsp;[BlendCurrentAlpha](TexOpCaps::BlendCurrentAlpha) \|&nbsp;[BlendDiffuseAlpha](TexOpCaps::BlendDiffuseAlpha) \|&nbsp;[BlendFactorAlpha](TexOpCaps::BlendFactorAlpha) \|&nbsp;[BlendTextureAlpha](TexOpCaps::BlendTextureAlpha) \|&nbsp;[BlendTextureAlphaPM](TexOpCaps::BlendTextureAlphaPM) \|&nbsp;[BumpEnvMap](TexOpCaps::BumpEnvMap) \|&nbsp;[BumpEnvMapLuminance](TexOpCaps::BumpEnvMapLuminance) \|&nbsp;[Disable](TexOpCaps::Disable) \|&nbsp;[DotProduct3](TexOpCaps::DotProduct3) \|&nbsp;[Lerp](TexOpCaps::Lerp) \|&nbsp;[Modulate](TexOpCaps::Modulate) \|&nbsp;[Modulate2x](TexOpCaps::Modulate2x) \|&nbsp;[Modulate4x](TexOpCaps::Modulate4x) \|&nbsp;[ModulateAlphaAddColor](TexOpCaps::ModulateAlphaAddColor) \|&nbsp;[ModulateColorAddAlpha](TexOpCaps::ModulateColorAddAlpha) \|&nbsp;[ModulateInvAlphaAddColor](TexOpCaps::ModulateInvAlphaAddColor) \|&nbsp;[ModulateInvColorAddAlpha](TexOpCaps::ModulateInvColorAddAlpha) \|&nbsp;[MultiplyAdd](TexOpCaps::MultiplyAdd) \|&nbsp;[Premodulate](TexOpCaps::Premodulate) \|&nbsp;[SelectArg1](TexOpCaps::SelectArg1) \|&nbsp;[SelectArg2](TexOpCaps::SelectArg2) \|&nbsp;[Subtract](TexOpCaps::Subtract)}
    pub texture_op_caps:                                TexOpCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8
    pub max_texture_blend_stages:                       dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8
    pub max_simultaneous_textures:                      dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [VtxPCaps]::{[DirectionalLights](VtxPCaps::DirectionalLights) \|&nbsp;[LocalViewer](VtxPCaps::LocalViewer) \|&nbsp;[MaterialSource7](VtxPCaps::MaterialSource7) \|&nbsp;[PositionalLights](VtxPCaps::PositionalLights) \|&nbsp;[TexGen](VtxPCaps::TexGen) \|&nbsp;[TexGenSphereMap](VtxPCaps::TexGenSphereMap) \|&nbsp;[Tweening](VtxPCaps::Tweening)}
    pub vertex_processing_caps:                         VtxPCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 10
    pub max_active_lights:                              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 6
    pub max_user_clip_planes:                           dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 4
    pub max_vertex_blend_matrices:                      dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 8
    pub max_vertex_blend_matrix_index:                  dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 256.0
    pub max_point_size:                                 float,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 5592405
    pub max_primitive_count:                            dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 16777215
    pub max_vertex_index:                               dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 16
    pub max_streams:                                    dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 508
    pub max_stream_stride:                              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [ShaderVersion::VS_3_0]
    pub vertex_shader_version:                          ShaderVersion,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 256
    pub max_vertex_shader_const:                        dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [ShaderVersion::PS_3_0]
    pub pixel_shader_version:                           ShaderVersion,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 3.4028235e38
    pub pixel_shader_1x_max_value:                      float,

    // DX9 specific

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [DevCaps2]::{[CanStretchRectFromTextures](DevCaps2::CanStretchRectFromTextures) \|&nbsp;[PresampledDMapNPatch](DevCaps2::PresampledDMapNPatch) \|&nbsp;[StreamOffset](DevCaps2::StreamOffset) \|&nbsp;[VertexElementsCanShareStreamOffset](DevCaps2::VertexElementsCanShareStreamOffset)}
    pub dev_caps2:                                      DevCaps2,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 1.0
    pub max_npatch_tessellation_level:                  float,

    #[doc(hidden)]
    pub reserved5:                                      Reserved5,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 0
    pub master_adapter_ordinal:                         AdapterOrdinal,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 0
    pub adapter_ordinal_in_group:                       uint,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 4
    pub number_of_adapters_in_group:                    uint,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [DtCaps]::{[UByte4](DtCaps::UByte4) \|&nbsp;[UByte4N](DtCaps::UByte4N) \|&nbsp;[Short2N](DtCaps::Short2N) \|&nbsp;[Short4N](DtCaps::Short4N) \|&nbsp;[UShort2N](DtCaps::UShort2N) \|&nbsp;[UShort4N](DtCaps::UShort4N) \|&nbsp;[UDec3](DtCaps::UDec3) \|&nbsp;[Dec3N](DtCaps::Dec3N) \|&nbsp;[Float16_2](DtCaps::Float16_2) \|&nbsp;[Float16_4](DtCaps::Float16_4)}
    pub decl_types:                                     DtCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 4
    pub num_simultaneous_rts:                           dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTFilterCaps]::{[MagFPoint](PTFilterCaps::MagFPoint) \|&nbsp;[MagFLinear](PTFilterCaps::MagFLinear) \|&nbsp;[MinFPoint](PTFilterCaps::MinFPoint) \|&nbsp;[MinFLinear](PTFilterCaps::MinFLinear)}
    pub stretch_rect_filter_caps:                       PTFilterCaps, // just some subset

    /// <h3>Sacrilege</h3>
    ///
    /// [VShaderCaps20] {<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;caps: [Vs20Caps]::[Predication](Vs20Caps::Predication),<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;dynamic_flow_control_depth: 24,<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;num_temps: 32,<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;static_flow_control_depth: 4,<br>
    /// }<br>
    pub vs_20_caps:                                     VShaderCaps20,

    /// <h3>Sacrilege</h3>
    ///
    /// [PShaderCaps20] {<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;caps: [Ps20Caps]::{[ArbitrarySwizzle](Ps20Caps::ArbitrarySwizzle) \|&nbsp;[GradientInstructions](Ps20Caps::GradientInstructions) \|&nbsp;[Predication](Ps20Caps::Predication) \|&nbsp;[NoDependentReadLimit](Ps20Caps::NoDependentReadLimit) \|&nbsp;[NoTexInstructionLimit](Ps20Caps::NoTexInstructionLimit)},<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;dynamic_flow_control_depth: 24,<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;num_temps: 32,<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;static_flow_control_depth: 4,<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;num_instruction_slots: 512,<br>
    /// }
    pub ps_20_caps:                                     PShaderCaps20,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | [PTFilterCaps]::{[MagFPoint](PTFilterCaps::MagFPoint) \|&nbsp;[MagFLinear](PTFilterCaps::MagFLinear) \|&nbsp;[MinFPoint](PTFilterCaps::MinFPoint) \|&nbsp;[MinFLinear](PTFilterCaps::MinFLinear)}
    pub vertex_texture_filter_caps:                     PTFilterCaps,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 4294967295
    pub max_vshader_instructions_executed:              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 4294967295
    pub max_pshader_instructions_executed:              dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 32768
    pub max_vertex_shader_30_instruction_slots:         dword,

    /// | System    | Value     |
    /// | --------- | --------- |
    /// | Sacrilege | 32768
    pub max_pixel_shader_30_instruction_slots:          dword,
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dpshadercaps2_0)\]
/// D3DPSHADERCAPS2_0
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct PShaderCaps20 {
    pub caps:                                           Ps20Caps,
    pub dynamic_flow_control_depth:                     int,    // FIXME: type
    pub num_temps:                                      int,    // FIXME: type
    pub static_flow_control_depth:                      int,    // FIXME: type
    pub num_instruction_slots:                          int,    // FIXME: type
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dvshadercaps2_0)\]
/// D3DVSHADERCAPS2_0
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct VShaderCaps20 {
    pub caps:                                           Vs20Caps,
    pub dynamic_flow_control_depth:                     int,    // FIXME: type
    pub num_temps:                                      int,    // FIXME: type
    pub static_flow_control_depth:                      int,    // FIXME: type
}

impl Deref    for Caps { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DCAPS9; }
impl DerefMut for Caps { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DCAPS9> for Caps { fn from(value: D3DCAPS9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Caps> for D3DCAPS9 { fn from(value: Caps   ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! {
    Caps => D3DCAPS9 {
        device_type                                     => DeviceType,
        adapter_ordinal                                 => AdapterOrdinal,
        caps                                            => Caps,
        caps2                                           => Caps2,
        caps3                                           => Caps3,
        presentation_intervals                          => PresentationIntervals,
        cursor_caps                                     => CursorCaps,
        dev_caps                                        => DevCaps,
        primitive_misc_caps                             => PrimitiveMiscCaps,
        raster_caps                                     => RasterCaps,
        z_cmp_caps                                      => ZCmpCaps,
        src_blend_caps                                  => SrcBlendCaps,
        dest_blend_caps                                 => DestBlendCaps,
        alpha_cmp_caps                                  => AlphaCmpCaps,
        shade_caps                                      => ShadeCaps,
        texture_caps                                    => TextureCaps,
        texture_filter_caps                             => TextureFilterCaps,
        cube_texture_filter_caps                        => CubeTextureFilterCaps,
        volume_texture_filter_caps                      => VolumeTextureFilterCaps,
        texture_address_caps                            => TextureAddressCaps,
        volume_texture_address_caps                     => VolumeTextureAddressCaps,
        line_caps                                       => LineCaps,
        max_texture_width                               => MaxTextureWidth,
        max_texture_height                              => MaxTextureHeight,
        max_volume_extent                               => MaxVolumeExtent,
        max_texture_repeat                              => MaxTextureRepeat,
        max_texture_aspect_ratio                        => MaxTextureAspectRatio,
        max_anisotropy                                  => MaxAnisotropy,
        max_vertex_w                                    => MaxVertexW,
        guard_band_left                                 => GuardBandLeft,
        guard_band_top                                  => GuardBandTop,
        guard_band_right                                => GuardBandRight,
        guard_band_bottom                               => GuardBandBottom,
        extents_adjust                                  => ExtentsAdjust,
        stencil_caps                                    => StencilCaps,
        fvf_caps                                        => FVFCaps,
        texture_op_caps                                 => TextureOpCaps,
        max_texture_blend_stages                        => MaxTextureBlendStages,
        max_simultaneous_textures                       => MaxSimultaneousTextures,
        vertex_processing_caps                          => VertexProcessingCaps,
        max_active_lights                               => MaxActiveLights,
        max_user_clip_planes                            => MaxUserClipPlanes,
        max_vertex_blend_matrices                       => MaxVertexBlendMatrices,
        max_vertex_blend_matrix_index                   => MaxVertexBlendMatrixIndex,
        max_point_size                                  => MaxPointSize,
        max_primitive_count                             => MaxPrimitiveCount,
        max_vertex_index                                => MaxVertexIndex,
        max_streams                                     => MaxStreams,
        max_stream_stride                               => MaxStreamStride,
        vertex_shader_version                           => VertexShaderVersion,
        max_vertex_shader_const                         => MaxVertexShaderConst,
        pixel_shader_version                            => PixelShaderVersion,
        pixel_shader_1x_max_value                       => PixelShader1xMaxValue,
        dev_caps2                                       => DevCaps2,
        max_npatch_tessellation_level                   => MaxNpatchTessellationLevel,
        reserved5                                       => Reserved5,
        master_adapter_ordinal                          => MasterAdapterOrdinal,
        adapter_ordinal_in_group                        => AdapterOrdinalInGroup,
        number_of_adapters_in_group                     => NumberOfAdaptersInGroup,
        decl_types                                      => DeclTypes,
        num_simultaneous_rts                            => NumSimultaneousRTs,
        stretch_rect_filter_caps                        => StretchRectFilterCaps,
        vs_20_caps                                      => VS20Caps,
        ps_20_caps                                      => PS20Caps,
        vertex_texture_filter_caps                      => VertexTextureFilterCaps,
        max_vshader_instructions_executed               => MaxVShaderInstructionsExecuted,
        max_pshader_instructions_executed               => MaxPShaderInstructionsExecuted,
        max_vertex_shader_30_instruction_slots          => MaxVertexShader30InstructionSlots,
        max_pixel_shader_30_instruction_slots           => MaxPixelShader30InstructionSlots,
    }

    VShaderCaps20 => D3DVSHADERCAPS2_0 {
        caps                                            => Caps,
        dynamic_flow_control_depth                      => DynamicFlowControlDepth,
        num_temps                                       => NumTemps,
        static_flow_control_depth                       => StaticFlowControlDepth,
    }

    PShaderCaps20 => D3DPSHADERCAPS2_0 {
        caps                                            => Caps,
        dynamic_flow_control_depth                      => DynamicFlowControlDepth,
        num_temps                                       => NumTemps,
        static_flow_control_depth                       => StaticFlowControlDepth,
        num_instruction_slots                           => NumInstructionSlots,
    }
}


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Reserved5(DWORD);

//#cpp2rust D3DCAPS9                    = d3d9::Caps
//#cpp2rus_ D3DCONTENTPROTECTIONCAPS    = d3d::
//#cpp2rus_ D3DOVERLAYCAPS              = d3d::
//#cpp2rust D3DPSHADERCAPS2_0           = d3d::PShaderCaps20
//#cpp2rust D3DVSHADERCAPS2_0           = d3d::VShaderCaps20
