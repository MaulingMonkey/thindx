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
//! ### Why not [winapi] directly?
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
//! ### Interfaces
//!
//! These are all refcounting COM smart pointers convertable to/from [mcom::Rc].
//! <div class="interface-tree">
//!
//! [Unknown] - The root type from which all sane COM types derive
//! &nbsp;  ├─ [Direct3D]\[[Ex](crate::Direct3DEx)\] - Core factories for creating [Device]\[[Ex](crate::Direct3DEx)\]s and [SwapChain]\[[Ex](crate::SwapChainEx)\]s<span style="opacity: 25%">
//! &nbsp;  ├─ [Device]\[[Ex](crate::DeviceEx)\] - Create resources associated with an individual GPU
//! &nbsp;  ├─ [SwapChain]\[[Ex](crate::SwapChainEx)\] - Manages swapping buffers for individual "views" (monitors/windows)
//! &nbsp;  ├─ [Resource]
//! &nbsp;  │      ├─ [Surface] - 2D buffer of pixels
//! &nbsp;  │      ├─ [BaseTexture] - A GPU-friendly collection of pixels
//! &nbsp;  │      │      ├─ [Texture] - 2D texture
//! &nbsp;  │      │      ├─ [CubeTexture] - 6-sided 2D texture
//! &nbsp;  │      │      └─ [VolumeTexture] - Dense 3D texture</span>
//! &nbsp;  │      ├─ [IndexBuffer] - An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) indexes verticies in a [VertexBuffer] when rendering.
//! &nbsp;  │      └─ [VertexBuffer] - A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) typically contains points of a mesh to be rendered.
//! &nbsp;  ├─ <strike>[Resource]</strike> You'd expect these to be resources, but they aren't - they derive from [Unknown].<span style="opacity: 25%">
//! &nbsp;  │      ├─ [Volume] - 3D buffer of pixels</span>
//! &nbsp;  │      ├─ [PixelShader] - per-fragment GPU program
//! &nbsp;  │      └─ [VertexShader] - per-vertex GPU program
//! &nbsp;  ├─ [Query] - An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
//! &nbsp;  ├─ [StateBlock] - Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state) changes to [Device] state.
//! &nbsp;  └─ [VertexDeclaration] - Describes the layout of the contents of a [VertexBuffer]</div>
//!
//! ### Enums
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [D3D](crate::D3D)\[[ERR](crate::D3DERR)\] | [D3DERR]                  | Windows HRESULTs optimized for displaying Direct3D errors
//! | [BackBufferType]                          | [D3DBACKBUFFER_TYPE]      | [Mono](crate::BackBufferType::Mono), [Left](crate::BackBufferType::Left), or [Right](crate::BackBufferType::Right)
//! | [DeclMethod8]                             | [D3DDECLMETHOD]           | Operation performed by the tessellator (or any procedural geometry routine).
//! | [DeclType8]                               | [D3DDECLTYPE]             | [Float1](crate::DeclType8::Float1), [Float2](crate::DeclType8::Float2), ... - Defines a vertex declaration data type.
//! | [DeclUsage8]                              | [D3DDECLUSAGE]            | [Position](crate::DeclUsage8::Position), [TexCoord](crate::DeclUsage8::TexCoord), ... - Identifies the intended use of vertex data.
//! | [DevType]                                 | [D3DDEVTYPE]              | Specifies what kind of [Device] should be created
//! | [Format]                                  | [D3DFORMAT]               | Texture and vertex element formats
//! | [LightType]                               | [D3DLIGHTTYPE]            | Defines the type of a light ([Point](crate::LightType::Point), [Spot](crate::LightType::Spot), or [Directional](crate::LightType::Directional))
//! | [MultiSample]                             | [D3DMULTISAMPLE_TYPE]     | Defines the levels of full-scene multisampling to apply
//! | [Pool]                                    | [D3DPOOL]                 | Specifies what memory pool [Resource]s should be stored in
//! | [PrimitiveType]                           | [D3DPRIMITIVETYPE]        | Defines the primitives supported by Direct3D.
//! | [QueryType]                               | [D3DQUERYTYPE]            | Identifies the query type.
//! | [ResourceType]                            | [D3DRESOURCETYPE]         | Specifies the type of a [Resource]/[Volume]
//! | [SGR]                                     | [D3DSGR]                  | Indicates whether gamma correction should be applied.
//! | [StateBlockType]                          | [D3DSTATEBLOCKTYPE]       | Predefined sets of pipeline state used by state blocks
//! | [SwapEffect]                              | [D3DSWAPEFFECT]           | Defines [Device::present] swap effects.
//!
//! [D3DERR]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr
//! [D3DBACKBUFFER_TYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type
//! [D3DDECLMETHOD]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod
//! [D3DDECLTYPE]:              https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype
//! [D3DDECLUSAGE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage
//! [D3DDEVTYPE]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype
//! [D3DFORMAT]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
//! [D3DLIGHTTYPE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype
//! [D3DMULTISAMPLE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
//! [D3DPOOL]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
//! [D3DPRIMITIVETYPE]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dprimitivetype
//! [D3DQUERYTYPE]:             https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dquerytype
//! [D3DRESOURCETYPE]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
//! [D3DSGR]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsgr
//! [D3DSTATEBLOCKTYPE]:        https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstateblocktype
//! [D3DSWAPEFFECT]:            https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dswapeffect
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

#[macro_use] mod macros;
#[allow(unused_imports)] use macros::*;

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
