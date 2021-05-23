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
//! ### Why not `winapi` directly?
//!
//! *   This crate aims to make fns safe/sound/slightly rusty when possible
//! *   Attempts to verify API soundness through mass unit tests, even if they mostly test Direct3D9's behavior.
//! *   Doc comments for one-stop intellisense, safety documentation, etc.
//!
//! ### Why not `<other graphics crate>`?
//!
//! *   Most other graphics crates focus on **hiding** the underlying graphics API as much as possible, improving application portability.
//! *   This crate **surfaces** the underlying graphics API as much as possible, improving the potential for interoperability with other graphics code / use in middleware applications.
//!
//! ### Is this crate sound?
//!
//! Probably not.  I'm exposing a huge legacy C++ API to Rust.  Mistakes will happen.
//!
//! That said, soundness *is* a **very high priority** goal.  `thin3d9` will add things like extra bounds checks, parameter
//! validation, extra init, etc. if need be in order to ensure soundness in safe fns whenever possible.  When it's not
//! possible to validate unsoundness away, the fns in question should be marked `unsafe`.  This crate strives to be sounder
//! than whatever manual FFI you'd write yourself would be, and that's a **high** bar.
//!
//! But there are some practical limits to this.  If a background driver thread invokes UB if it fails to allocate memory,
//! without any direct correlation to specific API misuse like a large integer overflowing, that's a bug I can't sanely
//! mitigate via safe fns.  I mean, theoretically I could write a pure-software clone of the entire DirectX runtime... but no.
//!
//! Additionally, while I'm seeking to validate my APIs via testing, older implementations of the APIs in question may have
//! more bugs / unchecked parameters / ??? that I'll fail to mitigate due to being unable to trigger them myself.  Ditto for
//! the proxy objects returned by various graphics debuggers.  While I'm happy to investigate, accept pull requests, expand
//! test coverage, etc. it's worth assuming this crate is unsound on older versions unless you've tested yourself.
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
//! [IUnknown]:                     https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
//! [IDirect3D9]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9
//! [IDirect3D9Ex]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9ex
//! [IDirect3DDevice9]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9
//! [IDirect3DDevice9Ex]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex
//! [IDirect3DSwapChain9]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9
//! [IDirect3DSwapChain9Ex]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex
//! [IDirect3DResource9]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9
//! [IDirect3DSurface9]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dsurface9
//! [IDirect3DBaseTexture9]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9
//! [IDirect3DCubeTexture9]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9
//! [IDirect3DTexture9]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9
//! [IDirect3DVolumeTexture9]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9
//! [IDirect3DIndexBuffer9]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9
//! [IDirect3DVertexBuffer9]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9
//! [IDirect3DVolume9]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9
//! [IDirect3DPixelShader9]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dpixelshader9
//! [IDirect3DVertexShader9]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexshader9
//! [IDirect3DQuery9]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9
//! [IDirect3DStateBlock9]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9
//! [IDirect3DVertexDeclaration9]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9
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
//! [D3DERR]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr
//! [D3DBACKBUFFER_TYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type
//! [D3DBASIS]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype
//! [D3DBASISTYPE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype
//! [D3DBLEND]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblend
//! [D3DBLENDOP]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop
//! [D3DCMP]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc
//! [D3DCMPFUNC]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc
//! [D3DCOMPOSERECTS]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop
//! [D3DCOMPOSERECTSOP]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop
//! [D3DCUBEMAP_FACE]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces
//! [D3DCUBEMAP_FACES]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcubemap-faces
//! [D3DCULL]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcull
//! [D3DDMT]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens
//! [D3DDEBUGMONITORTOKENS]:    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens
//! [D3DDECLMETHOD]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod
//! [D3DDECLTYPE]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype
//! [D3DDECLUSAGE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage
//! [D3DDEGREE]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype
//! [D3DDEGREETYPE]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype
//! [D3DDEVTYPE]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype
//! [D3DDISPLAYROTATION]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation
//! [D3DFILL]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode
//! [D3DFILLMODE]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode
//! [D3DFOG]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode
//! [D3DFOGMODE]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode
//! [D3DFMT]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
//! [D3DFORMAT]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
//! [D3DLIGHTTYPE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype
//! [D3DMCS]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource
//! [D3DMATERIALCOLORSOURCE]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource
//! [D3DMULTISAMPLE]:           https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
//! [D3DMULTISAMPLE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
//! [D3DPATCHEDGE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle
//! [D3DPATCHEDGESTYLE]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle
//! [D3DPOOL]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
//! [D3DPT]:                    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype
//! [D3DPRIMITIVETYPE]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype
//! [D3DQUERYTYPE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dquerytype
//! [D3DRS]:                    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype
//! [D3DRENDERSTATETYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drenderstatetype
//! [D3DRTYPE]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
//! [D3DRESOURCETYPE]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
//! [D3DSAMP]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype
//! [D3DSAMPLERSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype
//! [D3DSTT]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type
//! [D3DSAMPLER_TEXTURE_TYPE]:  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type
//! [D3DSCANLINEORDERING]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dscanlineordering
//! [D3DSGR]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsgr
//! [D3DSHADE]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode
//! [D3DSHADEMODE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode
//! [D3DSBT]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype
//! [D3DSTATEBLOCKTYPE]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype
//! [D3DSTENCILOP]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop
//! [D3DSTREAMSOURCE]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstreamsource
//! [D3DSWAPEFFECT]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dswapeffect
//! [D3DTADDRESS]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress
//! [D3DTEXTUREADDRESS]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureaddress
//! [D3DTEXF]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype
//! [D3DTEXTUREFILTERTYPE]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype
//! [D3DTOP]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop
//! [D3DTEXTUREOP]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtextureop
//! [D3DTSS]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype
//! [D3DTEXTURESTAGESTATETYPE]: https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturestagestatetype
//! [D3DTS]:                    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype
//! [D3DTRANSFORMSTATETYPE]:    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtransformstatetype
//! [D3DZB]:                    https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype
//! [D3DZBUFFERTYPE]:           https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype
//!
//! ### Flags
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [Create]                                  | [D3DCREATE_*]             | A combination of one or more flags that controls [Direct3D::create_device] behavior.
//! | [FVF]                                     | [D3DFVF_*]                | Describes the contents of vertices interleaved in a single data stream.
//! | [GetData]                                 | [D3DGETDATA_*]            | Controls how [Query::get_data_inplace] behaves
//! | [Issue]                                   | [D3DISSUE_*]              | Controls how [Query::issue] behaves
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
//! | [IndexBufferDesc]                         | [D3DINDEXBUFFER_DESC]     | Describes an [IndexBuffer]
//! | [Light]                                   | [D3DLIGHT9]               | Describes lighting information
//! | [Material]                                | [D3DMATERIAL9]            | Describes a material that responds to lighting
//! | [Rect]                                    | [D3DRECT] / [RECT]        | `[0i32 .. 1i32, 2i32 .. 3i32]` signed x/y range structure
//! | [Vector]                                  | [D3DVECTOR]               | A `{ x, y, z }` 3-dimensional floating point position
//! | [VertexBufferDesc]                        | [D3DVERTEXBUFFER_DESC]    | Describes an [VertexBuffer]
//! | [VertexElement]                           | [D3DVERTEXELEMENT9]       | An element of a [VertexDeclaration]
//! | [Viewport]                                | [D3DVIEWPORT9]            | A `{ x, y, width, height, min_z, max_z }` area to render into
//!
//! [D3DADAPTER_IDENTIFIER9]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dadapter-identifier9
//! [D3DCAPS9]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9
//! [D3DCLIPSTATUS9]:           https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
//! [D3DCOLORVALUE]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolorvalue
//! [D3DDISPLAYMODE]:           https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymode
//! [D3DINDEXBUFFER_DESC]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dindexbuffer-desc
//! [D3DLIGHT9]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlight9
//! [D3DMATERIAL9]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterial9
//! [D3DRECT]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drect
//! [D3DVECTOR]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvector
//! [D3DVERTEXBUFFER_DESC]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvertexbuffer-desc
//! [D3DVERTEXELEMENT9]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvertexelement9
//! [D3DVIEWPORT9]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dviewport9
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
//! ### Re-Exports
//!
//! | crate     | version   |
//! | --------- | --------- |
//! | [mcom]    | 0.1.1+
//! | [winapi]  | 0.3.9+
//!
//! ### Features
//!
//! | feature               | description           |
//! | --------------------- | --------------------- |
//! |                       | Default: **enabled** by default|
//! | `9ex`                 | `!defined(D3D_DISABLE_9EX)` - Enables [Direct3DEx], [DeviceEx], [SwapChainEx], etc.
//! | `impl-poor-defaults`  | Implement [Default] for a bunch of types... even when there might not be a super sane default.
//! |                       | Default: disabled     |
//! | `impl-from-unchecked` | Implement [From]\<[D3DFORMAT]\> for [Format] and similar traits.<br>While these should generally be sound, and may ease porting (e.g. they'll allow using `D3DFMT_UNKNOWN` instead of [Format::UNKNOWN]), they'll also ease authoring bugs (e.g. they'll allow using `9001` for an [Into]\<[Format]\> parameter as well.)

