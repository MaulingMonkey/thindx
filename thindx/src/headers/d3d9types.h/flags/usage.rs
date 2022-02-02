#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
type D3DUSAGE = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage)\]
/// DWORD / D3DUSAGE_*
///
/// Usage options that identify how resources are to be used.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Usage(D3DUSAGE);
// TODO: usage table from https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage#usage-and-resource-combinations?

flags! {
    Usage => D3DUSAGE;
    None, AutoGenMipMap, DepthStencil, DMap, DoNotClip, Dynamic, NonSecure, NPatches, Points, RenderTarget, RTPatches,
    SoftwareProcessing, TextAPI, WriteOnly, RestrictedContent, RestrictSharedResource, RestrictSharedResourceDriver,
}

#[allow(non_upper_case_globals)] impl Usage { // These are enum-like
    pub const None                          : Usage = Usage(0);

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
    #[cfg(not(feature = "9ex"))]
    pub(crate) const NonSecure              : Usage = Usage(D3DUSAGE_NONSECURE);

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
    /// [Usage::SoftwareProcessing] is used with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) to find out if a particular texture format can be used as a vertex texture during software vertex processing.
    /// If it can, the texture must be created in [Pool::Scratch].
    pub const SoftwareProcessing            : Usage = Usage(D3DUSAGE_SOFTWAREPROCESSING);

    /// This usage flag must be specified for vertex buffers and source surfaces, used in calls to [IDirect3DDevice9ExExt::compose_rects].
    /// Textures created with this usage flag cannot be used for texture filtering.
    /// Vertex buffers, created with this usage flag, cannot be used as input stream sources.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const TextAPI                       : Usage = Usage(D3DUSAGE_TEXTAPI);
    #[cfg(not(feature = "9ex"))]
    pub(crate) const TextAPI                : Usage = Usage(D3DUSAGE_TEXTAPI);

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
    #[cfg(not(feature = "9ex"))]
    pub(crate) const RestrictedContent      : Usage = Usage(D3DUSAGE_RESTRICTED_CONTENT);

    /// Setting this flag indicates that access to the shared resource should be restricted.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const RestrictSharedResource        : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE);
    #[cfg(not(feature = "9ex"))]
    pub(crate) const RestrictSharedResource : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE);

    /// Setting this flag indicates that the driver should restrict access to the shared resource.
    /// The caller must create an authenticated channel with the driver.
    /// The driver should then allow access to processes that attempt to open that shared resource.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:** This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const RestrictSharedResourceDriver  : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER);
    #[cfg(not(feature = "9ex"))]
    pub(crate) const RestrictSharedResourceDriver : Usage = Usage(D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER);



    /// Query the resource format to see if it supports texture filter types other than [d3d::TexF::Point] (which is always supported).
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QueryFilter                   : Usage = Usage(D3DUSAGE_QUERY_FILTER);

    /// Query the resource about a legacy bump map.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QueryLegacyBumpMap            : Usage = Usage(D3DUSAGE_QUERY_LEGACYBUMPMAP);

    /// Query the resource to verify support for post pixel shader blending support.
    /// If [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) fails with [d3d::Usage::QueryPostPixelShaderBlending], post pixel blending operations are not supported.
    /// These include alpha test, pixel fog, render-target blending, color write enable, and dithering.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QueryPostPixelShaderBlending  : Usage = Usage(D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING);

    /// Query the resource to verify if a texture supports gamma correction during a read operation.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QuerySRGBRead                 : Usage = Usage(D3DUSAGE_QUERY_SRGBREAD);

    /// Query the resource to verify if a texture supports gamma correction during a write operation.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QuerySRGBWrite                : Usage = Usage(D3DUSAGE_QUERY_SRGBWRITE);

    /// Query the resource to verify support for vertex shader texture sampling.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QueryVertexTexture            : Usage = Usage(D3DUSAGE_QUERY_VERTEXTEXTURE);

    /// Query the resource to verify support for texture wrapping and mip-mapping.
    ///
    /// Valid for use with [check_device_format](d3d9::IDirect3DDevice9Ext::check_device_format) only.
    pub const QueryWrapAndMip               : Usage = Usage(D3DUSAGE_QUERY_WRAPANDMIP);
}

impl Default for Usage {
    fn default() -> Self { Usage::None }
}

//#cpp2rust D3DUSAGE_RENDERTARGET                       = d3d::Usage::RenderTarget
//#cpp2rust D3DUSAGE_DEPTHSTENCIL                       = d3d::Usage::DepthStencil
//#cpp2rust D3DUSAGE_DYNAMIC                            = d3d::Usage::Dynamic
//#cpp2rust D3DUSAGE_NONSECURE                          = d3d::Usage::NonSecure
//#cpp2rust D3DUSAGE_AUTOGENMIPMAP                      = d3d::Usage::AutoGenMipMap
//#cpp2rust D3DUSAGE_DMAP                               = d3d::Usage::DMap
//#cpp2rust D3DUSAGE_QUERY_LEGACYBUMPMAP                = d3d::Usage::QueryLegacyBumpMap
//#cpp2rust D3DUSAGE_QUERY_SRGBREAD                     = d3d::Usage::QuerySRGBRead
//#cpp2rust D3DUSAGE_QUERY_FILTER                       = d3d::Usage::QueryFilter
//#cpp2rust D3DUSAGE_QUERY_SRGBWRITE                    = d3d::Usage::QuerySRGBWrite
//#cpp2rust D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING     = d3d::Usage::QueryPostPixelShaderBlending
//#cpp2rust D3DUSAGE_QUERY_VERTEXTEXTURE                = d3d::Usage::QueryVertexTexture
//#cpp2rust D3DUSAGE_QUERY_WRAPANDMIP                   = d3d::Usage::QueryWrapAndMip
//#cpp2rust D3DUSAGE_WRITEONLY                          = d3d::Usage::WriteOnly
//#cpp2rust D3DUSAGE_SOFTWAREPROCESSING                 = d3d::Usage::SoftwareProcessing
//#cpp2rust D3DUSAGE_DONOTCLIP                          = d3d::Usage::DoNotClip
//#cpp2rust D3DUSAGE_POINTS                             = d3d::Usage::Points
//#cpp2rust D3DUSAGE_RTPATCHES                          = d3d::Usage::RTPatches
//#cpp2rust D3DUSAGE_NPATCHES                           = d3d::Usage::NPatches
//#cpp2rust D3DUSAGE_TEXTAPI                            = d3d::Usage::TextAPI
//#cpp2rust D3DUSAGE_RESTRICTED_CONTENT                 = d3d::Usage::RestrictedContent
//#cpp2rust D3DUSAGE_RESTRICT_SHARED_RESOURCE           = d3d::Usage::RestrictSharedResource
//#cpp2rust D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER    = d3d::Usage::RestrictSharedResourceDriver
