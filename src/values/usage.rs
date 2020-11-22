#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DUSAGE = u32; // there's no actual type

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage)\]
/// DWORD / D3DUSAGE_*
///
/// Usage options that identify how resources are to be used.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Usage(D3DUSAGE);
// TODO: usage table from https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage#usage-and-resource-combinations?

impl Usage {
    /// Convert a raw [D3DUSAGE] value into a [Usage].  This is *probably* safe... probably...
    ///
    /// [D3DUSAGE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage
    pub const fn from_unchecked(usage: D3DUSAGE) -> Self { Self(usage) }

    /// Convert a [Usage] into a raw [D3DUSAGE].
    ///
    /// [D3DUSAGE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage
    pub const fn into(self) -> D3DUSAGE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Usage {
    /// The resource will automatically generate mipmaps.
    /// See [Automatic Generation of Mipmaps (Direct3D 9)].
    /// Automatic generation of mipmaps is not supported for volume textures and depth stencil surfaces/textures.
    /// This usage is not valid for a resource in system memory ([Pool::SystemMem]).
    ///
    /// [Automatic Generation of Mipmaps (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/automatic-generation-of-mipmaps
    pub const AutoGenMipMap                 : Usage = Usage(D3DUSAGE_AUTOGENMIPMAP);

    /// The resource will be a depth stencil buffer. [Usage::DepthStencil] can only be used with [Pool::Default].
    pub const DepthStencil                  : Usage = Usage(D3DUSAGE_DEPTHSTENCIL);

    /// The resource will be a displacement map.
    pub const DMap                          : Usage = Usage(D3DUSAGE_DMAP);

    /// Set to indicate that the vertex buffer content will never require clipping. When rendering with buffers that have this flag set, the D3DRS_CLIPPING render state must be set to false.
    pub const DoNotClip                     : Usage = Usage(D3DUSAGE_DONOTCLIP);

    /// Set to indicate that the vertex buffer requires dynamic memory use.
    /// This is useful for drivers because it enables them to decide where to place the buffer.
    /// In general, static vertex buffers are placed in video memory and dynamic vertex buffers are placed in AGP memory.
    /// Note that there is no separate static use.
    /// If you do not specify [Usage::Dynamic], the vertex buffer is made static.
    /// [Usage::Dynamic] is strictly enforced through the [Lock::Discard] and [Lock::NoOverwrite] locking flags.
    /// As a result, [Lock::Discard] and [Lock::NoOverwrite] are valid only on vertex buffers created with [Usage::Dynamic].
    /// They are not valid flags on static vertex buffers. For more information, see [Managing Resources (Direct3D 9)].
    ///
    /// For more information about using dynamic vertex buffers, see [Performance Optimizations (Direct3D 9)].
    ///
    /// [Usage::Dynamic] and [Pool::Managed] are incompatible and should not be used together. See [Pool].
    ///
    /// Textures can specify [Usage::Dynamic].
    /// However, managed textures cannot use [Usage::Dynamic].
    /// For more information about dynamic textures, see [Using Dynamic Textures].
    ///
    /// [Managing Resources (Direct3D 9)]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/managing-resources
    /// [Performance Optimizations (Direct3D 9)]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/performance-optimizations
    /// [Using Dynamic Textures]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/performance-optimizations
    pub const Dynamic                       : Usage = Usage(D3DUSAGE_DYNAMIC);

    /// Allow a shared surface created by a secure application to be opened by a non-secure application that has the shared handle.
    /// Differences between **Direct3D 9** and **Direct3D 9Ex**: This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const NonSecure                     : Usage = Usage(D3DUSAGE_NONSECURE);

    /// Set to indicate that the vertex buffer is to be used for drawing N-patches.
    pub const NPatches                      : Usage = Usage(D3DUSAGE_NPATCHES);

    /// Set to indicate that the vertex or index buffer will be used for drawing point sprites.
    /// The buffer will be loaded in system memory if software vertex processing is needed to emulate point sprites.
    pub const Points                        : Usage = Usage(D3DUSAGE_POINTS);

    /// The resource will be a render target. [Usage::RenderTarget] can only be used with [Pool::Default].
    pub const RenderTarget                  : Usage = Usage(D3DUSAGE_RENDERTARGET);

    /// Set to indicate that the vertex buffer is to be used for drawing high-order primitives.
    pub const RTPatches                     : Usage = Usage(D3DUSAGE_RTPATCHES);

    /// If this flag is used, vertex processing is done in software.
    /// If this flag is not used, vertex processing is done in hardware.
    ///
    /// The [Usage::SoftwareProcessing] flag can be set when mixed-mode or software vertex processing (D3DCREATE_MIXED_VERTEXPROCESSING / D3DCREATE_SOFTWARE_VERTEXPROCESSING) is enabled for that device.
    /// [Usage::SoftwareProcessing] must be set for buffers to be used with software vertex processing in mixed mode, but it should not be set for the best possible performance when using hardware index processing in mixed mode (D3DCREATE_HARDWARE_VERTEXPROCESSING).
    /// However, setting [Usage::SoftwareProcessing] is the only option when a single buffer is used with both hardware and software vertex processing.
    /// [Usage::SoftwareProcessing] is allowed for mixed and software devices.
    ///
    /// [Usage::SoftwareProcessing] is used with CheckDeviceFormat to find out if a particular texture format can be used as a vertex texture during software vertex processing.
    /// If it can, the texture must be created in [Pool::Scratch].
    pub const SoftwareProcessing            : Usage = Usage(D3DUSAGE_SOFTWAREPROCESSING);

    /// This usage flag must be specified for vertex buffers and source surfaces, used in calls to [DeviceEx::compose_rects].
    /// Textures created with this usage flag cannot be used for texture filtering.
    /// Vertex buffers, created with this usage flag, cannot be used as input stream sources.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const TextAPI                       : Usage = Usage(D3DUSAGE_TEXTAPI);

    /// Informs the system that the application writes only to the vertex buffer.
    /// Using this flag enables the driver to choose the best memory location for efficient write operations and rendering.
    /// Attempts to read from a vertex buffer that is created with this capability will fail.
    /// Buffers created with [Pool::Default] that do not specify [Usage::WriteOnly] may suffer a severe performance penalty.
    /// [Usage::WriteOnly] only affects the performance of [Pool::Default] buffers.
    pub const WriteOnly                     : Usage = Usage(D3DUSAGE_WRITEONLY);

    /// Setting this flag indicates that the resource might contain protected content.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const RestrictedContent             : Usage = Usage(D3DUSAGE_RESTRICTED_CONTENT);

    /// Setting this flag indicates that access to the shared resource should be restricted.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const RestrictSharedResource        : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE);

    /// Setting this flag indicates that the driver should restrict access to the shared resource.
    /// The caller must create an authenticated channel with the driver.
    /// The driver should then allow access to processes that attempt to open that shared resource.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const RestrictSharedResourceDriver  : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER);
}

impl BitOrAssign for Usage {
    fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
}

impl BitOr for Usage {
    type Output = Self;
    fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl Debug for Usage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Usage::AutoGenMipMap                => write!(f, "Usage::AutoGenMipMap"),
            Usage::DepthStencil                 => write!(f, "Usage::DepthStencil"),
            Usage::DMap                         => write!(f, "Usage::DMap"),
            Usage::DoNotClip                    => write!(f, "Usage::DoNotClip"),
            Usage::Dynamic                      => write!(f, "Usage::Dynamic"),
            Usage::NonSecure                    => write!(f, "Usage::NonSecure"),
            Usage::NPatches                     => write!(f, "Usage::NPatches"),
            Usage::Points                       => write!(f, "Usage::Points"),
            Usage::RenderTarget                 => write!(f, "Usage::RenderTarget"),
            Usage::RTPatches                    => write!(f, "Usage::RTPatches"),
            Usage::SoftwareProcessing           => write!(f, "Usage::SoftwareProcessing"),
            Usage::TextAPI                      => write!(f, "Usage::TextAPI"),
            Usage::WriteOnly                    => write!(f, "Usage::WriteOnly"),
            Usage::RestrictedContent            => write!(f, "Usage::RestrictedContent"),
            Usage::RestrictSharedResource       => write!(f, "Usage::RestrictSharedResource"),
            Usage::RestrictSharedResourceDriver => write!(f, "Usage::RestrictSharedResourceDriver"),
            other                               => write!(f, "Usage({})", other.0 as u32),
        }
    }
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Usage {
    fn default() -> Self { Usage(0) }
}

impl From<Usage> for D3DUSAGE {
    fn from(value: Usage) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DUSAGE> for Usage {
    fn from(value: D3DUSAGE) -> Self { Self(value) }
}
