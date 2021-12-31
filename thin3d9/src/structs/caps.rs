use crate::*;

use winapi::shared::d3d9caps::*;

// use std::fmt::{self, Debug, Formatter}; // TODO: a super awesome Debug impl
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DCAPS9
///
/// Contains various information about an adapter/device's capabilities and limitations
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Caps(D3DCAPS9);

impl Caps {
    pub fn device_type(&self) -> DevType { DevType::from_unchecked(self.DeviceType) }
    // TODO: A *lot* more utility methods
}

impl Default for Caps {
    fn default() -> Self {
        Self(D3DCAPS9 {
            // reduce usage of the danger keyword by manually initializing
            DeviceType:                         0,
            AdapterOrdinal:                     0,
            Caps:                               0,
            Caps2:                              0,
            Caps3:                              0,
            PresentationIntervals:              0,
            CursorCaps:                         0,
            DevCaps:                            0,
            PrimitiveMiscCaps:                  0,
            RasterCaps:                         0,
            ZCmpCaps:                           0,
            SrcBlendCaps:                       0,
            DestBlendCaps:                      0,
            AlphaCmpCaps:                       0,
            ShadeCaps:                          0,
            TextureCaps:                        0,
            TextureFilterCaps:                  0,
            CubeTextureFilterCaps:              0,
            VolumeTextureFilterCaps:            0,
            TextureAddressCaps:                 0,
            VolumeTextureAddressCaps:           0,
            LineCaps:                           0,
            MaxTextureWidth:                    0,
            MaxTextureHeight:                   0,
            MaxVolumeExtent:                    0,
            MaxTextureRepeat:                   0,
            MaxTextureAspectRatio:              0,
            MaxAnisotropy:                      0,
            MaxVertexW:                         0.0,
            GuardBandLeft:                      0.0,
            GuardBandTop:                       0.0,
            GuardBandRight:                     0.0,
            GuardBandBottom:                    0.0,
            ExtentsAdjust:                      0.0,
            StencilCaps:                        0,
            FVFCaps:                            0,
            TextureOpCaps:                      0,
            MaxTextureBlendStages:              0,
            MaxSimultaneousTextures:            0,
            VertexProcessingCaps:               0,
            MaxActiveLights:                    0,
            MaxUserClipPlanes:                  0,
            MaxVertexBlendMatrices:             0,
            MaxVertexBlendMatrixIndex:          0,
            MaxPointSize:                       0.0,
            MaxPrimitiveCount:                  0,
            MaxVertexIndex:                     0,
            MaxStreams:                         0,
            MaxStreamStride:                    0,
            VertexShaderVersion:                0,
            MaxVertexShaderConst:               0,
            PixelShaderVersion:                 0,
            PixelShader1xMaxValue:              0.0,
            DevCaps2:                           0,
            MaxNpatchTessellationLevel:         0.0,
            Reserved5:                          0,
            MasterAdapterOrdinal:               0,
            AdapterOrdinalInGroup:              0,
            NumberOfAdaptersInGroup:            0,
            DeclTypes:                          0,
            NumSimultaneousRTs:                 0,
            StretchRectFilterCaps:              0,
            VS20Caps:                           D3DVSHADERCAPS2_0 { Caps: 0, DynamicFlowControlDepth: 0, NumTemps: 0, StaticFlowControlDepth: 0 },
            PS20Caps:                           D3DPSHADERCAPS2_0 { Caps: 0, DynamicFlowControlDepth: 0, NumTemps: 0, StaticFlowControlDepth: 0, NumInstructionSlots: 0 },
            VertexTextureFilterCaps:            0,
            MaxVShaderInstructionsExecuted:     0,
            MaxPShaderInstructionsExecuted:     0,
            MaxVertexShader30InstructionSlots:  0,
            MaxPixelShader30InstructionSlots:   0,
        })
    }
}

impl Deref for Caps {
    type Target = D3DCAPS9;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Caps {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<D3DCAPS9> for Caps {
    fn from(value: D3DCAPS9) -> Self { Self(value) }
}

impl From<Caps> for D3DCAPS9 {
    fn from(value: Caps) -> Self { value.0 }
}
