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
//! &nbsp;  │      │      └─ [VolumeTexture] - Dense 3D texture
//! &nbsp;  │      ├─ [IndexBuffer]
//! &nbsp;  │      └─ [VertexBuffer]
//! &nbsp;  ├─ <strike>[Resource]</strike> You'd expect these to be resources, but they aren't - they derive from [Unknown].
//! &nbsp;  │      ├─ [Volume] - 3D buffer of pixels
//! &nbsp;  │      ├─ [PixelShader] - per-fragment GPU program
//! &nbsp;  │      └─ [VertexShader] - per-vertex GPU program
//! &nbsp;  ├─ [Query]
//! &nbsp;  ├─ [StateBlock]
//! &nbsp;  └─ [VertexDeclaration]</span></div>
//!
//! ### Structures
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [AdapterIdentifier]                       | [D3DADAPTER_IDENTIFIER9]  | Adapter metadata (driver, description, driver version, vendor/device ids, ...)
//! | [Caps]                                    | [D3DCAPS9]                | Adapter/device capabilities and limitations
//! | [Color]                                   | [D3DCOLOR]                | 0xAA<span style="color: red">RR</span><span style="color: green">GG</span><span style="color: blue">BB</span> style 32-bit color
//! | [Rect]                                    | [D3DRECT] / [RECT]        | `[0i32 .. 1i32, 2i32 .. 3i32]` signed x/y range structure
//!
//! ### Enum-like Values
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | [D3D](crate::D3D)\[[ERR](crate::D3DERR)\] | [D3DERR]                  | Windows HRESULTs optimized for displaying Direct3D errors
//! | [DevType]                                 | [D3DDEVTYPE]              | Specifies what kind of [Device] should be created
//! | [Format]                                  | [D3DFORMAT]               | Texture and vertex element formats
//! | [MultiSample]                             | [D3DMULTISAMPLE_TYPE]     | Defines the levels of full-scene multisampling to apply
//! | [Pool]                                    | [D3DPOOL]                 | Specifies what memory pool [Resource]s should be stored in
//! | [ResourceType]                            | [D3DRESOURCETYPE]         | Specifies the type of a [Resource]/[Volume]
//! | [SdkVersion]                              | DWORD                     | Specify what Direct3D SDK to use ([Direct3D](crate::Direct3D)\[[Ex](crate::Direct3DEx)\]::[create](crate::Direct3D::create)'s only parameter)
//!
//! ### Traits
//!
//! | `thin3d9` type                            | docs.microsoft.com        | description   |
//! | ----------------------------------------- | ------------------------- | ------------- |
//! | unsafe [Raw]                              |                           | Conversion trait for converting between [thin3d9](crate) ⇄ [winapi]
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
//!
//!
//! [Direct3D9]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/
//! [D3DADAPTER_IDENTIFIER9]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dadapter-identifier9
//! [D3DCAPS9]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9
//! [D3DCOLOR]:                 https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcolor
//! [D3DDEVTYPE]:               https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddevtype
//! [D3DERR]:                   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3derr
//! [D3DFORMAT]:                https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dformat
//! [D3DMULTISAMPLE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
//! [D3DPOOL]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpool
//! [D3DRECT]:                  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drect
//! [D3DRESOURCETYPE]:          https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
//! [RECT]:                     https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect

use mcom::Rc;

#[doc(no_inline)]                                   pub extern crate mcom;
#[doc(no_inline)]                                   pub extern crate winapi;
#[path="interfaces/_interfaces.rs"] mod interfaces; pub use interfaces::*;
#[path="values/_values.rs"]         mod values;     pub use values::*;
#[path="traits/_traits.rs"]         pub mod traits; #[doc(no_inline)] pub use traits::*;




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
