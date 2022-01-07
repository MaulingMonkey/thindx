//! <!-- style specific to the root page -->
//! <style>
//!     #main > h2      { display: none; }
//!     #main > table   { display: none !important; }
//!     #main   code    { white-space: nowrap !important; }
//!     /*
//!     */
//! </style>
//!
//! Thin [Direct3D9] wrappers for Rust, designed to be suitable for:
//! *   Greenfield projects targeting ancient APIs for some reason
//! *   Graphics middleware looking to integrate into existing D3D9 codebases
//!
//! [Direct3D9]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/
//!
//! ### Interfaces
//!
//! These are all refcounting COM smart pointers convertable to/from [mcom::Rc].
//! <div class="interface-tree">
//!
//! [Unknown] - The root type from which all sane COM types derive
//! &nbsp;  ├─ [Direct3D]\[[Ex](crate::Direct3DEx)\] - Core factories for creating [Device]\[[Ex](crate::Direct3DEx)\]s and [SwapChain]\[[Ex](crate::SwapChainEx)\]s<span style="opacity: 25%">
//! &nbsp;  ├─ [Device]\[[Ex](crate::DeviceEx)\] - Create resources & dispatches rendering for an individual GPU
//! &nbsp;  ├─ [SwapChain]\[[Ex](crate::SwapChainEx)\] - Manages swapping buffers for individual "views" (monitors/windows)</span>
//! &nbsp;  ├─ [Resource]
//! &nbsp;  │      ├─ [Surface] - 2D buffer of pixels
//! &nbsp;  │      ├─ [BaseTexture] - A GPU-friendly collection of pixels
//! &nbsp;  │      │      ├─ [CubeTexture] - 6-sided 2D texture
//! &nbsp;  │      │      ├─ [Texture] - 2D texture
//! &nbsp;  │      │      └─ [VolumeTexture] - Dense 3D texture
//! &nbsp;  │      ├─ [IndexBuffer] - An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) indexes verticies in a [VertexBuffer] when rendering.
//! &nbsp;  │      └─ [VertexBuffer] - A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) typically contains points of a mesh to be rendered.
//! &nbsp;  ├─ <strike>[Resource]</strike> You'd expect these to be resources, but they aren't - they derive from [Unknown].
//! &nbsp;  │      ├─ [Volume] - 3D buffer of pixels
//! &nbsp;  │      ├─ [PixelShader] - per-fragment GPU program
//! &nbsp;  │      └─ [VertexShader] - per-vertex GPU program
//! &nbsp;  ├─ [Query] - An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
//! &nbsp;  ├─ [StateBlock] - Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state) changes to [Device] state.
//! &nbsp;  └─ [VertexDeclaration] - Describes the layout of the contents of a [VertexBuffer]</div>
//!
//! | thin3d9 struct        | docs.microsoft.com \[Ext\]                                                | description   |
//! | --------------------- | ------------------------------------------------------------------------- | ------------- |
//! | [Unknown]             | [IUnknown]\[[Ext](IUnknownExt)\]                                          | The root type from which all sane COM types derive
//! | [Direct3D]            | [IDirect3D9]\[[Ext](IDirect3D9Ext)\]                                      | Core factory for creating [Device]s
//! | [Direct3DEx]          | [IDirect3D9Ex]\[[Ext](IDirect3D9ExExt)\]                                  | Core factory for creating [DeviceEx]s
//! | [Device]              | [IDirect3DDevice9]\[[Ext](IDirect3DDevice9Ext)\]                          | Create resources & dispatches rendering for an individual GPU
//! | [DeviceEx]            | [IDirect3DDevice9Ex]\[[Ext](IDirect3DDevice9ExExt)\]                      | Create resources & dispatches rendering for an individual GPU
//! | [SwapChain]           | [IDirect3DSwapChain9]\[[Ext](IDirect3DSwapChain9Ext)\]                    | Manages swapping buffers for individual "views" (monitors/windows)
//! | [SwapChainEx]         | [IDirect3DSwapChain9Ex]\[[Ext](IDirect3DSwapChain9ExExt)\]                | Manages swapping buffers for individual "views" (monitors/windows)
//! | [Resource]            | [IDirect3DResource9]\[[Ext](IDirect3DResource9Ext)\]                      |
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
//! ### Enums
//!
//! ⚠️ **NOTE:** D3D `enum`s are represented as Rust `struct`s to avoid undefined behavior related to unlisted enumerants.
//!
//! ⚠️ **NOTE:** [DeclMethod8], [DeclType8], and [DeclUsage8] are all 8-bit, despite `enum D3DDECL*` being 32-bit.<br>
//! &nbsp; &nbsp; &nbsp; &nbsp; ❌ This makes them unsuitable for raw function FFI, due to ABI mismatches.<br>
//! &nbsp; &nbsp; &nbsp; &nbsp; ✔️ This makes them suitable for [VertexElement] FFI, as [D3DVERTEXELEMENT9]::{Method,Type,and Usage} are all `BYTE`s.<br>
//!
//! | `thin3d9` value                           | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [D3D](crate::D3D)\[[ERR](crate::D3DERR)\]::\* | [D3DERR]_\*           | Windows HRESULTs optimized for displaying Direct3D errors
//! | [BackBufferType]::\*                      | [D3DBACKBUFFER_TYPE]_\*   | [Mono](crate::BackBufferType::Mono), [Left](crate::BackBufferType::Left), or [Right](crate::BackBufferType::Right) for [stereo](https://en.wikipedia.org/wiki/Stereoscopy) rendering
//! | [Basis]::\*                               | [D3DBASISTYPE]_\*         | Tesselation basis
//! | [Blend]::\*                               | [D3DBLEND]_\*             | Target/output alpha/color blending factors
//! | [BlendOp]::\*                             | [D3DBLENDOP]_\*           | Target/output alpha/color blending operation
//! | [Cmp]::\*                                 | [D3DCMP]_\*               | ZBuffer/depth comparison operation
//! | [ComposeRects]::\*                        | [D3DCOMPOSERECTS]_\*      |
//! | [CubeMapFace]::\*                         | [D3DCUBEMAP_FACE]_\*      | Which face of a cubemap to lock/update/acquire/???
//! | [Cull]::\*                                | [D3DCULL]_\*              | [None](crate::Cull::None), [CW](crate::Cull::CW), [CCW](crate::Cull::CCW)
//! | [DMT]::\*                                 | [D3DDMT]_\*               | Debug monitor tokens.
//! | [DeclMethod8]::\*                         | [D3DDECLMETHOD]_\*        | Tesselation method.
//! | [DeclType8]::\*                           | [D3DDECLTYPE]_\*          | [Float1](crate::DeclType8::Float1), [Float2](crate::DeclType8::Float2), ... - Defines a vertex declaration data type.
//! | [DeclUsage8]::\*                          | [D3DDECLUSAGE]_\*         | [Position](crate::DeclUsage8::Position), [TexCoord](crate::DeclUsage8::TexCoord), ... - Defines the intended use of vertex data.
//! | [Degree]::\*                              | [D3DDEGREE]_\*            | Curve [polynomial degree](https://en.wikipedia.org/wiki/Degree_of_a_polynomial)
//! | [DevType]::\*                             | [D3DDEVTYPE]_\*           | Specifies what kind of [Device] should be created
//! | [DisplayRotation]::\*                     | [D3DDISPLAYROTATION]_\*   | Orientation of the monitor/display
//! | [Fill]::\*                                | [D3DFILL]_\*              | [Point](crate::Fill::Point), [Wireframe](crate::Fill::Wireframe), or [Solid](crate::Fill::Solid) polygon rendering
//! | [Fog]::\*                                 | [D3DFOG]_\*               | [None](crate::Fog::None), [Exp](crate::Fog::Exp), [Exp2](crate::Fog::Exp2), or [Linear](crate::Fog::Linear) fog falloff
//! | [Fmt]::\*                                 | [D3DFMT]_\*               | Texture and vertex element formats
//! | [LightType]::\*                           | [D3DLIGHTTYPE]_\*         | Defines the type of a light ([Point](crate::LightType::Point), [Spot](crate::LightType::Spot), or [Directional](crate::LightType::Directional))
//! | [MCS]::\*                                 | [D3DMCS]_\*               | Lighting material source
//! | [MultiSample]::\*                         | [D3DMULTISAMPLE]_\*       | Defines the levels of full-scene multisampling to apply
//! | [PatchEdge]::\*                           | [D3DPATCHEDGE]_\*         | [Discrete](crate::PatchEdge::Discrete) or [Continuous](crate::PatchEdge::Continuous) tesselation.
//! | [Pool]::\*                                | [D3DPOOL]_\*              | Specifies what memory pool [Resource]s should be stored in
//! | [PT]::\*                                  | [D3DPT]_\*                | Defines the primitives supported by Direct3D.
//! | [QueryType]::\*                           | [D3DQUERYTYPE]_\*         | Identifies the query type.
//! | [RS]::\*                                  | [D3DRS]_\*                |
//! | [RType]::\*                               | [D3DRTYPE]_\*             | Specifies the type of a [Resource]/[Volume]
//! | [Samp]::\*                                | [D3DSAMP]_\*              |
//! | [STT]::\*                                 | [D3DSTT]_\*               |
//! | [ScanlineOrdering]::\*                    | [D3DSCANLINEORDERING]_\*  |
//! | [SGR]::\*                                 | [D3DSGR]_\*               | Indicates whether gamma correction should be applied.
//! | [Shade]::\*                               | [D3DSHADE]_\*             |
//! | [SBT]::\*                                 | [D3DSBT]_\*               | Predefined sets of pipeline state used by state blocks
//! | [StencilOp]::\*                           | [D3DSTENCILOP]_\*         |
//! | [StreamSource]::\*                        | [D3DSTREAMSOURCE]_\*      |
//! | [SwapEffect]::\*                          | [D3DSWAPEFFECT]_\*        | Defines [IDirect3DDevice9Ext::present] swap effects.
//! | [TAddress]::\*                            | [D3DTADDRESS]_\*          |
//! | [TexF]::\*                                | [D3DTEXF]_\*              |
//! | [TOP]::\*                                 | [D3DTOP]_\*               |
//! | [TSS]::\*                                 | [D3DTSS]_\*               |
//! | [TS]::\*                                  | [D3DTS]_\*                |
//! | [ZB]::\*                                  | [D3DZB]_\*                |
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [D3D](crate::D3D)\[[ERR](crate::D3DERR)\] | [D3DERR]                  | Windows HRESULTs optimized for displaying Direct3D errors
//! | [BackBufferType]                          | [D3DBACKBUFFER_TYPE]      | [Mono](crate::BackBufferType::Mono), [Left](crate::BackBufferType::Left), or [Right](crate::BackBufferType::Right) for [stereo](https://en.wikipedia.org/wiki/Stereoscopy) rendering
//! | [BasisType]                               | [D3DBASISTYPE]            | Tesselation basis
//! | [Blend]                                   | [D3DBLEND]                | Target/output alpha/color blending factors
//! | [BlendOp]                                 | [D3DBLENDOP]              | Target/output alpha/color blending operation
//! | [CmpFunc]                                 | [D3DCMPFUNC]              | ZBuffer/depth comparison operation
//! | [ComposeRectsOp]                          | [D3DCOMPOSERECTSOP]       |
//! | [CubeMapFace]                             | [D3DCUBEMAP_FACES]        | Which face of a cubemap to lock/update/acquire/???
//! | <span class="inaccurate">[CubeMapFaces]   | [D3DCUBEMAP_FACES]        | This isn't a mask, [CubeMapFace] reads way better in all contexts!
//! | [Cull]                                    | [D3DCULL]                 | [None](crate::Cull::None), [CW](crate::Cull::CW)], [CCW](crate::Cull::CCW)
//! | [DebugMonitorTokens]                      | [D3DDEBUGMONITORTOKENS]   | Debug monitor tokens.
//! | [DeclMethod8]                             | [D3DDECLMETHOD]           | Tesselation method.
//! | [DeclType8]                               | [D3DDECLTYPE]             | [Float1](crate::DeclType8::Float1), [Float2](crate::DeclType8::Float2), ... - Defines a vertex declaration data type.
//! | [DeclUsage8]                              | [D3DDECLUSAGE]            | [Position](crate::DeclUsage8::Position), [TexCoord](crate::DeclUsage8::TexCoord), ... - Defines the intended use of vertex data.
//! | [DegreeType]                              | [D3DDEGREETYPE]           | Curve [polynomial degree](https://en.wikipedia.org/wiki/Degree_of_a_polynomial)
//! | [DevType]                                 | [D3DDEVTYPE]              | Specifies what kind of [Device] should be created
//! | [DisplayRotation]                         | [D3DDISPLAYROTATION]      | Orientation of the monitor/display
//! | [FillMode]                                | [D3DFILLMODE]             | [Point](crate::Fill::Point), [Wireframe](crate::Fill::Wireframe), or [Solid](crate::Fill::Solid) polygon rendering
//! | [FogMode]                                 | [D3DFOGMODE]              | [None](crate::Fog::None), [Exp](crate::Fog::Exp), [Exp2](crate::Fog::Exp2), or [Linear](crate::Fog::Linear) fog falloff
//! | [Format]                                  | [D3DFORMAT]               | Texture and vertex element formats
//! | [LightType]                               | [D3DLIGHTTYPE]            | Defines the type of a light ([Point](crate::LightType::Point), [Spot](crate::LightType::Spot), or [Directional](crate::LightType::Directional))
//! | [MaterialColorSource]                     | [D3DMATERIALCOLORSOURCE]  | Lighting material source
//! | [MultiSampleType]                         | [D3DMULTISAMPLE_TYPE]     | Defines the levels of full-scene multisampling to apply
//! | [PatchEdgeStyle]                          | [D3DPATCHEDGESTYLE]       | [Discrete](crate::PatchEdge::Discrete) or [Continuous](crate::PatchEdge::Continuous) tesselation.
//! | [Pool]                                    | [D3DPOOL]                 | Specifies what memory pool [Resource]s should be stored in
//! | [PrimitiveType]                           | [D3DPRIMITIVETYPE]        | Defines the primitives supported by Direct3D.
//! | [QueryType]                               | [D3DQUERYTYPE]            | Identifies the query type.
//! | [RenderStateType]                         | [D3DRENDERSTATETYPE]      |
//! | [ResourceType]                            | [D3DRESOURCETYPE]         | Specifies the type of a [Resource]/[Volume]
//! | [SamplerStateType]                        | [D3DSAMPLERSTATETYPE]     |
//! | [SamplerTextureType]                      | [D3DSAMPLER_TEXTURE_TYPE] |
//! | [ScanlineOrdering]                        | [D3DSCANLINEORDERING]     |
//! | [SGR]                                     | [D3DSGR]                  | Indicates whether gamma correction should be applied.
//! | [ShadeMode]                               | [D3DSHADEMODE]            |
//! | [StateBlockType]                          | [D3DSTATEBLOCKTYPE]       | Predefined sets of pipeline state used by state blocks
//! | [StencilOp]                               | [D3DSTENCILOP]            |
//! | [StreamSource]                            | [D3DSTREAMSOURCE]         |
//! | [SwapEffect]                              | [D3DSWAPEFFECT]           | Defines [IDirect3DDevice9Ext::present] swap effects.
//! | [TextureAddress]                          | [D3DTEXTUREADDRESS]       |
//! | [TextureFilterType]                       | [D3DTEXTUREFILTERTYPE]    |
//! | [TextureOp]                               | [D3DTEXTUREOP]            |
//! | [TextureStageStateType]                   | [D3DTEXTURESTAGESTATETYPE]|
//! | [TransformStateType]                      | [D3DTRANSFORMSTATETYPE]   |
//! | [ZBufferType]                             | [D3DZBUFFERTYPE]          |
//!
//! ### Flags
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [Create]                                  | [D3DCREATE_*]             | A combination of one or more flags that controls [Direct3D::create_device] behavior.
//! | [FVF]                                     | [D3DFVF_*]                | Describes the contents of vertices interleaved in a single data stream.
//! | [GetData]                                 | [D3DGETDATA_*]            | Controls how [IDirect3DQuery9Ext::get_data_inplace] behaves
//! | [Issue]                                   | [D3DISSUE_*]              | Controls how [IDirect3DQuery9Ext::issue] behaves
//! | [Lock]                                    | [D3DLOCK_*]               | A combination of zero or more locking options that describe the type of lock to perform.
//! | [Usage]                                   | [D3DUSAGE_*]              | Usage options that identify how resources are to be used.
//!
//! [D3DCREATE_*]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcreate
//! [D3DFVF_*]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfvf
//! [D3DGETDATA_*]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dquery9-getdata
//! [D3DISSUE_*]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dquery9-issue
//! [D3DLOCK_*]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock
//! [D3DUSAGE_*]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dusage
//!
//! ### Traits
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | unsafe [Raw]                              |                           | Conversion trait for converting between [thin3d9](crate) ⇄ [winapi]
//! | unsafe [SharedHandleParam]                | \*mut HANDLE              | Placeholder for [Sharing Resources](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9lh#sharing-resources) `*mut HANDLE`s
//!
//! ### Structures
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [AdapterIdentifier]                       | [D3DADAPTER_IDENTIFIER9]  | Adapter metadata (driver, description, driver version, vendor/device ids, ...)
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
//! [RECT]:                     https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect
//!
//! ### Values
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [bool32]                                  | [BOOL]                    | 32-bit boolean type that's ABI-compatible with Win32's [BOOL]
//! | [Color]                                   | [D3DCOLOR]                | 0xAA<span style="color: red">RR</span><span style="color: green">GG</span><span style="color: blue">BB</span> style 32-bit color
//! | [Luid]                                    | [LUID]                    | A 64-bit locally unique identifier
//! | [SdkVersion]                              | DWORD                     | Specify what Direct3D SDK to use ([Direct3D](crate::Direct3D)\[[Ex](crate::Direct3DEx)\]::[create](crate::Direct3D::create)'s only parameter)
//!
//! [BOOL]:                     https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL
//! [D3DCOLOR]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolor
//! [LUID]:                     https://docs.microsoft.com/en-us/previous-versions/windows/hardware/drivers/ff549708(v=vs.85)
//!
//! ### Wrappers
//!
//! | Wrapper type                              | Underlying type           | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [SafeDevice]                              | [Device]                  | Device pointer + enough metadata to make more API calls safe
//!
//! ### Features
//!
//! | feature               | description           |
//! | --------------------- | --------------------- |
//! |                       | Default: **enabled** by default|
//! | `9ex`                 | `!defined(D3D_DISABLE_9EX)` - Enables [Direct3DEx], [DeviceEx], [SwapChainEx], etc.
