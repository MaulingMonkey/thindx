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

    pub device_type:                                    DevType,
    pub adapter_ordinal:                                AdapterOrdinal, // UINT

    // Caps from DX7 Draw

    pub caps:                                           Caps1,
    pub caps2:                                          Caps2,
    pub caps3:                                          Caps3,

    /// Bit mask of available presentation swap intervals.
    ///
    /// Intervals documented by caps include: [Present::IntervalImmediate], [Present::IntervalOne], ..., [Present::IntervalFour]
    pub presentation_intervals:                         Present,

    // Cursor Caps

    pub cursor_caps:                                    CursorCaps,

    // 3D Device Caps

    pub dev_caps:                                       DevCaps,
    pub primitive_misc_caps:                            PMiscCaps,
    pub raster_caps:                                    PRasterCaps,
    pub z_cmp_caps:                                     PCmpCaps,
    pub src_blend_caps:                                 PBlendCaps,
    pub dest_blend_caps:                                PBlendCaps,

    /// Alpha-test comparison capabilities. This member can include the same capability flags defined for the ZCmpCaps member.
    /// If this member contains only the [PCmpCaps::Always] or only the [PCmpCaps::Never] capability, the driver does not support alpha tests.
    /// Otherwise, the flags identify the individual comparisons that are supported for alpha testing.
    pub alpha_cmp_caps:                                 PCmpCaps,
    pub shade_caps:                                     PShadeCaps,
    pub texture_caps:                                   PTextureCaps,
    pub texture_filter_caps:                            PTFilterCaps,
    pub cube_texture_filter_caps:                       PTFilterCaps,
    pub volume_texture_filter_caps:                     PTFilterCaps,
    pub texture_address_caps:                           PTAddressCaps,
    pub volume_texture_address_caps:                    PTAddressCaps,
    pub line_caps:                                      LineCaps,
    pub max_texture_width:                              dword,
    pub max_texture_height:                             dword,
    pub max_volume_extent:                              dword,
    pub max_texture_repeat:                             dword,
    pub max_texture_aspect_ratio:                       dword,
    pub max_anisotropy:                                 dword,
    pub max_vertex_w:                                   float,
    pub guard_band_left:                                float,
    pub guard_band_top:                                 float,
    pub guard_band_right:                               float,
    pub guard_band_bottom:                              float,
    pub extents_adjust:                                 float,
    pub stencil_caps:                                   StencilCaps,
    pub fvf_caps:                                       FvfCaps,
    pub texture_op_caps:                                TexOpCaps,
    pub max_texture_blend_stages:                       dword,
    pub max_simultaneous_textures:                      dword,
    pub vertex_processing_caps:                         VtxPCaps,
    pub max_active_lights:                              dword,
    pub max_user_clip_planes:                           dword,
    pub max_vertex_blend_matrices:                      dword,
    pub max_vertex_blend_matrix_index:                  dword,
    pub max_point_size:                                 float,
    pub max_primitive_count:                            dword,
    pub max_vertex_index:                               dword,
    pub max_streams:                                    dword,
    pub max_stream_stride:                              dword,
    pub vertex_shader_version:                          ShaderVersion,
    pub max_vertex_shader_const:                        dword,
    pub pixel_shader_version:                           ShaderVersion,
    pub pixel_shader_1x_max_value:                      float,

    // DX9 specific

    pub dev_caps2:                                      DevCaps2,
    pub max_npatch_tessellation_level:                  float,
    #[doc(hidden)]
    pub reserved5:                                      Reserved5,
    pub master_adapter_ordinal:                         AdapterOrdinal,
    pub adapter_ordinal_in_group:                       uint,
    pub number_of_adapters_in_group:                    uint,
    pub decl_types:                                     DtCaps,
    pub num_simultaneous_rts:                           dword,
    pub stretch_rect_filter_caps:                       PTFilterCaps, // just some subset
    pub vs_20_caps:                                     VShaderCaps20,
    pub ps_20_caps:                                     PShaderCaps20,
    pub vertex_texture_filter_caps:                     PTFilterCaps,
    pub max_vshader_instructions_executed:              dword,
    pub max_pshader_instructions_executed:              dword,
    pub max_vertex_shader_30_instruction_slots:         dword,
    pub max_pixel_shader_30_instruction_slots:          dword,
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dpshadercaps2_0)\]
/// D3DPSHADERCAPS2_0
#[derive(Clone, Copy, Default, Debug, Pod, Zeroable)]
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
#[derive(Clone, Copy, Default, Debug, Pod, Zeroable)]
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
