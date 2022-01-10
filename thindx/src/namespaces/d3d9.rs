#![allow(rustdoc::broken_intra_doc_links)] // temporary

//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9-graphics)\]
//! Direct3D 9 related types and APIs
//!
//! Thin Direct3D9 wrappers for Rust, designed to be suitable for:
//! *   Greenfield projects targeting ancient APIs for some reason
//! *   Graphics middleware looking to integrate into existing D3D9 codebases
//!
//! ### Interfaces
//!
//! These are all refcounting COM smart pointers convertable to/from [mcom::Rc].
//!
#![doc = include_str!("d3d9-interface-tree.md")] // Breaks horribly in rust-analyzer's intellisense
//!
//! | Rust COM Pointer      | C+++ \[Rust Ext Trait\]                                                   | Description   |
//! | --------------------- | ------------------------------------------------------------------------- | ------------- |
//! | [Unknown]             | [IUnknown]\[[Ext](IUnknownExt)\]                                          | The root type from which all sane COM types derive
//! | [Direct3D]            | [IDirect3D9]\[[Ext](IDirect3D9Ext)\]                                      | Core factory for creating [Device]s
//! | [Direct3DEx]          | [IDirect3D9Ex]\[[Ext](IDirect3D9ExExt)\]                                  | Core factory for creating [DeviceEx]s
//! | [Device]              | [IDirect3DDevice9]\[[Ext](IDirect3DDevice9Ext)\]                          | Create resources & dispatches rendering for an individual GPU
//! | [DeviceEx]            | [IDirect3DDevice9Ex]\[[Ext](IDirect3DDevice9ExExt)\]                      | Create resources & dispatches rendering for an individual GPU
//! | [SwapChain]           | [IDirect3DSwapChain9]\[[Ext](IDirect3DSwapChain9Ext)\]                    | Manages swapping buffers for individual monitors/windows
//! | [SwapChainEx]         | [IDirect3DSwapChain9Ex]\[[Ext](IDirect3DSwapChain9ExExt)\]                | Manages swapping buffers for individual monitors/windows
//! | [Resource]            | [IDirect3DResource9]\[[Ext](IDirect3DResource9Ext)\]                      | [Surface] / [*Texture](BaseTexture) / [IndexBuffer] / [VertexBuffer]
//! | [Surface]             | [IDirect3DSurface9]\[[Ext](IDirect3DSurface9Ext)\]                        | 2D buffer of pixels
//! | [BaseTexture]         | [IDirect3DBaseTexture9]\[[Ext](IDirect3DBaseTexture9Ext)\]                | A GPU-friendly collection of pixels
//! | [CubeTexture]         | [IDirect3DCubeTexture9]\[[Ext](IDirect3DCubeTexture9Ext)\]                | 6-sided 2D texture
//! | [Texture]             | [IDirect3DTexture9]\[[Ext](IDirect3DTexture9Ext)\]                        | 2D texture
//! | [VolumeTexture]       | [IDirect3DVolumeTexture9]\[[Ext](IDirect3DVolumeTexture9Ext)\]            | Dense 3D texture
//! | [IndexBuffer]         | [IDirect3DIndexBuffer9]\[[Ext](IDirect3DIndexBuffer9Ext)\]                | An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) indexes verticies in a [VertexBuffer] when rendering.
//! | [VertexBuffer]        | [IDirect3DVertexBuffer9]\[[Ext](IDirect3DVertexBuffer9Ext)\]              | A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) typically contains points of a mesh to be rendered.
//! | [Volume]              | [IDirect3DVolume9]\[[Ext](IDirect3DVolume9Ext)\]                          | 3D buffer of pixels
//! | [PixelShader]         | [IDirect3DPixelShader9]\[[Ext](IDirect3DPixelShader9Ext)\]                | per-fragment GPU program
//! | [VertexShader]        | [IDirect3DVertexShader9]\[[Ext](IDirect3DVertexShader9Ext)\]              | per-vertex GPU program
//! | [Query]               | [IDirect3DQuery9]\[[Ext](IDirect3DQuery9Ext)\]                            | An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
//! | [StateBlock]          | [IDirect3DStateBlock9]\[[Ext](IDirect3DStateBlock9Ext)\]                  | Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state) changes to [Device] state.
//! | [VertexDeclaration]   | [IDirect3DVertexDeclaration9]\[[Ext](IDirect3DVertexDeclaration9Ext)\]    | Describes the layout of the contents of a [VertexBuffer]
//!
//! ### Enumerations
//!
//! ⚠️ **NOTE:** D3D `enum`s are represented as Rust `struct`s to avoid undefined behavior related to unlisted enumerants.
//!
//! ⚠️ **NOTE:** [DeclMethod8], [DeclType8], and [DeclUsage8] are all 8-bit, despite `enum D3DDECL*` being 32-bit.<br>
//! &nbsp; &nbsp; &nbsp; &nbsp; ❌ This makes them unsuitable for raw function FFI, due to ABI mismatches.<br>
//! &nbsp; &nbsp; &nbsp; &nbsp; ✔️ This makes them suitable for [VertexElement] FFI, as [D3DVERTEXELEMENT9]::{Method,Type,and Usage} are all `BYTE`s.<br>
//!
//! | Rust                                      | C++                       | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [D3D](crate::D3D)\[[ERR](crate::D3DERR)\]         <br> [D3D](crate::D3D)\[[ERR](crate::D3DERR)\]::\*  | [D3DERR]                      <br> [D3DERR]_\*                | Windows HRESULTs optimized for displaying Direct3D errors
//! | [BackBufferType]                                  <br> [BackBufferType]::\*                           | [D3DBACKBUFFER_TYPE]          <br> [D3DBACKBUFFER_TYPE]_\*    | [Mono](crate::BackBufferType::Mono), [Left](crate::BackBufferType::Left), or [Right](crate::BackBufferType::Right) for [stereo](https://en.wikipedia.org/wiki/Stereoscopy) rendering
//! | [BasisType]                                       <br> [Basis]::\*                                    | [D3DBASISTYPE]                <br> [D3DBASISTYPE]_\*          | Tesselation basis
//! | [Blend]                                           <br> [Blend]::\*                                    | [D3DBLEND]                    <br> [D3DBLEND]_\*              | Target/output alpha/color blending factors
//! | [BlendOp]                                         <br> [BlendOp]::\*                                  | [D3DBLENDOP]                  <br> [D3DBLENDOP]_\*            | Target/output alpha/color blending operation
//! | [CmpFunc]                                         <br> [Cmp]::\*                                      | [D3DCMPFUNC]                  <br> [D3DCMP]_\*                | ZBuffer/depth comparison operation
//! | [ComposeRectsOp]                                  <br> [ComposeRects]::\*                             | [D3DCOMPOSERECTSOP]           <br> [D3DCOMPOSERECTS]_\*       |
//! | ~~CubeMapFaces~~                                  <br> [CubeMapFace]::\*                              | [D3DCUBEMAP_FACES]            <br> [D3DCUBEMAP_FACE]_\*       | Which face of a cubemap to lock/update/acquire/??? <br> This isn't a mask, [CubeMapFace] reads way better in all contexts!
//! | [Cull]                                            <br> [Cull]::\*                                     | [D3DCULL]                     <br> [D3DCULL]_\*               | [None](crate::Cull::None), [CW](crate::Cull::CW), [CCW](crate::Cull::CCW)
//! | [DebugMonitorTokens]                              <br> [DMT]::\*                                      | [D3DDEBUGMONITORTOKENS]       <br> [D3DDMT]_\*                | Debug monitor tokens.
//! | [DeclMethod8]                                     <br> [DeclMethod8]::\*                              | [D3DDECLMETHOD]               <br> [D3DDECLMETHOD]_\*         | Tesselation method.
//! | [DeclType8]                                       <br> [DeclType8]::\*                                | [D3DDECLTYPE]                 <br> [D3DDECLTYPE]_\*           | [Float1](crate::DeclType8::Float1), [Float2](crate::DeclType8::Float2), ... - Defines a vertex declaration data type.
//! | [DeclUsage8]                                      <br> [DeclUsage8]::\*                               | [D3DDECLUSAGE]                <br> [D3DDECLUSAGE]_\*          | [Position](crate::DeclUsage8::Position), [TexCoord](crate::DeclUsage8::TexCoord), ... - Defines the intended use of vertex data.
//! | [DegreeType]                                      <br> [Degree]::\*                                   | [D3DDEGREETYPE]               <br> [D3DDEGREE]_\*             | Curve [polynomial degree](https://en.wikipedia.org/wiki/Degree_of_a_polynomial)
//! | [DevType]                                         <br> [DevType]::\*                                  | [D3DDEVTYPE]                  <br> [D3DDEVTYPE]_\*            | Specifies what kind of [Device] should be created
//! | [DisplayRotation]                                 <br> [DisplayRotation]::\*                          | [D3DDISPLAYROTATION]          <br> [D3DDISPLAYROTATION]_\*    | Orientation of the monitor/display
//! | [FillMode]                                        <br> [Fill]::\*                                     | [D3DFILLMODE]                 <br> [D3DFILL]_\*               | [Point](crate::Fill::Point), [Wireframe](crate::Fill::Wireframe), or [Solid](crate::Fill::Solid) polygon rendering
//! | [FogMode]                                         <br> [Fog]::\*                                      | [D3DFOGMODE]                  <br> [D3DFOG]_\*                | [None](crate::Fog::None), [Exp](crate::Fog::Exp), [Exp2](crate::Fog::Exp2), or [Linear](crate::Fog::Linear) fog falloff
//! | [Format]                                          <br> [Fmt]::\*                                      | [D3DFORMAT]                   <br> [D3DFMT]_\*                | Texture and vertex element formats
//! | [LightType]                                       <br> [LightType]::\*                                | [D3DLIGHTTYPE]                <br> [D3DLIGHTTYPE]_\*          | Defines the type of a light ([Point](crate::LightType::Point), [Spot](crate::LightType::Spot), or [Directional](crate::LightType::Directional))
//! | [MaterialColorSource]                             <br> [MCS]::\*                                      | [D3DMATERIALCOLORSOURCE]      <br> [D3DMCS]_\*                | Lighting material source
//! | [MultiSampleType]                                 <br> [MultiSample]::\*                              | [D3DMULTISAMPLE_TYPE]         <br> [D3DMULTISAMPLE]_\*        | Defines the levels of full-scene multisampling to apply
//! | [PatchEdgeStyle]                                  <br> [PatchEdge]::\*                                | [D3DPATCHEDGESTYLE]           <br> [D3DPATCHEDGE]_\*          | [Discrete](crate::PatchEdge::Discrete) or [Continuous](crate::PatchEdge::Continuous) tesselation.
//! | [Pool]                                            <br> [Pool]::\*                                     | [D3DPOOL]                     <br> [D3DPOOL]_\*               | Specifies what memory pool [Resource]s should be stored in
//! | [PrimitiveType]                                   <br> [PT]::\*                                       | [D3DPRIMITIVETYPE]            <br> [D3DPT]_\*                 | Defines the primitives supported by Direct3D.
//! | [QueryType]                                       <br> [QueryType]::\*                                | [D3DQUERYTYPE]                <br> [D3DQUERYTYPE]_\*          | Identifies the query type.
//! | [RenderStateType]                                 <br> [RS]::\*                                       | [D3DRENDERSTATETYPE]          <br> [D3DRS]_\*                 |
//! | [ResourceType]                                    <br> [RType]::\*                                    | [D3DRESOURCETYPE]             <br> [D3DRTYPE]_\*              | Specifies the type of a [Resource]/[Volume]
//! | [SamplerStateType]                                <br> [Samp]::\*                                     | [D3DSAMPLERSTATETYPE]         <br> [D3DSAMP]_\*               |
//! | [SamplerTextureType]                              <br> [STT]::\*                                      | [D3DSAMPLER_TEXTURE_TYPE]     <br> [D3DSTT]_\*                |
//! | [ScanlineOrdering]                                <br> [ScanlineOrdering]::\*                         | [D3DSCANLINEORDERING]         <br> [D3DSCANLINEORDERING]_\*   |
//! | [SGR]                                             <br> [SGR]::\*                                      | [D3DSGR]                      <br> [D3DSGR]_\*                | Indicates whether gamma correction should be applied.
//! | [ShadeMode]                                       <br> [Shade]::\*                                    | [D3DSHADEMODE]                <br> [D3DSHADE]_\*              |
//! | [StateBlockType]                                  <br> [SBT]::\*                                      | [D3DSTATEBLOCKTYPE]           <br> [D3DSBT]_\*                | Predefined sets of pipeline state used by state blocks
//! | [StencilOp]                                       <br> [StencilOp]::\*                                | [D3DSTENCILOP]                <br> [D3DSTENCILOP]_\*          |
//! | [StreamSource]                                    <br> [StreamSource]::\*                             | [D3DSTREAMSOURCE]             <br> [D3DSTREAMSOURCE]_\*       |
//! | [SwapEffect]                                      <br> [SwapEffect]::*                                | [D3DSWAPEFFECT]               <br> [D3DSWAPEFFECT]_\*         | Defines [IDirect3DDevice9Ext::present] swap effects.
//! | [TextureAddress]                                  <br> [TAddress]::\*                                 | [D3DTEXTUREADDRESS]           <br> [D3DTADDRESS]_\*           |
//! | [TextureFilterType]                               <br> [TexF]::\*                                     | [D3DTEXTUREFILTERTYPE]        <br> [D3DTEXF]_\*               |
//! | [TextureOp]                                       <br> [TOP]::\*                                      | [D3DTEXTUREOP]                <br> [D3DTOP]_\*                |
//! | [TextureStageStateType]                           <br> [TSS]::\*                                      | [D3DTEXTURESTAGESTATETYPE]    <br> [D3DTSS]_\*                |
//! | [TransformStateType]                              <br> [TS]::\*                                       | [D3DTRANSFORMSTATETYPE]       <br> [D3DTS]_\*                 |
//! | [ZBufferType]                                     <br> [ZB]::\*                                       | [D3DZBUFFERTYPE]              <br> [D3DZB]_\*                 |
//!
//! ### Flags
//!
//! | Rust                                      | C++                       | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [Create]                                  | [D3DCREATE_*]             | A combination of one or more flags that controls [Direct3D::create_device] behavior.
//! | [FVF]                                     | [D3DFVF_*]                | Describes the contents of vertices interleaved in a single data stream.
//! | [GetData]                                 | [D3DGETDATA_*]            | Controls how [IDirect3DQuery9Ext::get_data_inplace] behaves
//! | [Issue]                                   | [D3DISSUE_*]              | Controls how [IDirect3DQuery9Ext::issue] behaves
//! | [Lock]                                    | [D3DLOCK_*]               | A combination of zero or more locking options that describe the type of lock to perform.
//! | [Usage]                                   | [D3DUSAGE_*]              | Usage options that identify how resources are to be used.
//!
//! ### Traits
//!
//! | Rust                                      | C++                       | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | unsafe [Raw]                              |                           | Conversion trait for converting between [thindx](crate) ⇄ [winapi]
//! | unsafe [SharedHandleParam]                | \*mut HANDLE              | Placeholder for [Sharing Resources](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9lh#sharing-resources) `*mut HANDLE`s
//!
//! ### Structures
//!
//! | Rust                                      | C++                       | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [AdapterIdentifier]                       | [D3DADAPTER_IDENTIFIER9]  | Adapter metadata (driver, Description, driver version, vendor/device ids, ...)
//! | [Caps]                                    | [D3DCAPS9]                | Adapter/device capabilities and limitations
//! | [ClipStatus]                              | [D3DCLIPSTATUS9]          | Describes the current clip status.
//! | [ColorValue]                              | [D3DCOLORVALUE]           | A <code>{ <span style="color: red">r</span>, <span style="color: green">g</span>, <span style="color: blue">b</span>, a }</code> floating-point color
//! | [DisplayMode]                             | [D3DDISPLAYMODE]          | A `{ width, height, refresh_rate, format }` display mode
//! | [DisplayModeEx]                           | [D3DDISPLAYMODEEX]        | A `{ width, height, refresh_rate, format, scanline_ordering }` display mode
//! | [IndexBufferDesc]                         | [D3DINDEXBUFFER_DESC]     | Describes an [IndexBuffer]
//! | [Light]                                   | [D3DLIGHT9]               | Describes lighting information
//! | [Material]                                | [D3DMATERIAL9]            | Describes a material that responds to lighting
//! | [PresentStats]                            | [D3DPRESENTSTATS]         | Describes swapchain statistics relating to PresentEx calls.
//! | [RasterStatus]                            | [D3DRASTER_STATUS]        | `{ in_vblank, scan_line }`
//! | [Rect]                                    | [D3DRECT] / [RECT]        | `[0i32 .. 1i32, 2i32 .. 3i32]` signed x/y range structure
//! | [Vector]                                  | [D3DVECTOR]               | A `{ x, y, z }` 3-dimensional floating point position
//! | [VertexBufferDesc]                        | [D3DVERTEXBUFFER_DESC]    | Describes an [VertexBuffer]
//! | [VertexElement]                           | [D3DVERTEXELEMENT9]       | An element of a [VertexDeclaration]
//! | [Viewport]                                | [D3DVIEWPORT9]            | A `{ x, y, width, height, min_z, max_z }` area to render into
//!
//! ### Values
//!
//! | Rust                                      | C++                       | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [bool32]                                  | [BOOL]                    | 32-bit boolean type that's ABI-compatible with Win32's [BOOL]
//! | [Color]                                   | [D3DCOLOR]                | 0xAA<span style="color: red">RR</span><span style="color: green">GG</span><span style="color: blue">BB</span> style 32-bit color
//! | [Luid]                                    | [LUID]                    | A 64-bit locally unique identifier
//! | [SdkVersion]                              | DWORD                     | Specify what Direct3D SDK to use ([Direct3D](crate::Direct3D)\[[Ex](crate::Direct3DEx)\]::[create](crate::Direct3D::create)'s only parameter)
//!
//! ### Wrappers
//!
//! | Wrapper type                              | Underlying type           | Description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [SafeDevice]                              | [Device]                  | Device pointer + enough metadata to make more API calls safe
//!
//! ### Features
//!
//! | feature               | Description           |
//! | --------------------- | --------------------- |
// |                       | Default: **enabled** by default|
//! | `9ex`                 | `!defined(D3D_DISABLE_9EX)` - Enables [Direct3DEx], [DeviceEx], [SwapChainEx], etc.
//!
#![doc = include_str!("../headers/_misc/cpp2url.md")]
#![doc = include_str!("../headers/d3d9.h/cpp2url.md")]
#![doc = include_str!("../headers/d3d9caps.h/cpp2url.md")]
#![doc = include_str!("../headers/d3d9types.h/cpp2url.md")]

#![allow(unused_imports)]

mod shared_handle;

use crate::Unknown;
use abibool::bool32;
use u32 as DWORD;

pub use crate::d3d9_h::*;
pub use crate::d3d9types_h::*;
pub use shared_handle::SharedHandleParam;