#![allow(broken_intra_doc_links)] // TODO: temporary
#![deny(unreachable_patterns)]

#[cfg(test)] fn testfast() -> bool { std::env::var_os("TESTFAST").is_some() }

pub use abibool::bool32;
use mcom::Rc;

#[doc(no_inline)]                                   pub extern crate mcom;
#[doc(no_inline)]                                   pub extern crate winapi;

// These define types which later mods may extend
#[path="wrappers/_wrappers.rs"]     mod wrappers;   pub use wrappers::*;
#[path="interfaces/_interfaces.rs"] mod interfaces; pub use interfaces::*;

#[path="enums/_enums.rs"]           mod enums;      pub use enums::*;
#[path="flags/_flags.rs"]           mod flags;      pub use flags::*;
#[path="structs/_structs.rs"]       mod structs;    pub use structs::*;
#[path="traits/_traits.rs"]         mod traits;     pub use traits::*;
#[path="values/_values.rs"]         mod values;     pub use values::*;

#[path="extra/_extra.rs"] #[cfg(        feature = "extra"        )] pub mod extra;
#[path="extra/_extra.rs"] #[cfg(all(not(feature = "extra"), test))] mod extra;



// Several of these should probably be wrapped...?
#[doc(no_inline)] pub use winapi::shared::d3d9caps::{
    D3DCAPS9,
};
#[doc(no_inline)] pub use winapi::shared::d3d9types::{
    D3DDISPLAYMODE,
    D3DDISPLAYMODEEX,
    D3DPRESENT_PARAMETERS,
};
#[doc(no_inline)] pub use winapi::shared::windef::{
    HWND,
};
